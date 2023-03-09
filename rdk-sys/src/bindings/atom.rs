#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("cpp/include/ro_mol.h");
        include!("cpp/include/atom.h");

        pub type Atom;
        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn atom_from_symbol(symbol: &CxxString) -> Result<SharedPtr<Atom>>;
        pub fn get_symbol(atom: SharedPtr<Atom>) -> String;
    }
}
