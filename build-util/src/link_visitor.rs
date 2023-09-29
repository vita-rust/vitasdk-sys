use std::{
    collections::{HashMap, HashSet},
    path::Path,
    rc::Rc,
};

use crate::vita_headers_db::{missing_features_filter, stub_lib_name, VitaDb};
use syn::visit_mut::VisitMut;

const DEFINED_ELSEWHERE_FUNCTIONS: [(&str, &str); 3] = [
    // Defined in vitasdk newlib libc implementation
    ("vitasdk_get_tls_data", "vitasdk-utils"),
    ("vitasdk_get_pthread_data", "vitasdk-utils"),
    ("vitasdk_delete_thread_reent", "vitasdk-utils"),
];
const DEFINED_ELSEWHERE_VARIABLES: [(&str, &str); 0] = [];

pub use syn;

pub struct Link {
    /// link.function[function_name] = stub_library_name
    function: HashMap<String, Vec<Rc<str>>>,
    /// link.variable[variable_name] = stub_library_name
    variable: HashMap<String, Vec<Rc<str>>>,
    stub_libs: HashSet<Rc<str>>,
}

impl Link {
    pub fn load(db: &Path) -> Self {
        let mut link = Link {
            function: DEFINED_ELSEWHERE_FUNCTIONS
                .into_iter()
                .map(|(func, feat)| (func.into(), vec![Rc::from(feat.to_owned())]))
                .collect(),
            variable: DEFINED_ELSEWHERE_VARIABLES
                .into_iter()
                .map(|(var, feat)| (var.into(), vec![Rc::from(feat.to_owned())]))
                .collect(),
            stub_libs: HashSet::new(),
        };

        let mut db = VitaDb::load(db);

        let mut predicate = missing_features_filter();

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
                            .push(Rc::clone(&stub_lib_name))
                    }

                    for variable_name in lib.variable_nids.keys() {
                        link.variable
                            .entry(variable_name.clone())
                            .or_default()
                            .push(Rc::clone(&stub_lib_name))
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
            panic!("Missing features in vitasdk-sys `Cargo.toml`. \
                Please run `cargo run -p vitasdk-sys-build-util -- stub-libs --as-features` and replace stub lib features in vitasdk-sys Cargo.toml with outputed ones.")
        }

        link
    }
}

impl VisitMut for Link {
    fn visit_foreign_item_fn_mut(&mut self, i: &mut syn::ForeignItemFn) {
        let symbol = i.sig.ident.to_string();

        match self.function.get(&symbol) {
            None => panic!("Undefined foreign fn `{symbol}`"),
            Some(features) => match features.as_slice() {
                [feature] => i.attrs.extend([
                    syn::parse_quote!(#[cfg(feature = #feature)]),
                    syn::parse_quote!(#[cfg_attr(docsrs, doc(cfg(feature = #feature)))]),
                ]),
                features => {
                    log::warn!(
                        "Function `{symbol}` is defined in multiple locations: {features:?}"
                    );
                    let feature_gates = || {
                        features.iter().map(|feature| -> syn::MetaNameValue {
                            syn::parse_quote!(feature = #feature)
                        })
                    };
                    i.attrs.extend([
                        {
                            let feature_gates = feature_gates();
                            syn::parse_quote!(#[cfg(any(#(#feature_gates),*))])
                        },
                        {
                            let feature_gates = feature_gates();
                            syn::parse_quote!(#[cfg_attr(docsrs, doc(cfg(any(#(#feature_gates),*))))])
                        },
                    ])
                }
            },
        }

        syn::visit_mut::visit_foreign_item_fn_mut(self, i)
    }

    fn visit_foreign_item_static_mut(&mut self, i: &mut syn::ForeignItemStatic) {
        let symbol = i.ident.to_string();

        match self.variable.get(&symbol) {
            None => panic!("Undefined foreign static `{symbol}`"),
            Some(features) => {
                let [feature] = features.as_slice() else {
                    panic!("Variable `{symbol}` is defined in multiple locations: {features:?}")
                };
                i.attrs.extend([
                    syn::parse_quote!(#[cfg(feature = #feature)]),
                    syn::parse_quote!(#[cfg_attr(docsrs, doc(cfg(feature = #feature)))]),
                ])
            }
        }

        syn::visit_mut::visit_foreign_item_static_mut(self, i)
    }

    fn visit_file_mut(&mut self, i: &mut syn::File) {
        i.items.extend(self.stub_libs.iter().map(|stub_lib_name| {
            syn::parse_quote! {
                #[cfg(feature = #stub_lib_name)]
                #[link(name = #stub_lib_name, kind = "static")]
                extern "C" {}
            }
        }));

        syn::visit_mut::visit_file_mut(self, i)
    }
}
