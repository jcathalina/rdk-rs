use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use rdk_sys::*;

use crate::{Atom, Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::ROMol>,
}

pub struct ROMolAtomIterator<'a> {
    ro_mol: &'a ROMol,
    curr: usize,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ROMolError {
    #[error("could not convert smile to romol (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smile to romol (exception)")]
    ConversionException(String),
}

impl ROMol {
    pub fn from_smile(smile: &str) -> Result<Self, ROMolError> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol(&smile_cxx_string);
        match ptr {
            Ok(ptr) => {
                if ptr.is_null() {
                    Err(ROMolError::UnknownConversionError)
                } else {
                    Ok(ROMol { ptr })
                }
            }
            Err(e) => Err(ROMolError::ConversionException(e.to_string())),
        }
    }

    pub fn from_smile_with_params(
        smile: &str,
        params: &SmilesParserParams,
    ) -> Result<Self, cxx::Exception> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol_with_params(&smile_cxx_string, params.ptr.clone())?;
        Ok(Self { ptr })
    }

    pub fn as_smile(&self) -> String {
        ro_mol_ffi::mol_to_smiles(self.ptr.clone())
    }

    pub fn as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> RWMol {
        let ptr = rdk_sys::rw_mol_ffi::rw_mol_from_ro_mol(self.ptr.clone(), quick_copy, conf_id);
        RWMol { ptr }
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let ptr = fingerprint_ffi::fingerprint_mol(self.ptr.clone());
        Fingerprint::new(ptr)
    }

    pub fn get_num_atoms(&self) -> u32 {
        ro_mol_ffi::get_num_atoms(self.ptr.clone())
    }

    pub fn get_atom(&self, idx: usize) -> Atom {
        let ptr = ro_mol_ffi::get_atom_with_idx(self.ptr.clone(), idx);
        Atom { ptr }
    }

    pub fn iter(&self) -> ROMolAtomIterator {
        ROMolAtomIterator {
            ro_mol: self,
            curr: 0,
        }
    }

    pub fn as_inchi(&self) -> String {
        inchi_ffi::mol_to_inchi(self.ptr.clone())
    }
}

impl Debug for ROMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smile = self.as_smile();
        f.debug_tuple("ROMol").field(&smile).finish()
    }
}

impl Clone for ROMol {
    fn clone(&self) -> Self {
        ROMol {
            ptr: rdk_sys::ro_mol_ffi::copy_mol(self.ptr.clone()),
        }
    }
}

impl<'a> Iterator for ROMolAtomIterator<'a> {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        let num_atoms = self.ro_mol.get_num_atoms() as usize;
        if self.curr < num_atoms {
            let curr_atom = Some(self.ro_mol.get_atom(self.curr));
            self.curr += 1;
            curr_atom
        } else {
            None
        }
    }
}

pub struct SmilesParserParams {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::SmilesParserParams>,
}

impl SmilesParserParams {
    pub fn sanitize(&mut self, value: bool) {
        rdk_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(self.ptr.clone(), value);
    }
}

impl Default for SmilesParserParams {
    fn default() -> Self {
        SmilesParserParams {
            ptr: rdk_sys::ro_mol_ffi::new_smiles_parser_params(),
        }
    }
}
