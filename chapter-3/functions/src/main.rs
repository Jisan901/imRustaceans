fn main() {
    let b = add_five(6);  // statement i think
    let valid_value = valid_fn();
    println!("value of b is: {b}, valid_value is {valid_value}");
}

  //fn (parameters | arguments :type)  -> return type {body scope}
fn add_five(x: i32) -> i32 {
    let _ = 4 + x; // statement
    5 + x // expression
}

fn valid_fn() -> i32{
    5 //expression?
}

// let x = {
//     let y = 5;
//     y + 4
// };  This is a powerful feature in Rust that allows you to encapsulate logic within a block and use it directly as a value. It's a common pattern when you need to perform multiple steps to compute a value.


// Key Points:
// In Rust, the last expression in a function or block (without a semicolon) is automatically returned as the result.
// Statements don't return a value and are primarily used for actions like variable binding (let), function calls, etc.
// Expressions can be part of statements, and they always produce a value.