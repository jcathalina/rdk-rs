use rdk::{Atom, ROMol};

fn main() {
    let mol = ROMol::from_smile("CCC").unwrap();
    let n_atoms = mol.get_num_atoms();
    println!("n_atoms: {}", n_atoms);

    println!("mol: {:?}", mol);

    let atom = Atom::from_symbol("Br").unwrap();
    println!("atom: {:?}", atom);

    // println!("atom: {:?}", atom); // TODO: Implement atom class in its own file.
    let atom_iter = ROMol::iter(&mol);
    for a in atom_iter {
        println!("{:?}", a);
    }

    let atom_iter2 = mol.iter();
    for a in atom_iter2 {
        println!("{:?}", a);
    }
}
