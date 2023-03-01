#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rdkit-lib/include/ro_mol.h");
        include!("rdkit-lib/include/rw_mol.h");

        pub type RWMol;
        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn rw_mol_from_mol_block(
            mol_block: &CxxString,
            sanitize: bool,
            remove_hs: bool,
            strict_parsing: bool,
        ) -> SharedPtr<RWMol>;

        pub fn rw_mol_from_ro_mol(
            mol: SharedPtr<ROMol>,
            quick_copy: bool,
            conf_id: i32,
        ) -> SharedPtr<RWMol>;

        pub fn rw_mol_from_rw_mol(mol: SharedPtr<RWMol>) -> SharedPtr<RWMol>;
    }
}