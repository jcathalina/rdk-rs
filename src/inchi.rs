use cxx::let_cxx_string;
use rdk_sys::*;

pub fn inchi_to_inchi_key(inchi: &str) -> Result<String, cxx::Exception> {
    let_cxx_string!(inchi_cxx_string = inchi);
    let inchi_key = inchi_ffi::inchi_to_inchi_key(&inchi_cxx_string)?;
    Ok(inchi_key)
}