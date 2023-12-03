fn main() {
    // an array of numbers
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}", numbers);

    //create a slice of 2nd and 3rd element
    let sliced = &numbers[1..3];
    println!("2nd and 3rd elemnts sliced = {:?}",sliced);

    // omit the start index 
    let slice2 = &numbers[..3];
    //This means the slice starts from 0 and goes up to index 3 (exclusive)
    println!("index 0 to index 3 sliced = {:?}",slice2);

    // omit the end index
    let slice3 = &numbers[2..];
    //This means the slice starts fro m index 2 and goes up to index 5 (exclusive)
    println!("index 2 to index 5 sliced = {:?}",slice3);

    // omit the start and end index
    // reference the whole array  
    let slice4 = &numbers[..];
    // This meams the slice starts from index 0 and goes up to index 5 (exclusive)
    println!("index 0 to index 5 sliced = {:?}",slice4);
}
