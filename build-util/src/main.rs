use std::{
    collections::{BTreeSet, HashSet},
    env,
    path::PathBuf,
    process::{self, Command, ExitCode},
};

use vitasdk_sys_build_util::vita_headers_db::{
    missing_features_filter, missing_libs_filter, VitaDb,
};

fn vitasdk_sys_manifest() -> PathBuf {
    let cargo = env::var_os("CARGO");
    let output = Command::new(cargo.as_deref().unwrap_or("cargo".as_ref()))
        .args(["locate-project", "--message-format", "plain", "--workspace"])
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Could not cargo locate-project; perhaps running outside from workspace directory?"
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn vita_headers_db_path() -> PathBuf {
    vitasdk_sys_manifest()
        .parent()
        .unwrap()
        .join("vita-headers/db")
}

fn print_help() {
    eprintln!(
        "\
Internal build utilities for vitasdk-sys crate

USAGE:
    vitasdk-sys-build-util [OPTIONS] <COMMAND>

Commands:
    stub-libs   Print all stub lib names
    bindgen     Generate vitasdk-sys bindings
    help        Print help

Options
    -h, --help  Print help
"
    )
}

fn main() -> ExitCode {
    env_logger::init();

    let cmd = std::env::args().nth(1);
    match cmd.as_deref() {
        Some("stub-libs") => stub_libs(),
        Some("bindgen") => bindgen(),
        Some("help" | "--help" | "-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        _ => {
            print_help();
            ExitCode::FAILURE
        }
    }
}

fn stub_libs() -> ExitCode {
    fn print_help() {
        eprintln!(
            "\
Print stub lib names

USAGE:
    vitasdk-sys-build-util stub-libs [OPTIONS]

Options:
    -h, --help           Print help
    -u, --user           Print only user stub libs. Mutually exclusive with `--kernel`.
    -k, --kernel         Print only kernel stub libs. Mutually exclusive with `--user`.
    -c, --conflicting    Print only stub libs with conflicting symbols
    --with-conflicting   Include stub libs with conflicting symbols
    --missing-features   Print only undefined vitasdk-sys stub features
    --as-features        Print stub libs as cargo features
    --all-stubs-feature  Prepend `all-stubs` feature if `--as-features` specified
    --missing-libs       Print only stub libs which do not exist in `$VITASDK/arm-vita-eabi/lib`
    --fail-if-any        Fail if any stub lib is printed
"
        )
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Flag {
        Help,
        User,
        Kernel,
        Conflicting,
        WithConflicting,
        MissingFeatures,
        AsFeatures,
        AllStubsFeature,
        MissingLibs,
        FailIfAny,
    }

    #[derive(Debug)]
    struct ParseFlagError;

    impl std::str::FromStr for Flag {
        type Err = ParseFlagError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "-h" | "--help" => Ok(Flag::Help),
                "-u" | "--user" => Ok(Flag::User),
                "-k" | "--kernel" => Ok(Flag::Kernel),
                "-c" | "--conflicting" => Ok(Flag::Conflicting),
                "--with-conflicting" => Ok(Flag::WithConflicting),
                "--missing-features" => Ok(Flag::MissingFeatures),
                "--as-features" => Ok(Flag::AsFeatures),
                "--all-stubs-feature" => Ok(Flag::AllStubsFeature),
                "--missing-libs" => Ok(Flag::MissingLibs),
                "--fail-if-any" => Ok(Flag::FailIfAny),
                _ => Err(ParseFlagError),
            }
        }
    }

    let Some(options) = std::env::args()
        .skip(2)
        .filter(|a| !a.is_empty())
        .map(|op| op.parse())
        .collect::<Result<HashSet<Flag>, ParseFlagError>>()
        .ok()
        .filter(|ops| !ops.contains(&Flag::Kernel) || !ops.contains(&Flag::User))
        .filter(|ops| ops.contains(&Flag::AsFeatures) || !ops.contains(&Flag::AllStubsFeature))
    else {
        print_help();
        return ExitCode::FAILURE;
    };

    if options.contains(&Flag::Help) {
        print_help();
        return ExitCode::SUCCESS;
    }

    let mut db = VitaDb::load(&vita_headers_db_path());
    if options.contains(&Flag::Conflicting) {
        db = db.split_conflicting();
    } else if !options.contains(&Flag::WithConflicting) {
        db.split_conflicting();
    }

    if options.contains(&Flag::User) {
        db.split_kernel();
    } else if options.contains(&Flag::Kernel) {
        db = db.split_kernel();
    }

    let vitasdk_sys_manifest = vitasdk_sys_manifest();
    let stub_libs: BTreeSet<_> = {
        let mut missing_features_filter = options
            .contains(&Flag::MissingFeatures)
            .then(|| missing_features_filter(&vitasdk_sys_manifest));
        let mut missing_libs_filter = options
            .contains(&Flag::MissingLibs)
            .then(missing_libs_filter);

        db.stub_lib_names()
            .filter(|s| {
                missing_features_filter
                    .as_mut()
                    .map(|f| f(s))
                    .unwrap_or(true)
            })
            .filter(|s| missing_libs_filter.as_mut().map(|f| f(s)).unwrap_or(true))
            .collect()
    };

    {
        use std::io::{self, Write};

        let mut stdout = io::stdout().lock();
        if options.contains(&Flag::AsFeatures) {
            if options.contains(&Flag::AllStubsFeature) {
                struct DisplayAsSequence<T>(T);

                impl<T> std::fmt::Debug for DisplayAsSequence<T>
                where
                    T: IntoIterator + Copy,
                    T::Item: std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        // Strings are valid feature names, so debug representation only adds quotation marks
                        f.debug_list().entries(self.0).finish()
                    }
                }

                writeln!(stdout, "all-stubs = {:#?}", DisplayAsSequence(&stub_libs)).unwrap();
            }
            stub_libs
                .iter()
                .for_each(|stub_lib| writeln!(stdout, "{stub_lib} = []").unwrap());
        } else {
            stub_libs
                .iter()
                .for_each(|stub_lib| writeln!(stdout, "{stub_lib}").unwrap());
        }
    }

    if !stub_libs.is_empty() && options.contains(&Flag::FailIfAny) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn bindgen() -> ExitCode {
    let vitasdk_sys_manifest = vitasdk_sys_manifest();
    let vita_headers = vitasdk_sys_manifest.parent().unwrap().join("vita-headers");
    let output = vitasdk_sys_manifest
        .parent()
        .unwrap()
        .join("src")
        .join("bindings.rs");
    let is_build_rs = false;

    vitasdk_sys_build_util::bindgen::generate(
        &vita_headers.join("include"),
        &vita_headers.join("db"),
        &output,
        &vitasdk_sys_manifest,
        is_build_rs,
    );

    ExitCode::SUCCESS
}
