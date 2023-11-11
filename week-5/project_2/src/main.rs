// Rust program to determine the annual incentive taking the
// the age and experience of an employee

use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("\nenter experience type 'experienced' or 'inexperienced': ");
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    experience = experience.trim().parse().expect("Invalid input");

    println!("\nEnter employees age :");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:f32 = age.trim().parse().expect("Not a valid number");

    if experience == "experienced" && age >= 40.00
    {
        println!("annual incentive is 1,560,000");
    } 
    else if experience == "experienced" &&  age >= 30.00 && age <= 40.00
    {
        println!("annual incentive is 1,480,000");
    }
    else if experience == "experienced" && age < 28.00
    {
        println!("annual incentive is 1,300,000");
    }
    else if experience == "inexperienced"  
    {  
        println!("annual incentive is 100,000");
    }
}
