//Rust program to calculate the area of a trapezium, rhombus, parallelogram, cube and cylinder.

use std::io;
fn main() {

    println!("Select any number for the shapes below");
    
    println!("1.Trapezium");
    println!("2.Rhombus");
    println!("3.Parallelogram");
    println!("4.Cube");
    println!("5.Cylinder");
    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("Failed to read input");
    let shape:String = shape.trim().parse().expect("Invalid shape number");
    
    if shape == "1"
    {      
    //Function to calculate the area of a trapezium.
     println!("Enter the height in cm:");
     let mut height = String::new();
     io::stdin().read_line(&mut height).expect("Failed to read input");
     let height:f64 = height.trim().parse().expect("Invalid height");
    
     println!("Enter the first base in cm");
     let mut base1 = String::new();
     io::stdin().read_line(&mut base1).expect("failed to read input");
     let base1:f64 = base1.trim().parse().expect("Invalid base");
    
     println!("Enter second base");
     let mut base2 = String::new();
     io::stdin().read_line(&mut base2).expect("Falied to read input");
     let base2:f64 = base2.trim().parse().expect("Invalid base");

     let area1 = trapezium_area(height, base1, base2);
     println!("Area of Trapezium = {}cm^2", area1);
    }
   
   else if shape == "2"
   {
   //Function to calculate the area of a Rhombus.
     println!("Enter diagonal one in cm");
     let mut diagonal1 = String::new();
     io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
     let diagonal1:f64 = diagonal1.trim().parse().expect("Invalid diagonal");

     println!("Enter diagonal two in cm");
     let mut diagonal2 = String::new();
     io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
     let diagonal2:f64 = diagonal2.trim().parse().expect("Invalid diagonal");

     let area2 = rhombus_area(diagonal1, diagonal2);
     println!("Area of Rhombus = {}cm^2", area2);
   }

   else if shape == "3"
   {
   //Function to calculate the area of a Parallelogram. 
     println!("Enter base in cm");
     let mut base = String::new();
     io::stdin().read_line(&mut base).expect("Failed to read input");
     let base:f64 = base.trim().parse().expect("Invalid base");
   
     println!("Enter altitude in cm");
     let mut altitude = String::new();
     io::stdin().read_line(&mut altitude).expect("Failed to read input");
     let altitude:f64 = altitude.trim().parse().expect("Invalid altitude");

     let area3 = parallelogram_area(base, altitude);
     println!("Area of Parallelogram = {}cm^2",area3);
   }
   else if shape == "4"
   {
   //Function to calculate the area of a Cube.
     println!("Enter length of the side in cm");
     let mut length = String::new();
     io::stdin().read_line(&mut length).expect("Failed to read input");
     let length:f64 = length.trim().parse().expect("Invalid lenght");

     let area4 = cube_area(length);
     println!("Area of Cube = {}cm^3",area4);
   }
   else if shape =="5"
   {
   //Function to calculate the volume of a Cylinder.
     println!("Enter radius of cylinder in cm");
     let mut radius = String::new();
     io::stdin().read_line(&mut radius).expect("Failed to read input");
     let radius:f64 = radius.trim().parse().expect("Invalid radius");

     println!("Enter height of cylinder in cm");
     let mut height = String::new();
     io::stdin().read_line(&mut height).expect("Failed to read input");
     let height:f64 =height.trim().parse().expect("Invalid height");
     let pi:f64 = 3.142;
     let area5 = cylinder_volume(radius, height, pi);
     println!("Enter Volume Cylinder ={}cm^3",area5);
    }
        }


fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    let area1 = height / 2.0 * (base1 + base2);
    area1
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    let area2 = 1.0 / 2.0 * (diagonal1 * diagonal2);
    area2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    let area3 = base * altitude;
    area3
}

fn cube_area(length: f64, ) -> f64 {
    let area4 = 6.0 * (length).powf(2.0);
    area4
}

fn cylinder_volume(radius: f64, height: f64, pi: f64) -> f64 {
    let area5 = pi * radius.powf(2.0) * height;
    area5
}