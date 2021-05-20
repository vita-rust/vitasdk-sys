use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use lazy_static::lazy_static;

mod config;
use config::Config;

const LIB_RS_PRELUDE: &str = "\
    #![no_std]\n\
    #![allow(non_camel_case_types)]\n\
    #![allow(non_snake_case)]\n\
    #![allow(non_upper_case_globals)]\n\n\
";

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
}

fn generate_lib_rs(modules: &[String]) {
    let mut lib = String::from(LIB_RS_PRELUDE);
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
    for entry in fs::read_dir(base).unwrap() {
        let entry = entry.unwrap();
        // Ignoring vitasdk.h and vitasdkkern.h for now
        if entry.metadata().unwrap().is_dir() {
            generate_module(&entry.path(), "../src".as_ref(), base.as_ref());
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
}

fn generate_module(path: &Path, dst_dir: &Path, base: &Path) -> bool {
    if path.is_dir() {
        let new_dst_dir = dst_dir.join(path.file_name().unwrap());
        // Two spaces used to align output
        println!("Created  directory at {:?}", &new_dst_dir);
        fs::create_dir(&new_dst_dir).ok();

        let mut modules = Vec::new();
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            if generate_module(&entry.path(), &new_dst_dir, base) {
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
        generate_mod_rs(&new_dst_dir, &modules);
    } else {
        let module_name = path.file_stem().unwrap().to_str().unwrap().replace('_', "");

        // Includes in vitasdk, used for C standard libraries (such as stdarg).
        let vitasdk_path = env::var("VITASDK").unwrap();
        let vitasdk_include = Path::new(&vitasdk_path)
            .join("arm-vita-eabi")
            .join("include");

        let mut builder = bindgen::Builder::default()
            .header(path.to_str().unwrap().to_owned())
            .detect_include_paths(false)
            .clang_args(&["-target", "arm-vita-eabi"])
            .clang_arg("-Ivita-headers/include")
            .clang_arg("-I".to_owned() + vitasdk_include.to_str().unwrap())
            .use_core()
            .size_t_is_usize(true)
            .ctypes_prefix("crate::ctypes")
            .rustfmt_bindings(true)
            .layout_tests(false)
            .allowlist_recursively(false)
            .prepend_enum_name(false)
            .generate_comments(false)
            .constified_enum_module(".*");

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
                for allowlist in CONFIG.default_allowlists.iter() {
                    let specific_pattern = allowlist.replace("{}", &module_name);
                    builder = builder
                        .allowlist_type(&specific_pattern)
                        .allowlist_function(&specific_pattern)
                        .allowlist_var(&specific_pattern);
                }
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
            for allowlist in CONFIG.default_allowlists.iter() {
                let specific_pattern = allowlist.replace("{}", &module_name);
                builder = builder
                    .allowlist_type(&specific_pattern)
                    .allowlist_function(&specific_pattern)
                    .allowlist_var(&specific_pattern);
            }
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
                && iter.next().map(|s| &s[..]) == Some("psp2")
        });
        if !has_types {
            match path.iter().nth(2).and_then(|os_str| os_str.to_str()) {
                Some("psp2") | Some("psp2kern") => {
                    if path.file_stem().unwrap().to_str().unwrap() != "types" {
                        includes.push(vec!["psp2".to_owned(), "types".to_owned()]);
                    }
                }
                Some("vitasdk") => (),
                Some(dir) => panic!("Unknown dir \"{}\", please hardcode it", dir),
                None => panic!("No dir?"),
            }
        }
    }
    // Sort includes alphabetically by modules from left to right
    includes.sort();
    includes
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), io::Error> {
    match fs::create_dir(&to) {
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
