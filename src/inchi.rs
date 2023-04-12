use cxx::let_cxx_string;
use rdk_sys::*;
use crate::ROMol;

pub fn inchi_to_inchi_key(inchi: &str) -> Result<String, cxx::Exception> {
    let_cxx_string!(inchi_cxx_string = inchi);
    let inchi_key = inchi_ffi::inchi_to_inchi_key(&inchi_cxx_string)?;
    Ok(inchi_key)
}

pub fn inchi_to_mol(inchi: &str) -> Result<ROMol, cxx::Exception> {
    let_cxx_string!(inchi_cxx_string = inchi);
    let ptr = inchi_ffi::inchi_to_mol(&inchi_cxx_string, true, true)?;
    Ok(ROMol { ptr })
}