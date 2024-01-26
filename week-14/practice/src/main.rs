//Globacom ltd
use std::io::Read;
fn main() {
    let mut file = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",globacom);
    println!("Hello, world");
}
//Rust program to show the structure of the globacom database schemas \
// of Globacom Ltd. 
use std::io;
use std::io::Read;