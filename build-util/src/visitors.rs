use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
    path::Path,
    rc::Rc,
};

use crate::vita_headers_db::{missing_features_filter, stub_lib_name, VitaDb};
use syn::{visit_mut::VisitMut, ForeignItem, Item, ItemForeignMod, Type};

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
        let mut items_by_features: BTreeMap<FeatureSet, Vec<ForeignItem>> = BTreeMap::new();

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
                let mut foreign_mod: ItemForeignMod = syn::parse_quote! {
                    extern "C" {}
                };

                // Adds feature attributes to mod
                if features.len() == 1 {
                    let feature = features.first();
                    foreign_mod.attrs.extend([
                        syn::parse_quote!(#[cfg(feature = #feature)]),
                        syn::parse_quote!(#[cfg_attr(docsrs, doc(cfg(feature = #feature)))]),
                    ]);
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
                    foreign_mod.attrs.extend([
                    {
                        let feature_gates = feature_gates();
                        syn::parse_quote!(#[cfg(any(#(#feature_gates),*))])
                    },
                    {
                        let feature_gates = feature_gates();
                        syn::parse_quote!(#[cfg_attr(docsrs, doc(cfg(any(#(#feature_gates),*))))])
                    },
                ]);
                }

                // Adds items to mod
                foreign_mod.items.extend(items);

                Item::ForeignMod(foreign_mod)
            }));

        i.items.extend(self.stub_libs.iter().map(|stub_lib_name| {
            syn::parse_quote! {
                #[cfg(feature = #stub_lib_name)]
                #[link(name = #stub_lib_name, kind = "static")]
                extern "C" {}
            }
        }));
    }
}

pub struct Sort;

impl VisitMut for Sort {
    fn visit_file_mut(&mut self, i: &mut syn::File) {
        // Need to visit children first as extern mods need to be identified.
        syn::visit_mut::visit_file_mut(self, i);

        // Sorts items on alphabetical order based on normalized identifier.
        // Bindgen items will be moved to the start.
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
                Item::Static(i) => (4, i.ident.to_string()),
                Item::Const(i) => (4, i.ident.to_string()),
                Item::Trait(i) => (4, i.ident.to_string()),
                Item::TraitAlias(i) => (4, i.ident.to_string()),
                Item::Type(i) => (4, i.ident.to_string()),
                Item::Enum(i) => (4, i.ident.to_string()),
                Item::Struct(i) => (4, i.ident.to_string()),
                Item::Impl(i) => {
                    let ident =
                        match &*i.self_ty {
                            Type::Path(path_type) => path_type.path.segments.iter().fold(
                                String::new(),
                                |acc, segment| {
                                    let ident_string = segment.ident.to_string();
                                    if acc.is_empty() {
                                        ident_string
                                    } else {
                                        acc + "::" + &ident_string
                                    }
                                },
                            ),
                            ty => {
                                log::warn!("impl on unexpected type {ty:?}");
                                String::new()
                            }
                        };
                    (4, ident)
                }
                Item::Fn(i) => (4, i.sig.ident.to_string()),
                Item::ForeignMod(i) => {
                    // For `extern` blocks, we use the first item's identifier.
                    // Would be ideal to use feature names instead.
                    let ident = match &i.items[..] {
                        // Stub blocks are already internally sorted.
                        [] => String::new(),
                        [item, ..] => match item {
                            ForeignItem::Fn(i) => i.sig.ident.to_string(),
                            ForeignItem::Static(i) => i.ident.to_string(),
                            ForeignItem::Type(i) => i.ident.to_string(),
                            i => {
                                log::warn!("Unexpected item in foreign mod: {i:?}");
                                String::new()
                            }
                        },
                    };
                    (4, ident)
                }
                Item::Union(i) => (4, i.ident.to_string()),
                i => {
                    log::warn!("Unexpected item: {i:?}");
                    (10, String::new())
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
