use std::{
    collections::{hash_map, HashMap},
    fmt, fs, io, mem,
    path::{Path, PathBuf},
};

use serde::Deserialize;

/// Blacklisted stub libs containing conflicting symbol definitions.
const CONFLICTING_STUB_LIBS: [&str; 4] = [
    // Defines `__aeabi_uidiv`, which is also defined by compiler_builtins.
    "SceSysclibForDriver_stub",
    // Defines `__aeabi_unwind_cpp_pr0` and probably other symbols that seem
    // to collide with std.
    "SceLibc_stub",
    // This one overrides pthread_getspecific and friends, which makes the app
    // crash when using thread locals...
    "SceLibMonoBridge_stub",
    // Conflicts with compiler_builtins
    "SceRtabi_stub",
];

#[derive(Default)]
pub struct VitaDb {
    pub imports_by_firmware: HashMap<String, VitaImports>,
}

impl VitaDb {
    pub fn load(db: &Path) -> Self {
        let mut out = VitaDb {
            imports_by_firmware: HashMap::new(),
        };

        for version_dir in db.read_dir().unwrap() {
            for yml in version_dir.unwrap().path().read_dir().unwrap() {
                let yml = yml.unwrap().path();
                log::debug!("Loading: {}", yml.display());
                let rdr = io::BufReader::new(fs::File::open(yml).unwrap());
                out.add_imports(serde_yaml::from_reader(rdr).unwrap());
            }
        }

        out
    }

    pub fn split_conflicting(&mut self) -> Self {
        self.split_filter(|imports, mod_name, _, lib_name, lib| {
            let stub_lib = stub_lib_name(
                mod_name,
                lib_name,
                lib.stub_name.as_deref(),
                lib.kernel,
                &imports.firmware,
            )
            .to_string();
            CONFLICTING_STUB_LIBS.contains(&stub_lib.as_str())
        })
    }

    pub fn split_kernel(&mut self) -> Self {
        self.split_filter(|_, _, _, _, lib| lib.kernel)
    }

    fn split_filter<F>(&mut self, mut f: F) -> Self
    where
        F: FnMut(&VitaImports, &str, &VitaImportsModule, &str, &VitaImportsLib) -> bool,
    {
        let mut split = VitaDb::default();
        self.imports_by_firmware.retain(|_, imports| {
            split.add_imports(imports.split_filter(&mut f));
            !imports.is_empty()
        });
        split
    }

    pub fn stub_lib_names(&self) -> impl Iterator<Item = String> + '_ {
        self.imports_by_firmware.values().flat_map(|imports| {
            imports.modules.iter().flat_map(|(module_name, module)| {
                module.libraries.iter().map(|(lib_name, lib)| {
                    stub_lib_name(
                        module_name,
                        lib_name,
                        lib.stub_name.as_deref(),
                        lib.kernel,
                        &imports.firmware,
                    )
                    .to_string()
                })
            })
        })
    }

    pub fn add_imports(&mut self, imports: VitaImports) {
        if imports.is_empty() {
            return;
        }
        let entry = self.imports_by_firmware.entry(imports.firmware.clone());
        match entry {
            hash_map::Entry::Occupied(o) => o.into_mut().merge_from(imports),
            hash_map::Entry::Vacant(v) => {
                v.insert(imports);
            }
        }
    }
}

pub fn missing_features_filter() -> impl FnMut(&String) -> bool {
    const VITASDK_SYS_MANIFEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../Cargo.toml");

    #[derive(Debug, serde::Deserialize)]
    struct CargoManifest {
        #[serde(default)]
        features: HashMap<String, Vec<String>>,
    }

    let manifest = fs::read_to_string(VITASDK_SYS_MANIFEST).unwrap();
    let manifest: CargoManifest = toml::from_str(&manifest).unwrap();

    move |stub_lib| !manifest.features.contains_key(stub_lib)
}

pub fn missing_libs_filter() -> impl FnMut(&String) -> bool {
    let lib_dir = PathBuf::from(std::env::var_os("VITASDK").unwrap()).join("arm-vita-eabi/lib");
    assert!(lib_dir.is_dir());
    move |stub_lib| !lib_dir.join(format!("lib{stub_lib}.a")).exists()
}

#[derive(Deserialize)]
pub struct VitaImports {
    pub version: i32,
    pub firmware: String,
    pub modules: HashMap<String, VitaImportsModule>,
}

impl VitaImports {
    pub fn clone_emptied(&self) -> Self {
        VitaImports {
            version: self.version,
            firmware: self.firmware.clone(),
            modules: HashMap::new(),
        }
    }

