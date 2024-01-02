use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "log-build")]
    env_logger::init();

    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-env-changed=VITASDK");
    let vitasdk = match env::var("VITASDK") {
        Ok(vitasdk) => {
            let vitasdk = PathBuf::from(vitasdk);
            let sysroot = vitasdk.join("arm-vita-eabi");

            assert!(
                sysroot.exists(),
                "VITASDK's sysroot does not exist, please install or update vitasdk first"
            );

            let lib = sysroot.join("lib");
            assert!(lib.exists(), "VITASDK's `lib` directory does not exist");
            println!("cargo:rustc-link-search=native={}", lib.to_str().unwrap());

            vitasdk
        }
        Err(env::VarError::NotPresent) => {
            panic!("VITASDK env var is not set")
        }
        Err(env::VarError::NotUnicode(s)) => {
            panic!("VITASDK env var is not a valid unicode but got: {s:?}")
        }
    };

    // Just to mark it as used
    #[cfg(not(feature = "bindgen"))]
    let _ = vitasdk;

    #[cfg(feature = "bindgen")]
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings_output = out_dir.join("bindings.rs");

        let vita_headers_include = vitasdk.join("arm-vita-eabi").join("include");
        let db = vitasdk.join("share").join("vita-headers").join("db");
        assert!(
            vita_headers_include.exists(),
            "VITASDK's `include` directory does not exist"
        );
        assert!(db.exists(), "VITASDK's `db` directory does not exist");

        let is_build_rs = true;

        vitasdk_sys_build_util::bindgen::generate(
            &vita_headers_include,
            &db,
            &bindings_output,
            &vitasdk_sys_manifest(),
            is_build_rs,
        );
    }
}

#[cfg(feature = "bindgen")]
fn vitasdk_sys_manifest() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml")
}
