use cxx::let_cxx_string;

#[test]
fn test_tautomer_enumerator() {
    let_cxx_string!(smile = "c1ccccc1C(=O)NC");
    let mol = rdk_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    let tautomer_enumerator = rdk_sys::mol_standardize_ffi::tautomer_enumerator();
    let tautomer_enumerator_result =
        rdk_sys::mol_standardize_ffi::tautomer_enumerate(tautomer_enumerator, mol);
    let size = rdk_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(
        tautomer_enumerator_result.clone(),
    );
    assert_eq!(size, 2);

    let first = rdk_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
        tautomer_enumerator_result.clone(),
        0,
    );
    let first_smile = rdk_sys::ro_mol_ffi::mol_to_smiles(first);
    assert_eq!("CN=C(O)c1ccccc1", first_smile);

    let second = rdk_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
        tautomer_enumerator_result,
        1,
    );
    let second_smile = rdk_sys::ro_mol_ffi::mol_to_smiles(second);
    assert_eq!("CNC(=O)c1ccccc1", second_smile);
}
