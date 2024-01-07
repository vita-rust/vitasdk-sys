use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
    path::Path,
    rc::Rc,
};

use quote::ToTokens;
use syn::{
    token, visit_mut::VisitMut, AttrStyle, Attribute, ForeignItem, Ident, Item, ItemForeignMod,
    MacroDelimiter, Meta, MetaList,
};

use crate::vita_headers_db::{missing_features_filter, stub_lib_name, VitaDb};

type FeatureSet = BTreeSet<Rc<str>>;

const DEFINED_ELSEWHERE_FUNCTIONS: [(&str, &str); 3] = [
    // Defined in vitasdk newlib libc implementation
    ("vitasdk_get_tls_data", "vitasdk-utils"),
    ("vitasdk_get_pthread_data", "vitasdk-utils"),
    ("vitasdk_delete_thread_reent", "vitasdk-utils"),
];
const DEFINED_ELSEWHERE_VARIABLES: [(&str, &str); 0] = [];

pub struct Link {
    /// link.function[function_name] = stub_library_name
    function: BTreeMap<String, BTreeSet<Rc<str>>>,
    /// link.variable[variable_name] = stub_library_name
    variable: BTreeMap<String, BTreeSet<Rc<str>>>,
    stub_libs: BTreeSet<Rc<str>>,
}

impl Link {
    pub fn load(db: &Path, vitasdk_sys_manifest: &Path) -> Self {
        let mut link = Link {
            function: DEFINED_ELSEWHERE_FUNCTIONS
                .into_iter()
                .map(|(func, feat)| (func.into(), BTreeSet::from([Rc::from(feat.to_owned())])))
                .collect(),
            variable: DEFINED_ELSEWHERE_VARIABLES
                .into_iter()
                .map(|(var, feat)| (var.into(), BTreeSet::from([Rc::from(feat.to_owned())])))
                .collect(),
            stub_libs: BTreeSet::new(),
        };

        let mut db = VitaDb::load(db);

        let mut predicate = missing_features_filter(vitasdk_sys_manifest);

        for imports in db.imports_by_firmware.values() {
            for (mod_name, mod_data) in &imports.modules {
                for (lib_name, lib) in &mod_data.libraries {
                    let stub_lib_name = stub_lib_name(
                        mod_name,
                        lib_name,
                        lib.stub_name.as_deref(),
                        lib.kernel,
                        &imports.firmware,
                    )
                    .to_string();
                    let stub_lib_name = link
                        .stub_libs
                        .get(stub_lib_name.as_str())
                        .cloned()
                        .unwrap_or_else(|| Rc::from(stub_lib_name));

                    for function_name in lib.function_nids.keys() {
                        link.function
                            .entry(function_name.clone())
                            .or_default()
                            .insert(Rc::clone(&stub_lib_name));
                    }

                    for variable_name in lib.variable_nids.keys() {
                        link.variable
                            .entry(variable_name.clone())
                            .or_default()
                            .insert(Rc::clone(&stub_lib_name));
                    }

                    link.stub_libs.insert(stub_lib_name);
                }
            }
        }

        let conflicting_db = db.split_conflicting();
        conflicting_db.stub_lib_names().for_each(|lib_stub| {
            link.stub_libs.remove(lib_stub.as_str());
        });

        if db.stub_lib_names().any(|s| predicate(&s)) {
            panic!("Missing features in vitasdk-sys `Cargo.toml`. Please run `cargo run -p vitasdk-sys-build-util -- stub-libs --as-features --all-stubs-feature` and replace stub lib features in vitasdk-sys Cargo.toml with outputed ones.")
        }

        link
    }
}

