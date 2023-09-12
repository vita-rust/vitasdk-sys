use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use bindgen::EnumVariation;
use lazy_static::lazy_static;

mod config;
use config::Config;

lazy_static! {
    static ref CONFIG: Config = {
        let config_string = String::from_utf8(fs::read("config.toml").unwrap()).unwrap();
        toml::from_str(&config_string).unwrap()
    };
}

fn main() {
    // Must read config before changing working directory.
    lazy_static::initialize(&CONFIG);

    generate_all_modules();
    copy_dir("../static".as_ref(), "../src".as_ref()).expect("Could not copy ../static to ../src");
    let cargo = env::var_os("CARGO");
    let status = Command::new(cargo.as_deref().unwrap_or_else(|| "cargo".as_ref()))
        .arg("fmt")
        .arg("--manifest-path=../Cargo.toml")
        .status()
        .expect("Spawning `cargo fmt` on generated bindings");
    assert!(status.success(), "Failed `cargo fmt` command: {status:?}");
}

fn generate_lib_rs(modules: &[String]) {
    let mut lib = CONFIG.lib_rs_prelude.clone();
    lib.push_str("pub mod ctypes;\n");
    for module in modules {
        lib.push_str(&format!("pub mod {};\n", module));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("../src/lib.rs")
        .unwrap();
    file.write_all(lib.as_bytes()).unwrap();
}

fn generate_mod_rs(path: &Path, modules: &[String]) {
    let mut mod_string = String::from("");
    for module in modules {
        mod_string.push_str(&format!("pub mod {};\n", module));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.join("mod.rs"))
        .unwrap();
    file.write_all(mod_string.as_bytes()).unwrap();
}

fn generate_all_modules() {
    let base = "vita-headers/include";
    let mut modules = Vec::new();
    let dst_dir: &Path = "../src".as_ref();
    for entry in fs::read_dir(base).unwrap() {
        let entry = entry.unwrap();
        // Ignoring vitasdk.h and vitasdkkern.h for now
        if entry.metadata().unwrap().is_dir() {
            generate_module(&entry.path(), dst_dir);
            #[rustfmt::skip]
            modules.push(
                entry
                    .path()
                    .file_stem().unwrap()
                    .to_str().unwrap()
                    .to_owned(),
            );
        }
    }
    modules.sort();
    generate_lib_rs(&modules);
    merge_module_roots(dst_dir);
}

fn generate_module(path: &Path, dst_dir: &Path) -> bool {
    if path.is_dir() {
        let new_dst_dir = dst_dir.join(path.file_name().unwrap());
        // Two spaces used to align output
        println!("Created  directory at {:?}", &new_dst_dir);
        fs::create_dir(&new_dst_dir).ok();

        let mut modules = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if generate_module(&entry.path(), &new_dst_dir) {
                #[rustfmt::skip]
                modules.push(
                    entry
                    .path()
                    .file_stem().unwrap()
                    .to_str().unwrap()
                    .to_owned(),
                );
            }
        }
        modules.sort();
        // Needed because sometimes there are .h and directories with the same name
        // These duplicated files will be found and merged later on merge_module_roots
        modules.dedup();
        generate_mod_rs(&new_dst_dir, &modules);
    } else {
        let module_name = path.file_stem().unwrap().to_str().unwrap().replace('_', "");

        // Includes in vitasdk, used for C standard libraries (such as stdarg).
        let vitasdk_path = env::var("VITASDK").unwrap();
        let vitasdk_include = Path::new(&vitasdk_path)
            .join("arm-vita-eabi")
            .join("include");

        let mut builder = bindgen::builder()
            .header(path.to_str().unwrap().to_owned())
            .detect_include_paths(false)
            .clang_args(&["-target", "arm-vita-eabi"])
            .clang_arg("-Ivita-headers/include")
            .clang_arg("-I".to_owned() + vitasdk_include.to_str().unwrap())
            .use_core()
            .size_t_is_usize(true)
            .ctypes_prefix("crate::ctypes")
            .layout_tests(false)
            .allowlist_recursively(false)
            .prepend_enum_name(false)
            .generate_comments(false)
            .formatter(bindgen::Formatter::None) // We run `cargo fmt` later
            .default_enum_style(EnumVariation::ModuleConsts);

        let includes = get_includes(path);
        if !includes.is_empty() {
            let mut imports: String = includes
                .iter()
                .map(|include| {
                    include.iter().fold(
                        String::from("#[allow(unused_imports)]\nuse crate"),
                        |mut acc, part| {
                            acc.push_str("::");
                            acc.push_str(part);
                            acc
                        },
                    ) + "::*;\n"
                })
                .collect();
            // Removes the last newline
            imports.pop();
            builder = builder.raw_line(imports);
        }

        if let Some(lists) = CONFIG
            .lists
            .iter()
            .find(|list| list.files.iter().any(|file| path.ends_with(file)))
        {
            if lists.skip {
                println!("Skipped file at {:?}", path);
                return false;
            }
            if !lists.exclude_default {
                builder = builder.allowlist_file(path.to_str().expect("path not valid utf-8"));
            }

            let raw_lines = lists.extra_lines.iter().fold(String::new(), |acc, import| {
                if acc.is_empty() {
                    import.to_owned()
                } else {
                    acc + "\n" + import
                }
            });
            if !raw_lines.is_empty() {
                builder = builder.raw_line(raw_lines);
            }

            for allowlist in lists.allowlists.iter() {
                let specific_pattern = allowlist.replace("{}", &module_name);
                builder = builder
                    .allowlist_type(&specific_pattern)
                    .allowlist_function(&specific_pattern)
                    .allowlist_var(&specific_pattern);
            }

            for allowlist in lists.allowlists_type.iter() {
                let specific_pattern = allowlist.replace("{}", &module_name);
                builder = builder.allowlist_type(&specific_pattern)
            }

            for allowlist in lists.allowlists_function.iter() {
                let specific_pattern = allowlist.replace("{}", &module_name);
                builder = builder.allowlist_function(&specific_pattern)
            }

            for allowlist in lists.allowlists_var.iter() {
                let specific_pattern = allowlist.replace("{}", &module_name);
                builder = builder.allowlist_var(&specific_pattern)
            }
        } else {
            // List not in config, use just default
            builder = builder.allowlist_file(path.to_str().expect("path not valid utf-8"));
        }

        let bindings = builder.generate().expect("Unable to generate bindings");

        let original_file_name: &Path = path.file_name().unwrap().as_ref();
        let new_file_path = dst_dir.join(original_file_name.with_extension("rs"));
        println!("Generated bindings at {:?}", &new_file_path);
        bindings
            .write_to_file(&new_file_path)
            .expect("Couldn't write bindings");
    }
    true
}

