//Rust program to enable companies access their data and create a file that stores each companies information 
use std::io;
fn main() {  
   let mut username = String::new();
   println!("Enter your username:");
   io::stdin().read_line(&mut username).expect("Failed to read");
   let username:String = username.trim().parse().expect("Invalid username");
   
   if username.len()>=3 && username.len()<=8{
      println!("you may proceed");
    }
   else{
      println!(" Error! username is invalid please try again");
   }

   let mut password = String::new();
   println!("\nEnter your password");
   io::stdin().read_line(&mut password).expect("Failed to read");
   let password:String = password.trim().parse().expect("Invalid username");
   
   if password.len()<=9{
   if password.chars().all(|c| c.is_lowercase() || c.is_digit(10)){
      println!("Welcome:{}",username);
    }
   }
   else {
      println!("Password is invalid please try again!");
   }

   create();
}

//Using struct to create the file 
use std::io::Write;
struct Company{
company_name:String,
year_company_was_founded:i32,
company_shares:f32,
company_liabilities:f32,
company_leverage:f32,
}

impl Company { 
    fn calc(&self) -> f32 {
        (self.company_shares - self.company_liabilities) / self.company_shares * 100.0

    }
}
fn create(){
let comp1 = Company{
    company_name: "cadbury_nigeria_plc".to_string(),
    year_company_was_founded:1965,
    company_shares:15_000_000.00,
    company_liabilities:5_500_000.00,
    company_leverage:0.00,
 };
 let comp2 = Company{
    company_name:"champion_breweries_plc".to_string(),
    year_company_was_founded:1974,
    company_shares:25_000_000.00,
    company_liabilities:8_000_000.00,
    company_leverage:0.00,
 };
 let comp3 = Company{
    company_name:"dangote_sugar_refinery_plc".to_string(),
    year_company_was_founded:1970,
    company_shares:18_000_000.00,
    company_liabilities:10_000_000.00,
    company_leverage:0.00,
 };
 let comp4 = Company{
    company_name:"flour_mills_nigeria_plc".to_string(),
    year_company_was_founded:1960,
    company_shares:32_000_000.00,
    company_liabilities:4_000_000.00,
    company_leverage:0.00,
 };
 let comp5= Company{
    company_name: "nestle_nigeria_plc".to_string(),
    year_company_was_founded:1961,
    company_shares:8_000_000.00,
    company_liabilities:1_500_000.00,
    company_leverage:0.00,
 };
 let comp6 = Company{
    company_name: "unilever_nigeria_plc".to_string(),
    year_company_was_founded:1923,
    company_shares:37_000_000.00,
    company_liabilities:11_000_000.00,
    company_leverage:0.00,
 };
 let comp7 = Company{
    company_name: "honeywell_nigeria_plc".to_string(),
    year_company_was_founded:1906,
    company_shares:34_000_000.00,
    company_liabilities:9_000_000.00,
    company_leverage:0.0,
 };
 let comp8 = Company{
    company_name: "nigerian_breweries_plc".to_string(),
    year_company_was_founded:1965,
    company_shares:30_000_000.00,
    company_liabilities:12_000_000.00,
    company_leverage:0.00,
 };
 //Converting variables from struct to string
    let var1 : String = format!("{}, {}, {}, {}, {}\n",comp1.company_name, comp1.year_company_was_founded, comp1.company_shares, comp1.company_liabilities, comp1.calc());
    let var2 : String = format!("{}, {}, {}, {}, {}\n",comp2.company_name, comp2.year_company_was_founded, comp2.company_shares, comp2.company_liabilities, comp2.calc());
    let var3 : String = format!("{}, {}, {}, {}, {}\n",comp3.company_name, comp3.year_company_was_founded, comp3.company_shares, comp3.company_liabilities, comp3.calc());
    let var4 : String = format!("{}, {}, {}, {}, {}\n",comp4.company_name, comp4.year_company_was_founded, comp4.company_shares, comp4.company_liabilities, comp4.calc());
    let var5 : String = format!("{}, {}, {}, {}, {}\n",comp5.company_name, comp5.year_company_was_founded, comp5.company_shares, comp5.company_liabilities, comp5.calc());
    let var6 : String = format!("{}, {}, {}, {}, {}\n",comp6.company_name, comp6.year_company_was_founded, comp6.company_shares, comp6.company_liabilities, comp6.calc());
    let var7 : String = format!("{}, {}, {}, {}, {}\n",comp7.company_name, comp7.year_company_was_founded, comp7.company_shares, comp7.company_liabilities, comp7.calc());
    let var8 : String = format!("{}, {}, {}, {}, {}\n",comp8.company_name, comp8.year_company_was_founded, comp8.company_shares, comp8.company_liabilities, comp8.calc());
 
 // Create the file
    let mut file = std::fs::File::create("CA2.txt").expect("create failed"); //Create file 
    file.write_all(var1.as_bytes()).expect("write failed");
    file.write_all(var2.as_bytes()).expect("write failed");
    file.write_all(var3.as_bytes()).expect("write failed");
    file.write_all(var4.as_bytes()).expect("write failed");
    file.write_all(var5.as_bytes()).expect("write failed");
    file.write_all(var6.as_bytes()).expect("write failed");
    file.write_all(var7.as_bytes()).expect("write failed");
    file.write_all(var8.as_bytes()).expect("write failed");

    //  20 million one
    let mut file = std::fs::File::create("comp20.txt").expect("create failed");
   
    if comp1.company_shares > 20_000_000.00 {
        let var20: String = format!("{}, {}\n", comp1.company_name, comp1.calc());
        file.write_all(var20.as_bytes()).expect("write failed");
    }
    if comp2.company_shares > 20_000_000.00 {
        let var21: String = format!("{}, {}\n", comp2.company_name, comp2.calc());
        file.write_all(var21.as_bytes()).expect("write failed");
    }
   
    if comp3.company_shares > 20_000_000.00 {
        let var22: String = format!("{}, {}\n", comp3.company_name, comp3.calc());
        file.write_all(var22.as_bytes()).expect("write failed");
    }
    if comp2.company_shares > 20_000_000.00 {
        let var23: String = format!("{}, {}\n", comp4.company_name, comp4.calc());
        file.write_all(var23.as_bytes()).expect("write failed");
    }
   
    if comp1.company_shares > 20_000_000.00 {
        let var24: String = format!("{}, {}\n", comp5.company_name, comp5.calc());
        file.write_all(var24.as_bytes()).expect("write failed");
    }
    if comp2.company_shares > 20_000_000.00 {
        let var25: String = format!("{}, {}\n", comp6.company_name, comp6.calc());
        file.write_all(var25.as_bytes()).expect("write failed");
    }

    if comp1.company_shares > 20_000_000.00 {
        let var26: String = format!("{}, {}\n", comp7.company_name, comp7.calc());
        file.write_all(var26.as_bytes()).expect("write failed");
    }
    if comp2.company_shares > 20_000_000.00 {
        let var27: String = format!("{}, {}\n", comp8.company_name, comp8.calc());
        file.write_all(var27.as_bytes()).expect("write failed");
    }

   
    if comp1.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp1.calc();
        println!("{}", c );
    }
    if comp2.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp2.calc();
        println!("{}", c );
    }
   
    if comp3.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp3.calc();
        println!("{}", c );
    }
    if comp2.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp4.calc();
        println!("{}", c );
    }
   
    if comp1.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp5.calc();
        println!("{}", c );
    }
    if comp2.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp6.calc();
        println!("{}", c );
    }

    if comp1.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp7.calc();
        println!("{}", c );
    }
    if comp2.company_liabilities < 10_000_000.00 {
        let c = 0.05 * comp8.calc();
        println!("{}", c );
     }
}
 
