use rdk_rs::{ROMol, Atom};

fn main() {
    let mol = ROMol::from_smile("CCC").unwrap();
    let n_atoms = mol.get_num_atoms();
    println!("n_atoms: {}", n_atoms);

    let atom = Atom::from_symbol("C").unwrap();
    println!("atom: {:?}", atom);

    // println!("atom: {:?}", atom); // TODO: Implement atom class in its own file.
}