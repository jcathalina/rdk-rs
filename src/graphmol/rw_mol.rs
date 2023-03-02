use std::fmt::Formatter;

use cxx::{let_cxx_string, SharedPtr};
use rdk_sys::*;

use crate::ROMol;

pub struct RWMol {
    pub(crate) ptr: SharedPtr<rdk_sys::rw_mol_ffi::RWMol>,
}

impl RWMol {
    pub fn from_mol_block(
        mol_block: &str,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Option<Self> {
        let_cxx_string!(mol_block = mol_block);

        let ptr = rdk_sys::rw_mol_ffi::rw_mol_from_mol_block(
            &mol_block,
            sanitize,
            remove_hs,
            strict_parsing,
        );

        if ptr.is_null() {
            None
        } else {
            Some(RWMol { ptr })
        }
    }

    pub fn as_smile(&self) -> String {
        let cast_ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdk_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdk_sys::ro_mol_ffi::ROMol>,
            >(self.ptr.clone())
        };
        ro_mol_ffi::mol_to_smiles(cast_ptr)
    }

    pub fn to_ro_mol(self) -> ROMol {
        let ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdk_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdk_sys::ro_mol_ffi::ROMol>,
            >(self.ptr)
        };
        ROMol { ptr }
    }
}

impl Clone for RWMol {
    fn clone(&self) -> Self {
        let ptr = rw_mol_ffi::rw_mol_from_rw_mol(self.ptr.clone());
        RWMol { ptr }
    }
}

impl std::fmt::Debug for RWMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smile = self.as_smile();
        f.debug_tuple("RWMol").field(&smile).finish()
    }
}
