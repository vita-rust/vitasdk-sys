use std::env;
use std::fs;
use std::path::PathBuf;

// Libraries to be excluded
static LIBRARY_BLOCKLIST: [&str; 4] = [
    // Defines `__aeabi_uidiv`, which is also defined by compiler_builtins.
    "libSceSysclibForDriver_stub.a",
    // Defines `__aeabi_unwind_cpp_pr0` and probably other symbols that seem
    // to collide with std.
    "libSceLibc_stub.a",
    // This one overrides pthread_getspecific and friends, which makes the app
    // crash when using thread locals...
    "libSceLibMonoBridge_stub.a",
    // Conflicts with compiler_builtins
    "libSceRtabi_stub.a",
];

fn main() {
    // Do not link libraries for docs.rs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    if let Ok(sdk) = env::var("VITASDK").map(PathBuf::from) {
        let lib_dir = sdk.join("arm-vita-eabi").join("lib");
        println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());

        // Statically link to all "libSce*.a" in the link directory
        for lib_name in fs::read_dir(lib_dir)
            .expect("Could not open VITASDK lib directory")
            .filter_map(|e| {
                let name = e.ok()?.file_name().into_string().ok()?;
                if name.ends_with("_stub.a")
                    && name.starts_with("libSce")
                    && !LIBRARY_BLOCKLIST.contains(&&name[..])
                {
                    name.strip_suffix(".a")
                        .and_then(|n| n.strip_prefix("lib"))
                        .map(|n| n.to_owned())
                } else {
                    None
                }
            })
        {
            println!("cargo:rustc-link-lib=static={}", lib_name);
        }
    } else {
        println!("cargo:warning=$VITASDK not set!");
    }
}
