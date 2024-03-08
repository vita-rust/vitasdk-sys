use std::{env, fs, io, path::Path, process};

use bindgen::callbacks::ParseCallbacks;
use quote::ToTokens;
use syn::visit_mut::VisitMut;

use crate::{
    visitors::{Link, Sort},
    vita_headers_db::VitaDb,
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
    let db = VitaDb::load(db);
    let link = Link::load(&db);

    if is_build_rs {
        db.check_missing_manifest_features(vitasdk_sys_manifest);
    } else {
        db.update_manifest_features(vitasdk_sys_manifest);
    }

    link.visit(&mut bindings);

    // We sort items here so generated bindings don't depend on the included order.
    Sort.visit_file_mut(&mut bindings);

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

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        let comment = comment.replace("@param[int]", "@param[in]");
        let comment = comment.replace("[inout]", "[in,out]");

        // doxygen_rs doesn't handle tabs well
        let comment = comment.replace('\t', " ");
        // Fixes links to functions and types
        let comment = comment.replace("@see ::", "@see crate::");

        // Escape square brackets for non-links
        let comment =
            lazy_regex::regex_replace_all!("\\[([\\d]+)\\]", &comment, |_, num: &str| format!(
                "\\[{num}\\]"
            ));

        // Without space these brackets are considered to be a link
        let comment = comment.replace(
            "[1,SCE_GXM_MAX_SCENES_PER_RENDERTARGET]",
            "[1, SCE_GXM_MAX_SCENES_PER_RENDERTARGET]",
        );

        let comment = doxygen_rs::transform(&comment);

        let comment = comment.strip_prefix("!<").unwrap_or(&comment);
        let comment = comment.strip_prefix('!').unwrap_or(&comment);
        let comment = comment.strip_prefix('<').unwrap_or(&comment);
        let comment = comment.trim();

        Some(comment.to_string())
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
        .generate_comments(true)
        .parse_callbacks(Box::new(Callbacks))
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
