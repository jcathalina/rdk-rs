const CPP_VERSION_FLAG: &'static str = "-std=c++17";

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    env_logger::init();

    let mut lib_paths = vec![];
    let mut include_paths = vec![];

    // prefer the prefix env var, if not, fall back to the base from the CLI
    match std::env::var("CONDA_PREFIX") {
        Ok(prefix) => {
            include_paths.push(format!("{prefix}/include"));
            include_paths.push(format!("{prefix}/include/rdkit"));
            lib_paths.push(format!("{prefix}/lib"));
        }
        Err(_) => {
            let conda = which::which("conda")
                .map(|p| p.to_str().unwrap().to_string())
                .unwrap_or_else(|_| panic!("conda not found"));
            let mut conda = std::process::Command::new(conda);
            conda.args(&["info", "--base"]);

            let output = conda.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();
            let conda_root = stdout.trim().to_string();

            lib_paths.push(format!("{}/lib", conda_root));
            include_paths.push(format!("{}/include", conda_root));
            include_paths.push(format!("{}/include/rdkit", conda_root));
        }
    }

    let dir = std::fs::read_dir("src/bindings").unwrap();
    let rust_files = dir
        .into_iter()
        .filter_map(|p| match p {
            Ok(p) => {
                if p.metadata().unwrap().is_file() {
                    Some(p.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .filter(|p| !p.ends_with("mod.rs"))
        .collect::<Vec<_>>();

    let mut wrapper_cc_paths = vec![];

    let wrapper_root = std::path::PathBuf::from("cpp");
    for file in &rust_files {
        let file_name = file.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        let base_name = &file_name[0..file_name.len() - 3];

        let cc_path = wrapper_root.join("src").join(format!("{}.cc", base_name));
        let meta = std::fs::metadata(&cc_path).unwrap();
        if !meta.is_file() {
            panic!("{} must exist", cc_path.display())
        }
        wrapper_cc_paths.push(cc_path);

        let h_path = wrapper_root
            .join("include")
            .join(format!("{}.h", base_name));
        let meta = std::fs::metadata(&h_path).unwrap();
        if !meta.is_file() {
            panic!("{} must exist", h_path.display())
        }
    }

    cxx_build::bridges(rust_files)
        .files(wrapper_cc_paths)
        .includes(include_paths)
        .include(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .flag(CPP_VERSION_FLAG)
        .warnings(false)
        // rdkit has warnings that blow up our build. we could enumerate all those warnings and tell
        // the compiler to allow them... .warnings_into_errors(true)
        .compile("rdkit");

    for path in lib_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }

    for lib in &[
        "Catalogs",
        "ChemReactions",
        "ChemTransforms",
        "coordgen",
        "DataStructs",
        "Depictor",
        "Descriptors",
        "FileParsers",
        "Fingerprints",
        "GenericGroups",
        "GraphMol",
        "MolStandardize",
        "MolTransforms",
        "PartialCharges",
        "RDGeneral",
        "RDGeometryLib",
        "RingDecomposerLib",
        "SmilesParse",
        "Subgraphs",
        "SubstructMatch",
    ] {
        println!("cargo:rustc-link-lib=dylib=RDKit{}", lib);
    }

    println!("cargo:rustc-link-lib=dylib=boost_serialization");
}
