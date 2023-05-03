#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cpp/include/ro_mol.h");

        pub type ROMol;
        pub type ExplicitBitVect = crate::fingerprint_ffi::ExplicitBitVect;
        pub type SmilesParserParams;
        pub type Atom = crate::atom_ffi::Atom;

        pub fn copy_mol(mol: SharedPtr<ROMol>) -> SharedPtr<ROMol>;

        pub fn smiles_to_mol(smi: &CxxString) -> Result<SharedPtr<ROMol>>;

        pub fn smiles_to_mol_with_params(
            smi: &CxxString,
            params: SharedPtr<SmilesParserParams>,
        ) -> Result<SharedPtr<ROMol>>;
        pub fn new_smiles_parser_params() -> SharedPtr<SmilesParserParams>;
        pub fn smiles_parser_params_set_sanitize(
            ptr: SharedPtr<SmilesParserParams>,
            sanitize: bool,
        );

        pub fn mol_to_smiles(mol: SharedPtr<ROMol>) -> String;

        pub fn smarts_to_mol(sma: &CxxString) -> Result<SharedPtr<ROMol>>;

        // 0b11111111
        pub type MolSanitizeException;
        pub fn detect_chemistry_problems(mol: SharedPtr<ROMol>) -> UniquePtr<CxxVector<CxxString>>;
        // pub fn mol_sanitize_exception_type() -> String;

        pub fn get_num_atoms(mol: SharedPtr<ROMol>) -> u32;
        pub fn get_atom_with_idx(mol: SharedPtr<ROMol>, idx: usize) -> SharedPtr<Atom>;
    }
}

#[cfg(test)]
mod tests {
    use crate::ro_mol_ffi;

    #[test]
    fn test_default_parse_smarts_to_mol() {
        cxx::let_cxx_string!(sma = "[C:1](=[O:2])O.[N:3]");
        let romol = ro_mol_ffi::smarts_to_mol(&sma).unwrap();
        assert!(!romol.is_null());
    }
}