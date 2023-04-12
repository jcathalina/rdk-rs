use rdk::{Atom, ROMol, inchi_to_inchi_key, inchi_to_mol};

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

    let inchi = mol.as_inchi();
    println!("inchi: {}", inchi);

    let key = inchi_to_inchi_key(&inchi).unwrap();
    println!("key: {}", key);

    let mol_from_inchi = inchi_to_mol(&inchi).unwrap();
    println!("mol_from_inchi: {:?}", mol_from_inchi);
}
