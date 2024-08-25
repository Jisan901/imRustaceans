const PER_BLADE_HEIGHT : i32 = 5;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 9; // mutable variable can change the value. but re assigning with let creates shadow.
    println!("The value of x is: {x}");
    println!("const is {}", PER_BLADE_HEIGHT);
    shadowing();
}

fn shadowing() {

    println!("shadowing");

    let x = 5;
    let x = x + 5; // creates a shadow on top of x 
    // mutable variable can't create a shadow it just assign the value based on base type
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // can't create shadow on top of parent.

    println!("The value of x is: {x}");
}