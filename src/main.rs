use rdk_rs::{ROMol};

fn main() {
    let mol = ROMol::from_smile("CCC").unwrap();
    let n_atoms = mol.get_num_atoms();
    println!("n_atoms: {}", n_atoms);

    let atom = mol.get_atom_with_idx(0u32);
    // println!("atom: {:?}", atom); // TODO: Implement atom class in its own file.
}