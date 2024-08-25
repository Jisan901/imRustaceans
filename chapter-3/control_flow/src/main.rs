fn main () {
    let depends = if 5>6 {true} else {false}; // inline if else
    
    let condition = depends;
    // multiline condition
    if condition {
        println!("condition was true");
    }
    // else if condition {
       // also available
    // }
    else {
        println!("condition was false");
    }
    
    
    // also valid
    // if condition {
    //     unimplemented!();
    // }
    // if condition {
    //     unimplemented!();
    // }
}