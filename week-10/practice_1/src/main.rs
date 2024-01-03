fn main() {
    let v = vec![202,350,330,400];
    //vector v owms the object in heap 
    
    // only a single variable owns the heap meemory at any given time
    let v2 = v;
    //here two variables owns heap value
    //two pointerss to the samw content is not alloed in rust
    

    //Rust is very smart in terms of memory access,so it detects a race condition 
    //as two variables point to have to same heap 

    println!("{:?}",v);
}