fn get_includes(path: &Path) -> Vec<Vec<String>> {
    let mut includes = Vec::new();
    let string = String::from_utf8(fs::read(path).unwrap()).unwrap();
    for line in string.lines() {
        if let Some(original_path) = line
            .trim()
            .strip_prefix("#include <")
            .and_then(|l| l.strip_suffix(".h>"))
        {
            let include: Vec<String> = original_path.split('/').map(|c| c.to_owned()).collect();
            // Skip standard library includes, such as stdarg.h
            if include.len() > 1 {
                includes.push(include);
            }
        }
    }
    // Add "types.h" to includes that have any other include but not types
    if !includes.is_empty() {
        // Check if "psp2/types.h" is in the include list
        let has_types = includes.iter().any(|include| {
            let mut iter = include.iter().rev();
            iter.next().map(|s| &s[..]) == Some("types")
                && iter.next().map(|s| &s[..]) == Some("psp2common")
        });
        if !has_types {
            match path.iter().nth(2).and_then(|os_str| os_str.to_str()) {
                Some("psp2") | Some("psp2kern") | Some("psp2common") | Some("vitasdk") => {
                    if path.file_stem().unwrap().to_str().unwrap() != "types" {
                        includes.push(vec!["psp2common".to_owned(), "types".to_owned()]);
                    }
                }
                Some(dir) => panic!("Unknown dir \"{}\", please hardcode it", dir),
                None => panic!("No dir?"),
            }
        }
    }
    // Sort includes alphabetically by modules from left to right
    includes.sort();
    includes
}

/// Sometimes there is both a directory with the same name as a header file,
/// such as psp2/paf.h and psp2/paf/. As we're using mod.rs as a convention,
/// we're generating at least two filesin that case : psp2/paf.rs and
/// psp2/paf/mod.rs, which is ambiguous because both refer to the same module.
///
/// This function finds all of those cases inside dst_dir and merges both files
/// into mod.rs.
fn merge_module_roots(dst_dir: &Path) {
    for entry in fs::read_dir(dst_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let new_dst_dir = dst_dir.join(path.file_name().unwrap());
            println!("Merging files inside directory {:?}", &new_dst_dir);
            merge_module_roots(&new_dst_dir);
        } else {
            let module_name = path.file_stem().unwrap();
            let mod_rs_path = dst_dir.join(module_name).join("mod.rs");
            if mod_rs_path.exists() {
                println!(
                    "Merging files {} and {}",
                    path.display(),
                    mod_rs_path.display()
                );
                merge(path.as_ref(), mod_rs_path.as_ref()).expect("error merging files");
            }
        }
    }
}

fn merge(from: &Path, into: &Path) -> Result<(), io::Error> {
    let mut from_file = fs::OpenOptions::new().read(true).open(from)?;

    let mut into_file = fs::OpenOptions::new().append(true).open(into)?;

    let _count = io::copy(&mut from_file, &mut into_file)?;
    fs::remove_file(from)?;
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), io::Error> {
    match fs::create_dir(to) {
        Ok(_) => (),
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => (),
        Err(err) => return Err(err),
    }
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(&entry.path(), &to.join(entry.file_name()))?;
        }
    }
    Ok(())
}
