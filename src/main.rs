use rdk_rs::ROMol;


fn main() {
    let mol = ROMol::from_smile("CCC").unwrap();
    println!("mol: {:#?}", mol);
}