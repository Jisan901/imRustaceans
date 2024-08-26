// heap is slower then stack


fn main() {
    let primitive_str = "we know the length at compile time"; // it should go on stack
    let none_primt_str = String::from("this string can be expand dynamic"); // so it should go on heap {ptr:10-16,len}
    let copy_of_ps = primitive_str;
    //let copy_of_nps = none_primt_str; // i am eating you none_primt_str {ptr:10-16,len}
    
    
    
    println!("ps {primitive_str}");
    println!("nps {none_primt_str}"); // who is he
    println!("cps {copy_of_ps}");
    //println!("cnps {copy_of_nps}");
    
    /*
    hidden,
    drop(primitive_str)  // who is he?
    drop(none_primt_str)
    drop(copy_of_ps)
    drop(copy_of_nps) // i am actually moved :)
    */
}


fn gpt_explanation() {
    let primitive_str = "we know the length at compile time"; 
    // This is a string slice (&str), stored on the stack because its length is known at compile time.

    let none_primt_str = String::from("this string can be expanded dynamically");
    // This is a heap-allocated String. The `String` object itself is stored on the stack, 
    // but it contains a pointer to the actual data on the heap.

    let copy_of_ps = primitive_str;
    // Here, you are copying the `&str`. Since `&str` is a reference type, it implements the `Copy` trait, 
    // so both `primitive_str` and `copy_of_ps` are valid and point to the same data.

    // let copy_of_nps = none_primt_str;
    // This would move `none_primt_str` into `copy_of_nps`. 
    // After the move, `none_primt_str` would be invalid and cannot be used anymore.

    println!("ps {primitive_str}");
    println!("nps {none_primt_str}"); // This will work because `none_primt_str` is still valid.
    println!("cps {copy_of_ps}");
    // println!("cnps {copy_of_nps}"); // This would cause a compile-time error since `none_primt_str` is moved.
}
