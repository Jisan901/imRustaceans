fn main() {
    let string_d = String::from("Hello");
    let string_p = 'A';
    
    let string_d = get_ownership_back(string_d); // d moves here  and d comes in by shadow
    
    
    let (string_d, length) = calc_len(string_d); // shadow
    
    println!("string {string_d} length: {length}");
    
    let dog = buy_dog(); // getting ownership of dog
    let dog2 = buy_dog(); // getting ownership of dog
    println!("my {dog},{dog2}");
    get_ownership(string_d); // moved to get_ownership function because of copying reference and dropping same memory issue 
    
    make_copy(string_p); // copy of primitive from stack
    
    println!("valid? p {string_p}");
    //println!("valid? d {string_d2}"); // not valid
    //println!("valid? d {string_d}"); // not valid
}

fn get_ownership(some_str: String) {
    println!("i owned you {some_str}");
}
fn make_copy(some_str: char) {
    println!("copied {some_str}");
}

fn get_ownership_back(arg: String) -> String {
    println!("he is mine '{arg}'");
    arg // if not returned it will drop
}


fn buy_dog() -> String {
    let dog = String::from("doggy!!"); // i am owner
    dog 
}


fn calc_len(s : String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}


//myfun(); // The returned `String` is immediately dropped since it's not bound to a variable.

// fn myfun() -> String {
//     let s = String::from("Hello, world!");
//     s // Ownership of `s` is returned, but no one claims it.
// }