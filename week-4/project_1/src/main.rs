//Rust program to calculate speed of a car given distance and time
use std::io;

fn main() {
    let mut distance1 = String::new();
    let mut time1 = String::new();
     let mut distance2 = String::new();
    let mut time2 = String::new();


// first speed of a car 
    println!("Enter first distance:");
    io::stdin().read_line(&mut distance1).expect("Not a valid string" );
    let d1:f64 = distance1.trim().parse().expect("Not a valid number");

    
    println!("Enter first time:");
    io::stdin().read_line(&mut time1).expect("Not a valid string" );
    let t1:f64 = time1.trim().parse().expect("Not a valid number");
    
    let d1:f64 =  d1 * 1.60;
    let mut speed1:f64 = d1 / t1;
    println!("The first speed of a car given distance and time: {}",speed1);

    // second speed of a car
     println!("Enter second  distance:");
    io::stdin().read_line(&mut distance2).expect("Not a valid string" );
    let d2:f64 = distance2.trim().parse().expect("Not a valid number");
   
    println!("Enter second time:");
    io::stdin().read_line(&mut time2).expect("Not a valid string" );
    let t2:f64 = time2.trim().parse().expect("Not a valid number");
    
    let d2:f64 =  d2 * 1.60;
    let mut speed2:f64 = d2 / t2;
    println!("The  second speed of a car given distance and time: {}",speed2);

    

}
