use rdkit_rs::ROMol;


fn main() {
    let mol = ROMol::from_smile("CCC").unwrap();
    println!("mol: {:#?}", mol);
}