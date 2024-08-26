fn main() {
    let mut s1 = String::from("hello");
    let length = calculate_length(&s1); // passing reference not ownership
    let r1 = &s1;
    println!("before change length of {s1} is {length} ref : {}", r1);
    let r1 = &mut s1; // can valid because r1 already passed on println and no longer used 
    dont_take_ownership_just_change(r1); // i am passing mutable ref // r1 dropped again
    
    let length = calculate_length(&s1); // passing reference not ownership
    
    let r1 = &s1; // if we don't declare this line it will a big prob for line 6
    println!("length of  is {length}, ref: {}",r1);
    

}


// &String will get a reference of String type
fn calculate_length(s: &String) -> usize {
    s.len()
}// s won't dropped because calculate_length isn't owner of the s

fn dont_take_ownership_just_change(s: &mut String) {
    s.push_str(", world");
}

//A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.


//Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

//Filename: src/main.rs

//This code does not compile!
    // let mut s = String::from("hello");
    //invalid
    // let r1 = &mut s;
    // let r2 = &mut s;
    //invalid 
    // let r1 = &s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    
    //valid
    // let r1 = &s;
    // let r2 = &s;
    
// We also cannot have a mutable reference while we have an immutable one to the same value.

/// hanging reference :O

//fn dangle() -> &String { // dangle returns a reference to a String

   // let s = String::from("hello"); // s is a new String

  //  &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
  
  
  // solution 
  
//   fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }