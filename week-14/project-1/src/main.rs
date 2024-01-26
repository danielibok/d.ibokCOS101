//Rust program to show the structure of the globacom database schemas \
// of Globacom Ltd. 
use std::io;
use std::io::Read;
fn main(){
    println!("\nGLOBACOM LTD DATABASE MANAGEMENT SYSTEM");
    println!("\nEnter your status: Administrator, Project Manager, Employee, Customer, Vendor");
    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed input");
    let status:String = status.trim().parse().expect("Failed");
    
    if 
    status == "Administrator" {
        administrator();
    }  

    if status == "Project Manager"{
        project_manager();
    }

    if status == "Employee"{
        employee();
    }

    if status == "Customer"{
        customer();
    }

    if status == "Vendor"{
        vendor();
    }
    else {
        println!("Ooops it seems you have inputed an invalid status type, the status type should be any of these:");
        println!("Administrator, Project Manager, Employee, Customer,Vendor");
    }
}
fn administrator(){
    let mut file_admin = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\globacom_dbase.sql").unwrap();
    let mut contents_admin = String::new();
    file_admin.read_to_string(&mut contents_admin).unwrap();
    print!("{}",contents_admin);
}

fn project_manager(){
    let mut file_project_manager = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\project_tb.sql").unwrap();
    let mut contents_project_manager = String::new();
    file_project_manager.read_to_string(&mut contents_project_manager).unwrap();
    print!("{}",contents_project_manager);
}

fn employee(){
    let mut file_employee = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\staff_tb.sql").unwrap();
    let mut contents_employee = String::new();
    file_employee.read_to_string(&mut contents_employee).unwrap();
    print!("{}",contents_employee);
}

fn customer(){
    let mut file_customer = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\Customer_Table_tb.sql").unwrap();
    let mut contents_customer = String::new();
    file_customer.read_to_string(&mut contents_customer).unwrap();
    print!("{}",contents_customer);
}

fn vendor(){
    let mut file_vendor = std::fs::File::open(r"C:\Users\danie\OneDrive\Desktop\Dataplan_Table_tb.sql").unwrap();
    let mut contents_vendor = String::new();
    file_vendor.read_to_string(&mut contents_vendor).unwrap();
    print!("{}",contents_vendor);
}