    pub fn take(&mut self) -> Self {
        let empty = self.clone_emptied();
        mem::replace(self, empty)
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn add_module<S: Into<String>>(&mut self, name: S, module: VitaImportsModule) {
        if !module.is_empty() {
            let old = self.modules.insert(name.into(), module);
            assert!(old.is_none());
        }
    }

    pub fn merge_from(&mut self, other: Self) {
        assert_eq!(self.version, other.version);
        assert_eq!(self.firmware, other.firmware);
        self.modules.extend(other.modules);
    }

    fn split_filter<F>(&mut self, mut f: F) -> Self
    where
        F: FnMut(&VitaImports, &str, &VitaImportsModule, &str, &VitaImportsLib) -> bool,
    {
        let empty = self.clone_emptied();
        let mut split = self.clone_emptied();
        self.modules.retain(|module_name, module| {
            split.add_module(
                module_name,
                module.split_filter(|module, lib_name, lib| {
                    f(&empty, module_name, module, lib_name, lib)
                }),
            );
            !module.is_empty()
        });
        split
    }
}

#[derive(Deserialize)]
pub struct VitaImportsModule {
    pub nid: u32,
    pub libraries: HashMap<String, VitaImportsLib>,
}

impl VitaImportsModule {
    pub fn clone_emptied(&self) -> Self {
        VitaImportsModule {
            nid: self.nid,
            libraries: HashMap::new(),
        }
    }

    pub fn take(&mut self) -> Self {
        let empty = self.clone_emptied();
        mem::replace(self, empty)
    }

    pub fn add_lib<S: Into<String>>(&mut self, name: S, lib: VitaImportsLib) {
        if !lib.is_empty() {
            let old = self.libraries.insert(name.into(), lib);
            assert!(old.is_none());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.libraries.is_empty()
    }

    fn split_filter<F>(&mut self, mut f: F) -> Self
    where
        F: FnMut(&VitaImportsModule, &str, &VitaImportsLib) -> bool,
    {
        let empty = self.clone_emptied();
        let mut split = self.clone_emptied();
        self.libraries.retain(|lib_name, lib| {
            split.add_lib(lib_name, lib.split_filter(|lib| f(&empty, lib_name, lib)));
            !lib.is_empty()
        });
        split
    }
}

#[derive(Deserialize)]
pub struct VitaImportsLib {
    pub kernel: bool,
    pub nid: u32,
    pub version: Option<u32>,
    #[serde(rename = "stubname", default)]
    pub stub_name: Option<String>,
    #[serde(rename = "functions", default)]
    pub function_nids: HashMap<String, u32>,
    #[serde(rename = "variables", default)]
    pub variable_nids: HashMap<String, u32>,
}

impl VitaImportsLib {
    fn clone_emptied(&self) -> Self {
        VitaImportsLib {
            kernel: self.kernel,
            nid: self.nid,
            version: self.version,
            stub_name: self.stub_name.clone(),
            function_nids: HashMap::new(),
            variable_nids: HashMap::new(),
        }
    }

    pub fn take(&mut self) -> Self {
        let empty = self.clone_emptied();
        mem::replace(self, empty)
    }

    pub fn is_empty(&self) -> bool {
        self.function_nids.is_empty() && self.variable_nids.is_empty()
    }

    fn split_filter<F>(&mut self, mut f: F) -> Self
    where
        F: FnMut(&VitaImportsLib) -> bool,
    {
        let mut split = self.clone_emptied();
        if f(self) {
            mem::swap(self, &mut split);
        }
        split
    }
}

pub(crate) fn stub_lib_name<'a>(
    mod_name: &'a str,
    lib_name: &'a str,
    stub_name: Option<&'a str>,
    kernel: bool,
    firmware: &'a str,
) -> impl fmt::Display + 'a {
    struct StubLibName<'a> {
        name: &'a str,
        firmware: &'a str,
    }

    impl fmt::Display for StubLibName<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.name)?;
            if self.firmware != "3.60" {
                f.write_str("_")?;
                self.firmware.split('.').try_for_each(|s| f.write_str(s))?;
            }
            f.write_str("_stub")
        }
    }

    if let Some(stubname) = stub_name {
        StubLibName {
            name: stubname,
            firmware,
        }
    } else if kernel {
        StubLibName {
            name: lib_name,
            firmware,
        }
    } else {
        StubLibName {
            name: mod_name,
            firmware,
        }
    }
}
