use std::{env, fs, io, path::Path, process};

use quote::ToTokens;
use syn::{ForeignItem, Item, Type};

use crate::link_visitor::{
    syn::{self, visit_mut::VisitMut},
    Link,
};

pub fn generate(
    vita_headers_include: &Path,
    db: &Path,
    bindings_output: &Path,
    vitasdk_sys_manifest: &Path,
    is_build_rs: bool,
) {
    log::info!("Generating preprocessed bindings");
    let headers = vitasdk_sys_manifest.parent().unwrap().join("headers");
    let bindings = generate_preprocessed_bindings(&headers, vita_headers_include, is_build_rs);

    let mut bindings = syn::parse_file(&bindings).expect("failed to parse file");

    log::info!(
        "Loading vita-headers metadata yaml files from \"{}\"",
        db.to_string_lossy()
    );
    let mut link = Link::load(db, vitasdk_sys_manifest);
    link.visit_file_mut(&mut bindings);

    // We sort items here so generated bindings don't depend on the included order.
    sort_items(&mut bindings.items);

    let bindings = bindings.into_token_stream();

    {
        log::info!(
            "Writing postprocessed bindings into {}",
            bindings_output.to_string_lossy()
        );
        let mut bindings_output = io::BufWriter::new(fs::File::create(bindings_output).unwrap());
        use std::io::Write;
        write!(bindings_output, "{bindings}").unwrap();
    }

    let cargo = env::var_os("CARGO");
    let mut fmt_cmd = process::Command::new(cargo.as_deref().unwrap_or_else(|| "cargo".as_ref()));
    fmt_cmd.args(["fmt", "--"]);
    fmt_cmd.arg(bindings_output);

    log::info!("Running formatting command: {fmt_cmd:?}");

    let fmt_result = fmt_cmd.status();
    match fmt_result {
        Ok(status) => {
            if status.success() {
                log::info!("Formatting command finished");
            } else if is_build_rs {
                log::warn!("Formatting command failed with status: {status:?}");
            } else {
                panic!("Formatting command failed with status: {status:?}");
            }
        }
        Err(error) => {
            if is_build_rs {
                log::warn!("Formatting command failed with error: {error:?}");
            } else {
                panic!("Formatting command failed with error: {error:?}");
            }
        }
    }
}

fn generate_preprocessed_bindings(
    headers: &Path,
    vita_headers_include: &Path,
    is_build_rs: bool,
) -> String {
    let builder = bindgen::Builder::default()
        .header(vita_headers_include.join("vitasdk.h").to_str().unwrap())
        .header(vita_headers_include.join("vitasdkkern.h").to_str().unwrap())
        .clang_arg("-I".to_string() + headers.to_str().unwrap())
        .clang_arg("-I".to_string() + vita_headers_include.to_str().unwrap())
        .clang_args(&["-target", "armv7a-none-eabihf"])
        .use_core()
        .ctypes_prefix("crate::ctypes")
        .generate_comments(false)
        .prepend_enum_name(false)
        .layout_tests(false)
        .formatter(bindgen::Formatter::None);

    let builder = if is_build_rs {
        builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    } else {
        builder
    };

    builder.generate().expect("bindgen failed").to_string()
}

/// Sorts items on alphabetical order based on normalized identifier.
/// Bindgen items will be moved to the start.
fn sort_items(items: &mut [Item]) {
    items.sort_by_cached_key(|item| {
        let (precedence, ident) =
            match item {
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
                    let ident = match i.items.len() {
                        // Could use link name here. But it's not strictly necessary as they're
                        // already sorted.
                        0 => String::new(),
                        len => {
                            if len > 1 {
                                log::warn!("Unexpected ForeignMod with multiple items: {i:?}");
                            }

                            match &i.items[0] {
                                ForeignItem::Fn(i) => i.sig.ident.to_string(),
                                ForeignItem::Static(i) => i.ident.to_string(),
                                ForeignItem::Type(i) => i.ident.to_string(),
                                i => {
                                    log::warn!("Unexpected item in foreign mod: {i:?}");
                                    String::new()
                                }
                            }
                        }
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
