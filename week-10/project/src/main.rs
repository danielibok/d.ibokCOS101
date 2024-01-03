 //Rust program to Calculate the total cost of laptops purchased 
 //if each customer purchased 3 from each brand. 
struct Laptop{ 
cost:i32,
number_of_laptops:i32,
}

fn main() {
    let l1 = Laptop{
        cost:650_000,
        number_of_laptops:3,
    };
    let l2 = Laptop{
        cost:755_000,
        number_of_laptops:3,
    };
    let l3 = Laptop{
        cost:550_000,
        number_of_laptops:3,
    };
    let l4 = Laptop{
        cost:850_000,
        number_of_laptops:3,
    };
    let hp = l1.cost * l1.number_of_laptops;
    let ibm = l2.cost * l2.number_of_laptops;
    let toshiba = l3.cost * l3.number_of_laptops;
    let dell = l4.cost * l4.number_of_laptops;
    
    let total_cost = hp + ibm + toshiba + dell;
    println!("The Total cost of your purchase is N{}",total_cost);
    //I sincerely apologize for commiting late :(
}