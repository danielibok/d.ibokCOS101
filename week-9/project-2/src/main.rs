//Rust program that reads personal details of the student from an array or vector,then displays the details of the students from a vector.
use std::io::Write;
fn main() {
    println!("PAU SIMS:");
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shamia Bolade","Adekunle Gold","Bianca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec!["300","100","200","200","100"];
    
    let mut file = std::fs::File::create("PAU SIMS.txt").expect("Failed to create file");

    for i in 0..student_name.len(){
        println!("\nStudent Name:{}\nMatric Number:{}\nDepartment:{}\nLevel:{}\n",student_name[i],matric_number[i],department[i],level[i]);

        file.write_all(format!("{}\t",student_name[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\t",matric_number[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\t",department[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\t",level[i]).as_bytes()).expect("Failed");

        println!("Welcome!");

    } 
}
