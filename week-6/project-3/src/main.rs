//Rust program to display the Multiplication Table of 1 to 15.

use std::io;
fn main() {
    for x in 0..15{
    println!("\nEnter the number");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Invalid input");
    let n:i32 = number.trim().parse().expect("Invalid input");

    let multiplication1:i32 = n * 1;
    let multiplication2:i32 = n * 2;
    let multiplication3:i32 = n * 3;
    let multiplication4:i32 = n * 4;
    let multiplication5:i32 = n * 5;
    let multiplication6:i32 = n * 6;
    let multiplication7:i32 = n * 7;
    let multiplication8:i32 = n * 8;
    let multiplication9:i32 = n * 9;
    let multiplication10:i32 = n * 10;
    let multiplication11:i32 = n * 11;
    let multiplication12:i32 = n * 12;

    println!("The Multiplication Table of 1 to n");
    println!("{}",multiplication1);
    println!("{}",multiplication2);
    println!("{}",multiplication3);
    println!("{}",multiplication4);
    println!("{}",multiplication5);
    println!("{}",multiplication6);
    println!("{}",multiplication7);
    println!("{}",multiplication8);
    println!("{}",multiplication9);
    println!("{}",multiplication10);
    println!("{}",multiplication11);
    println!("{}",multiplication12); 
  }
}
