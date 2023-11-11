// Rust program to find the roots of a quadratuc equation
// given values a , b and c 

use std::io;

fn main() 
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f64 = a.trim().parse().expect("Not a valid number");
    
    println!("Enter b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f64 = b.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f64 = c.trim().parse().expect("Not a valid number");

    //The Discriminant
    let d:f64 = b.powf(2.0) - 4.0 * a * c;
    println!("The Discriminant: {}", d);

if d > 0.0 {
     // Two distinct real roots
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a );
    println!("The roots are real and distinct:");
    println!("Root 1: {}", root1);
    println!("Root 2: {}", root2);
} else if d == 0.0  {
    // Exactly one real root
    let root = -b / (2.0 * a );
    println!("The root is real and equal:");
    println!("Root: {}", root);
} else { 
    // No real roots 
    println!("There are no real roots.");
       }
}




