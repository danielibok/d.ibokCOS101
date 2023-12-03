//Rust program to build A Public Service APS checker that validates Staff level.

use std::io; 
fn main() {
    let mut input = String::new();
    println!("\nEnter the number of people to use this system");
    io::stdin().read_line(&mut input).expect("Invalid number of users");
    let number_of_users:i64 = input.trim().parse().expect("Invalid number of users");
    for x in 0..=number_of_users{

    let v = vec![" Office Administrator", "Academic", "Lawyer", "Teacher"];
    let mut choice = String::new();
    println!("\nSelect one of the following careers: 0.Office Administrator, 1.Academic, 2.Lawyer, 3.Teacher");
    io::stdin().read_line(&mut choice).expect("Faoled tp read input");
    let index:String = choice.trim().parse().expect("Invalid input");
    
    if index == "0"
    { 
    let mut choice1 = String::new();
    println!("\nSelect your jurisdiction under Office Administration: 0.Intern , 1.Administrator, 2.Senior Administrator, 3.Office manager, 4.Director, 5.CEO" );
    io::stdin().read_line(&mut choice1).expect("Failed input");
    let index1:String = choice1.trim().parse().expect("Invalid input"); 

    if index1 == "0"
    {
       println!("You are an intern with positon APS 1-2 and you have 1-2years of experience");
    }
    if index1 == "1"
    {
       println!("You are an Administrator with position APS 3-5 and you have 3-5years of experience");
    }
    if index1 == "2"
    {
       println!("You are a Senior Administraator with position APS 5-8 and you have 5-8years of experience");
    }
    if index1 == "3"
    {
       println!("You are an Office Manager with position APS EL1 8-10 and you have 8-10years of experience");
    }
    if index1 == "4"
    {
       println!("You are a Director with position EL2 10-13 and you have 10-13 years of experience");
    }
    if index1 == "5"
    {
       println!("You are a CEO with position SES and you have over 13 years of experience");
    }
    }


   if index == "1"
   {
    let mut choice2 = String::new();
    println!("\nSelect your jurisdiction under Academics: 0.-- , 1.Research Assistant, 2.PhD Candidate, 3.Post-Doc Researcher, 4.Senior Lecturer, 5.Dean");
    io::stdin().read_line(&mut choice2).expect("Failed input");
    let index2:String = choice2.trim().parse().expect("Invalid input"); 
   
   if index2 == "0"
   {
      println!("You have a position of APS 1-2 and 1-2 years of experience");
   }
   if index2 == "1"
   {
       println!("You are a Research Assistant with position APS 3-5 and you have 3-5years of experience");
   }
   if index2 == "2"
   {
       println!("You are a PhD Candidate with position APS 5-8 and you have 5-8years of experience");
   }
   if index2 == "3"
   {
       println!("You are a Post Doc Researcher with position EL1 8-10 and you have 8-10years of experience");
   }
   if index2 == "4"
   {
       println!("You are a Research Assistant with position EL2 10-13 and you have 10-13years of experience");
   }
   if index2 == "5"
   {
       println!("You are a Dean with position SES and you have over 13years of experience");
   }
}

    if index == "2"
   {
    println!("\nSelect your jurisdiction as a Lawyer: 0.Paralegal, 1.Junior Associate, 2.Associate 3.Senior Associate 1-2, 4.Senior Associate 3-4, 5.Partner");
    let mut choice3 = String::new();
    io::stdin().read_line(&mut choice3).expect("Failed input");
    let index3:String = choice3.trim().parse().expect("Invalid input"); 

   if index3 == "0"
   {
   println!("You are a Paralegal Lawyer with a positon APS 1-2 and 1-2 years of experience");
   } 
   if index3 == "1"
   {
       println!("You are a Junior Associate Lawyer with position APS 3-5 and you have 3-5years of experience");
   }
   if index3 == "2"
   {
       println!("You are an Associate the position APS 5-8 and you have 5-8years of experience");
   }
   if index3 == "3"
   {
       println!("You are a Senior Associate 1-2 with position EL1 8-10 and you have 8-10years of experience");
   }
   if index3 == "4"
   {
       println!("You are a Senior Associate 3-4 with position EL2 10-13 and you have 10-13years of experience");
   }
   if index3 == "5"
   {
       println!("You are a Partner with position SES and you have over 13years of experience");
   }
}

   if index == "3"
   {
    println!("\nSelect your jurisdiction as a Lawyer: 0.Placement, 1.Classroom Teacher , 2.Shr Teacher, 3.Leading Teacher, 4.Deputy Principal, 5.Principal");
    let mut choice4 = String::new();
    io::stdin().read_line(&mut choice4).expect("Failed input");
    let index4:String = choice4.trim().parse().expect("Invalid input"); 

   if index4 == "0"
   {
    println!("You are a Placement Teacher with a position APS 1-2 and 1-2 years of experience");
   }

   if index4 == "0"
   {
      println!("You have a position of APS 1-2 and 1-2 years of experience");
   }
   if index4 == "1"
   {
       println!("You are a Classroom Teacher with position APS 3-5 and you have 3-5years of experience");
   }
   if index4 == "2"
   {
       println!("You are a Shr Teacher with position APS 5-8 and you have 5-8years of experience");
   }
   if index4 == "3"
   {
       println!("You are a Leading Teacher with position EL1 8-10 and you have 8-10years of experience");
   }
   if index4 == "4"
   {
       println!("You are a Deputy Principal with position EL2 10-13 and you have 10-13years of experience");
   }
   if index4 == "5"
   {
       println!("You are a Principa; with position SES and you have over 13years of experience");
   }
}
  }   
}
