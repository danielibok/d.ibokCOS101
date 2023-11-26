// Rust program that can ask the number of siblings of a client.

use std::io;

fn main() {

    println!("\nEnter the Number of Siblings");
    let mut n_o_s = String::new();
    io::stdin().read_line(&mut n_o_s).expect("Failed to read input");
    let number_of_siblings:u16 = n_o_s.trim().parse().expect("Invalid number of siblings");

    for x in 1..=number_of_siblings {
    
    println!("\nEnter First Name of Sibling");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to read input");
    
    println!("\nEnter Age of Sibling");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age_of_sibling:i64 = age.trim().parse().expect("Invalid age");

    if age_of_sibling >= 18 {
        println!("\nEnter Marital Status: 'Single' or 'Married' ");
        let mut marital_status = String::new();
        io::stdin().read_line(&mut marital_status).expect("Failed to read input");
        let marital_status:String = marital_status.trim().parse().expect("Invalid marital status");
       

        if marital_status == "Single" {
        println!("\nEnter Employment Status: 'Student' or 'Worker' ");
        let mut employment_status = String::new();
        io::stdin().read_line(&mut employment_status).expect("Invalid input");
        let employment_status:String = employment_status.trim().parse().expect("Invalid employment status"); 
        
               if employment_status == "Student" {
                println!("\nEnter Name of University:");
                let mut uni = String::new();
                io::stdin().read_line(&mut uni).expect("Invalid input");

                 println!("\nEnter Course of Study");
                 let mut course = String::new();
                  io::stdin().read_line(&mut course).expect("Invalid input");
                }else{
                     println!("Mad o");
                 }
            }

    if marital_status == "Married" {   
        println!("\nEnter whether Sibling has: 'Children' or 'No Children'");
        let mut cs = String::new();
        io::stdin().read_line(&mut cs).expect("Invalid input");
        let children_status:String = cs.trim().parse().expect("Invalid Children Status");

        println!("\nEnter City where Family Resides:");
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Invalid input");
        let city_of_residence:String = city.trim().parse().expect("Invalid City of Residence"); 
        }
    }

     if age_of_sibling <= 18 {
        println!("\nEnter Siblings WAEC status : 'yes' or 'no'");
        let mut waec_status = String::new();
        io::stdin().read_line(&mut waec_status).expect("Invalid input");
        let waec_status:String = waec_status.trim().parse().expect("Invalid WAEC Status");
   
        if waec_status == "yes"{
        println!("\nEnter Secondary School he/she attended:");
        let mut sschool = String::new();
        io::stdin().read_line(&mut sschool).expect("Invalid input");
        let sschool:String = sschool.trim().parse().expect("Invalid Secondary School"); 
   

        }else {
        println!("\nEnter Current Class level:");
        let mut class = String::new();
        io::stdin().read_line(&mut class).expect("Invalid input");
        let class:String = class.trim().parse().expect("Invalid class level"); 
         
          }
        }
    }
  }
