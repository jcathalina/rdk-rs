#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cpp/include/inchi.h");
        include!("cpp/include/ro_mol.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn mol_to_inchi(mol: SharedPtr<ROMol>) -> String;
        pub fn inchi_to_mol(inchi: &CxxString, sanitize: bool, remove_hydrogens: bool) -> Result<SharedPtr<ROMol>>;
        pub fn inchi_to_inchi_key(inchi: &CxxString) -> Result<String>;
    }
}

#[test]
fn test_mol_to_inchi() {
    cxx::let_cxx_string!(smile = "C");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    let inchi = rdkit_sys::inchi_ffi::mol_to_inchi(romol);
    assert_eq!(inchi, "InChI=1S/CH4/h1H4");
}

#[test]
fn test_good_inchi_to_mol() {
    cxx::let_cxx_string!(inchi = "InChI=1S/C2H6/c1-2/h1-2H3");
    let romol = rdkit_sys::inchi_ffi::inchi_to_mol(&inchi).unwrap();
    assert!(!romol.is_null());
    assert_eq!(rdkit_sys::inchi_ffi::mol_to_inchi(romol.clone()), "InChI=1S/C2H6/c1-2/h1-2H3");
    assert_eq!(rdkit_sys::ro_mol_ffi::mol_to_smiles(romol), "CC");
}

#[test]
fn test_bad_inchi_to_mol() {
    cxx::let_cxx_string!(bad_inchi = "asd");
    let romol = rdkit_sys::inchi_ffi::inchi_to_mol(&bad_inchi);
    assert!(romol.is_ok());
    assert!(romol.unwrap().is_null());
}

#[test]
fn test_good_inchi_to_inchi_key() {
    cxx::let_cxx_string!(inchi = "InChI=1S/CH4/h1H4");
    let inchi_key = rdkit_sys::inchi_ffi::inchi_to_inchi_key(&inchi).unwrap();
    assert_eq!(inchi_key, "VNWKTOKETHGBQD-UHFFFAOYSA-N");
}

#[test]
fn test_bad_inchi_to_inchi_key() {
    cxx::let_cxx_string!(bad_inchi = "asd");
    let inchi_key = rdkit_sys::inchi_ffi::inchi_to_inchi_key(&bad_inchi);
    assert!(inchi_key.is_ok());
    assert!(inchi_key.unwrap().is_empty());
}