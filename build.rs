use std::{env, fs, io, path::PathBuf, process};

use camino::{Utf8Path, Utf8PathBuf};
use quote::ToTokens;
use regex::Regex;
use vitasdk_sys_build_util::link_visitor::{
    syn::{self, visit_mut::VisitMut},
    Link,
};

fn vitasdk_sys_manifest() -> Utf8PathBuf {
    Utf8PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml")
}

fn main() {
    env_logger::init();

    println!("cargo:rerun-if-env-changed=VITASDK");
    match env::var("VITASDK") {
        Ok(vitasdk) => {
            let sysroot = Utf8PathBuf::from(vitasdk).join("arm-vita-eabi");

            assert!(
                sysroot.exists(),
                "VITASDK's sysroot does not exist, please install or update vitasdk first"
            );

            let lib = sysroot.join("lib");
            assert!(lib.exists(), "VITASDK's `lib` directory does not exist");
            println!("cargo:rustc-link-search=native={lib}");
        }
        Err(env::VarError::NotPresent) => {
            if env::var("DOCS_RS").is_err() {
                panic!("VITASDK env var is not set")
            }
        }
        Err(env::VarError::NotUnicode(s)) => {
            panic!("VITASDK env var is not a valid unicode but got: {s:?}")
        }
    }

    let vita_headers_submodule = Utf8Path::new("vita-headers");

    let original_include = vita_headers_submodule.join("include");
    println!("cargo:rerun-if-changed={original_include}");

    let out_dir = Utf8PathBuf::from(env::var("OUT_DIR").unwrap());
    let include = out_dir.join("vita_headers_localized_include");

    localize_bindings(&original_include, &include);

    let headers = Utf8Path::new("src/headers");
    println!("cargo:rerun-if-changed={headers}");
    for entry in headers.read_dir_utf8().unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), include.join(entry.file_name())).unwrap();
    }

    log::info!("Generating preprocessed bindings");
    let bindings = generate_preprocessed_bindings(&include);

    log::info!("Parsing preprocessed bindings");
    let mut bindings = syn::parse_file(&bindings).unwrap();

    let db = vita_headers_submodule.join("db");

    log::info!("Loading vita-headers metadata yaml files from \"{db}\"");
    let mut link = Link::load(db.as_ref(), vitasdk_sys_manifest().as_ref());
    link.visit_file_mut(&mut bindings);

    let bindings = bindings.into_token_stream();

    let bindings_output = out_dir.join("bindings.rs");

    {
        log::info!("Writing postprocessed bindings into {bindings_output}");
        let mut bindings_output = io::BufWriter::new(fs::File::create(&bindings_output).unwrap());
        use std::io::Write;
        write!(bindings_output, "{bindings}").unwrap();
    }

    let cargo = env::var_os("CARGO");
    let mut fmt_cmd = process::Command::new(cargo.as_deref().unwrap_or_else(|| "cargo".as_ref()));
    fmt_cmd.args(["fmt", "--"]);
    fmt_cmd.arg(bindings_output);

    log::info!("Running formatting command: {fmt_cmd:?}");
    let exit_status = fmt_cmd.status().unwrap();
    if exit_status.success() {
        log::info!("Formatting command finished");
    } else {
        log::warn!("Formatting command failed with status: {exit_status:?}");
    }
}

fn generate_preprocessed_bindings(include: &Utf8Path) -> String {
    bindgen::Builder::default()
        .header(include.join("all.h"))
        .clang_args(&["-target", "armv7a-none-eabihf"])
        .parse_callbacks(Box::new(BindgenCallbacks::new()))
        .use_core()
        .ctypes_prefix("crate::ctypes")
        .generate_comments(false)
        .prepend_enum_name(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("Bindgen failed")
        .to_string()
}

// Replace `#include <>` with `#include ""`
fn localize_bindings(original_include: &Utf8Path, localized_include: &Utf8Path) {
    struct Localizer<'a> {
        include_regex: Regex,
        local_include_root: &'a Utf8Path,
    }

    impl<'a> Localizer<'a> {
        fn new(local_include_root: &'a Utf8Path) -> Self {
            Localizer {
                include_regex: Regex::new(r"#include <([\w/]+\.h)>").unwrap(),
                local_include_root,
            }
        }

        fn localize_dir(&self, original_include: &Utf8Path, local_include: &Utf8Path) {
            fs::create_dir(local_include)
                .or_else(|e| match e.kind() {
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(e),
                })
                .unwrap();
            for entry in original_include.read_dir_utf8().unwrap() {
                let entry = entry.unwrap();
                let local_entry = local_include.join(entry.file_name());
                let original_entry = entry.path();
                let ty = entry.file_type().unwrap();
                if ty.is_dir() {
                    self.localize_dir(original_entry, &local_entry)
                } else if ty.is_file() {
                    self.localize_file(original_entry, &local_entry)
                } else {
                    panic!("{original_entry:?} is bad file type: {ty:?}")
                }
            }
        }

        // TODO: trace span to better find sources of errors
        fn localize_file(&self, original_include: &Utf8Path, local_include: &Utf8Path) {
            let relative_local_root = local_include
                .strip_prefix(self.local_include_root)
                .unwrap()
                .ancestors()
                .skip(2)
                .map(|_| "..")
                .collect::<Utf8PathBuf>();
            let original_include = fs::read_to_string(original_include).unwrap();
            let new_include = self.include_regex.replace_all(
                &original_include,
                |captures: &regex::Captures<'_>| {
                    // Do not replace if it's one of these include paths
                    if let "stddef.h" | "stdint.h" | "stdarg.h" = &captures[1] {
                        return captures[0].to_owned();
                    }
                    let path = relative_local_root.join(&captures[1]);
                    format!("#include \"{path}\"")
                },
            );

            fs::write(local_include, new_include.as_ref()).unwrap();
        }
    }

    Localizer::new(localized_include).localize_dir(original_include, localized_include);
}

#[derive(Debug)]
struct BindgenCallbacks {
    out_dir: PathBuf,
}

impl BindgenCallbacks {
    fn new() -> Self {
        BindgenCallbacks {
            out_dir: PathBuf::from(env::var_os("OUT_DIR").unwrap())
                .canonicalize()
                .unwrap(),
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for BindgenCallbacks {
    fn include_file(&self, filename: &str) {
        if !Utf8Path::new(filename)
            .canonicalize()
            .unwrap()
            .starts_with(&self.out_dir)
        {
            println!("cargo:rerun-if-changed={filename}")
        }
    }

    fn read_env_var(&self, key: &str) {
        println!("cargo:rerun-if-env-changed={key}");
    }
}