impl Link {
    pub fn visit(&self, i: &mut syn::File) {
        let mut items_by_features: BTreeMap<FeatureSet, Vec<ForeignItem>> = self
            .stub_libs
            .iter()
            .map(|stub_lib_name| (BTreeSet::from([stub_lib_name.clone()]), Vec::new()))
            .collect();

        // Moves mod items to to the "items_by_features" map
        i.items = mem::take(&mut i.items)
            .into_iter()
            .filter_map(|item| {
                // We only care about foreign mods
                let Item::ForeignMod(foreign_mod) = item else {
                    return Some(item);
                };

                for foreign_item in foreign_mod.items {
                    let features: FeatureSet = match &foreign_item {
                        ForeignItem::Fn(fn_item) => {
                            let symbol = fn_item.sig.ident.to_string();

                            self.function
                                .get(&symbol)
                                .expect("Undefined foreign fn `{symbol}`")
                                .to_owned()
                        }
                        ForeignItem::Static(static_item) => {
                            let symbol = static_item.ident.to_string();

                            self.variable
                                .get(&symbol)
                                .expect("Undefined foreign static `{symbol}`")
                                .to_owned()
                        }
                        _ => panic!("unexpected foreign item: {:?}", foreign_item),
                    };

                    items_by_features
                        .entry(features)
                        .or_default()
                        .push(foreign_item);
                }

                // We'll remove foreign mods for now to re-add them later grouped by feature
                None
            })
            .collect();

        i.items
            .extend(items_by_features.into_iter().map(|(features, items)| {
                let mut foreign_mod: ItemForeignMod = if features.len() == 1 {
                    let feature = features.first().unwrap();
                    syn::parse_quote! {
                        #[link(name = #feature, kind = "static")]
                        extern "C" {}
                    }
                } else {
                    syn::parse_quote! {
                        extern "C" {}
                    }
                };

                // Adds feature attributes to mod
                if features.len() == 1 {
                    let feature = features.first();
                    foreign_mod
                        .attrs
                        .push(syn::parse_quote!(#[cfg(feature = #feature)]));
                } else {
                    let item_idents: Vec<_> = items.iter().map(foreign_item_ident).collect();
                    log::warn!(
                        "Items `{item_idents:?}` are defined in multiple locations: {features:?}"
                    );
                    let feature_gates = || {
                        features.iter().map(|feature| -> syn::MetaNameValue {
                            syn::parse_quote!(feature = #feature)
                        })
                    };
                    foreign_mod.attrs.push({
                        let feature_gates = feature_gates();
                        syn::parse_quote!(#[cfg(any(#(#feature_gates),*))])
                    });
                }

                // Adds items to mod
                foreign_mod.items.extend(items);

                Item::ForeignMod(foreign_mod)
            }));
    }
}

pub struct Sort;

impl VisitMut for Sort {
    fn visit_file_mut(&mut self, i: &mut syn::File) {
        syn::visit_mut::visit_file_mut(self, i);

        // Sorts items on alphabetical order based on normalized identifier.
        // Bindgen items will be moved to the start.
        // Note: for simplicity, we rely on sort_by_cached_key being stable and  some things
        // already being sorted, such as impl blocks already being after their definition.
        i.items.sort_by_cached_key(|item| {
            let (precedence, ident) = match item {
                Item::ExternCrate(i) => (0, i.ident.to_string()),
                Item::Use(_i) => (1, String::new()),
                Item::Mod(i) => (2, i.ident.to_string()),
                Item::Macro(i) => (
                    3,
                    i.ident
                        .as_ref()
                        .map(|i| i.to_string())
                        .unwrap_or_else(String::new),
                ),
                Item::Struct(i) => (4, i.ident.to_string()),
                Item::Enum(i) => (4, i.ident.to_string()),
                Item::Union(i) => (4, i.ident.to_string()),
                Item::Impl(i) => {
                    let ident = i.self_ty.clone().into_token_stream().to_string();
                    (4, ident)
                }
                Item::Const(i) => (5, i.ident.to_string()),
                Item::Static(i) => (6, i.ident.to_string()),
                Item::Trait(i) => (7, i.ident.to_string()),
                Item::TraitAlias(i) => (8, i.ident.to_string()),
                Item::Fn(i) => (8, i.sig.ident.to_string()),
                Item::ForeignMod(i) => {
                    let ident = foreign_mod_ident(i);

                    (9, ident)
                }
                Item::Type(i) => (10, i.ident.to_string()),
                i => {
                    log::warn!("Unexpected item: {i:?}");
                    (11, String::new())
                }
            };
            consider_bindgen((precedence, normalize_str(&ident)))
        });
    }

    fn visit_item_foreign_mod_mut(&mut self, i: &mut ItemForeignMod) {
        i.items.sort_by_cached_key(foreign_item_ident);

        syn::visit_mut::visit_item_foreign_mod_mut(self, i)
    }
}

fn foreign_item_ident(foreign_item: &ForeignItem) -> String {
    match &foreign_item {
        ForeignItem::Fn(i) => i.sig.ident.to_string(),
        ForeignItem::Static(i) => i.ident.to_string(),
        ForeignItem::Type(i) => i.ident.to_string(),
        i => {
            log::warn!("Unexpected item in foreign mod: {i:?}");
            String::new()
        }
    }
}

/// Gets foreign mod identifier as the feature cfgs concatenated by `+`
fn foreign_mod_ident(foreign_mod: &ItemForeignMod) -> String {
    let ident = foreign_mod.attrs.iter().find_map(|attribute| {
        if attribute.style != AttrStyle::Outer {
            return None;
        }

        let Attribute {
            pound_token: token::Pound { .. },
            style: AttrStyle::Outer,
            bracket_token: token::Bracket { .. },
            meta:
                Meta::List(MetaList {
                    path,
                    delimiter: MacroDelimiter::Paren(token::Paren { .. }),
                    tokens,
                }),
        } = attribute
        else {
            return None;
        };

        // We're only insterested on `cfg`.
        if path.segments.len() != 1 || &path.segments[0].ident != "cfg" {
            return None;
        }

        let mut token_iter = tokens.clone().into_iter();
        let first_ident: Ident = syn::parse2(token_iter.next()?.into()).ok()?;
        if first_ident == "feature" {
            let punct: proc_macro2::Punct = syn::parse2(token_iter.next()?.into()).ok()?;
            let literal: proc_macro2::Literal = syn::parse2(token_iter.next()?.into()).ok()?;
            if token_iter.next().is_some() || punct.as_char() != '=' {
                return None;
            }
            Some(
                literal
                    .to_string()
                    .strip_prefix('"')?
                    .strip_suffix('"')?
                    .to_owned(),
            )
        } else if first_ident == "any" {
            let group: proc_macro2::Group = syn::parse2(token_iter.next()?.into()).ok()?;
            if token_iter.next().is_some() {
                return None;
            }

            enum Step {
                Ident,
                Equals,
                Literal,
                Separator,
            }

            let (features, _) = group.stream().into_iter().try_fold(
                (Vec::new(), Step::Ident),
                |(mut acc, step), token| match step {
                    Step::Ident => {
                        if syn::parse2::<Ident>(token.into()).ok()? != "feature" {
                            return None;
                        }
                        Some((acc, Step::Equals))
                    }
                    Step::Equals => {
                        if syn::parse2::<proc_macro2::Punct>(token.into())
                            .ok()?
                            .as_char()
                            != '='
                        {
                            return None;
                        }
                        Some((acc, Step::Literal))
                    }
                    Step::Literal => {
                        let literal: proc_macro2::Literal = syn::parse2(token.into()).ok()?;
                        acc.push(
                            literal
                                .to_string()
                                .strip_prefix('"')?
                                .strip_suffix('"')?
                                .to_owned(),
                        );
                        Some((acc, Step::Separator))
                    }
                    Step::Separator => {
                        if syn::parse2::<proc_macro2::Punct>(token.into())
                            .ok()?
                            .as_char()
                            != ','
                        {
                            return None;
                        }
                        Some((acc, Step::Ident))
                    }
                },
            )?;

            features.into_iter().reduce(|acc, f| acc + "+" + &f)
        } else {
            None
        }
    });

    ident.unwrap_or_else(String::new)
}

fn normalize_str(input: &str) -> String {
    input.to_lowercase().replace('_', "")
}

fn consider_bindgen(keys: (i32, String)) -> (i32, String) {
    let (precedence, ident) = keys;
    let new_precedence = if ident.starts_with("bindgen") {
        // Move bindgen items to the start of the file
        precedence - 16
    } else {
        precedence
    };
    (new_precedence, ident)
}
