//Rust program to find out who has the highest years of programming experience as a Developer through an Interview.
use std::io;

fn main() {
    let mut input = String::new();
    println!("\nEnter the number of people to be interviewed");
    io::stdin().read_line(&mut input).expect("Invalid number of users");
    let number_of_interviews:i64 = input.trim().parse().expect("Invalid number of users");
    for x in 0..=number_of_interviews{

    let mut input1 = String::new();
    println!("\nWhat is your name?");
    io::stdin().read_line(&mut input1).expect("Failed input");
    let name:String = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("\nEnter your age");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("\nAre you male or female?");
    io::stdin().read_line(&mut input3).expect("Failed input");
    let gender:String = input3.trim().parse().expect("Invalid input");

    let mut input4 = String::new();
    println!("\nEnter your email address");
    io::stdin().read_line(&mut input4).expect("Failed input");
    let email_address:String = input4.trim().parse().expect("Invalid input");

    let input5 = String::new();
    println!("\nEnter years of programming experience"); 
    io::stdin().read_line(&mut input4).expect("Failed input");
    let email_address:String = input4.trim().parse().expect("Invalid input");
}
}
