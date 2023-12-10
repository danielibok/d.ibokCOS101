//Rust Program to crate a file that saves the high-quality categories of drinks as indicated in the table. 
use std::io::Write; 
fn main() {
    let lager = "\nLager:\n33 Export\nDesperados\nGoldberg\nGulder\nHeneiken\nStar\n";
     let stout = "\nStout:\nLegend\nTurbo King\nWilliams\n";
     let non_alcoholic = "\nNon-Alcoholic:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";
    
    let mut file = std::fs::File::create("Nigerian Brewery Ltd.txt").expect("Failed to create file");
    
    file.write_all("NIgerian Brewery Limited's high quality drinks:\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    println!("\nData written to file. ");
}
