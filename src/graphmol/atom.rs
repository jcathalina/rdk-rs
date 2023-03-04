use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use rdk_sys::*;

pub struct Atom {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::Atom>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AtomError {
    #[error("could not convert symbol to atom (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smile to romol (exception)")]
    ConversionException(String)
}

impl Atom {
    pub fn from_symbol(symbol: &str) -> Result<Self, AtomError> {
        let_cxx_string!(sym_cxx_string = symbol);
        let ptr = atom_ffi::atom_from_symbol(&sym_cxx_string);
        match ptr {
            Ok(ptr) => {
                if ptr.is_null() {
                    Err(AtomError::UnknownConversionError)
                } else {
                    Ok(Atom { ptr })
                }
            },
            Err(e) => {
                Err(AtomError::ConversionException(e.to_string()))
            }
        }
    }

    pub fn to_symbol(&self) -> String {
        atom_ffi::get_symbol(self.ptr.clone())
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.to_symbol();
        f.debug_tuple("Atom").field(&symbol).finish()
    }
}