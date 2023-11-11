// Rust program to display the menu of food items available to take order for food items available to take order from the customer..

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Enter quantity of Poundo Yam and Edinkaiko Soup:");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let pounded_yam_and_edinkaiko_soup:f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter quantity of Fried Rice and Chicken:");
    io::stdin().read_line(&mut input2).expect("Invalid string");
    let fried_rice_and_chicken:f64 = input2.trim().parse().expect("Invalid input");
    
    println!("Enter quantity of Amala and Ewedu Soup:");
    io::stdin().read_line(&mut input3).expect("Invalid string");
    let amala_and_ewedu_soup:f64 = input3.trim().parse().expect("Invalid input");
    
    println!("Enter quantity of Eba and Egusi Soup:");
    io::stdin().read_line(&mut input4).expect("Invalid string");
    let eba_and_egusi_soup:f64 = input4.trim().parse().expect("Invalid input");

 
    println!("Enter quantity of White Rice and Stew:");
    io::stdin().read_line(&mut input5).expect("Invalid string");
    let white_rice_and_stew:f64 = input5.trim().parse().expect("Invalid input");

// Price of food items 
    let p:f64 = pounded_yam_and_edinkaiko_soup * 3_200.00;
    let f:f64 = fried_rice_and_chicken * 3_000.00;
    let a:f64 = amala_and_ewedu_soup * 2_500.00;
    let e:f64 = eba_and_egusi_soup * 2_000.00;
    let w:f64 = white_rice_and_stew * 2_500.00;

// Total Price (Charges) of food items 
    let tc:f64 =  p + f + a + e + w ;
    println!("The Total Charges is N{} : ", tc  );

// The Discount
    let d:f64 = tc * 0.05 ;
    println!("The Discount is N{} : ", d );

if tc > 10_000.00 
{
    println!("The Discount is N{} : ", d );
}
else 
{
    println!("The Final Charge = Total Charges : ");
}

// The Final Charge
let fc:f64 = tc - d;
println!("The Final Charge is N{}", fc);
}
