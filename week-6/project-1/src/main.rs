// Rust program that checks the eligibility of candidates to vote.

use std::io;

fn main() {
    for x in 0..150{
    println!("Student Council Votimg System.");
   
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    
    // input candidates status
    println!("\nEnter candidate status 'class rep' or 'not a class rep' : ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let candidates_status:String = input1.trim().parse().expect("Invalid status");
    
    // input candidates level
    println!("\nEnter candidates level:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _candidates_level:i32 = input2.trim().parse().expect("Invalid candidates level");
     
    // input candidates CGPA
    println!("\nEnter candidates CGPA  ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let cgpa:f64 = input4.trim().parse().expect("Invalid input");

     // input Name of candidate
    println!("\nPlease Enter your name.");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    // input email address 
    println!("\nEnter email address");
    let mut email_address = String::new();
    io::stdin().read_line(&mut email_address).expect("Invalid email address.");
    
    // input candidates department 
    println!("\nEnter your Departmnent.");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input.");
    let department:String = department.trim().parse().expect("Invalid department");

    // input candidates state of origin
    println!("\nEnter your State of Origin.");
    let mut state_of_origin = String::new();
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read input.");
    let state_of_origin:String = state_of_origin.trim().parse().expect("Invalid  State of Origin");

    if  candidates_status == "class rep" &&  _candidates_level > 100  && cgpa >= 4.0 
{
    println!("Candidate Information:");
    println!("{}",name);
    println!("{}",email_address); 
    println!("{}",department);
    println!("{}",state_of_origin);
    println!("You can vote");
}
   else 
{
    println!("Sorry, you are not eligible to vote");
}   

   println!("count {}",x );
  }
}
