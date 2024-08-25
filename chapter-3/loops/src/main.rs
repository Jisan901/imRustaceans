fn main() {
    infinite_loop();
    while_loop();
    for_loop();
}

fn infinite_loop() -> u8 {
    println!("\nloop\n");
    let mut i: u8 = 0;

    let result = 'label: loop {
        i = i + 1;
        println!("{i}");

        let value = loop {
            break 5
        };

        println!("Value from inner loop: {value}");

        if i == 5 {
            break 'label i;  // Return `i` when the loop exits
        }
    };

    result // The result is returned from the function
}



fn while_loop() {
    println!("\nwhile loop\n");
    let mut fuel_count = 6;
    println!("Lights on💡");
    while fuel_count != 0 {
        println!("consuming fuel ⛽");
        fuel_count -= 1;
    }
    println!("lights off 🕯️");
    
    
//fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }
}

fn for_loop() {
    
    println!("\nfor loop\n");
    
    let items = ["🍎", "🏀", "😺", "🐶", "🐘"];
    for item in items {
        println!("{item}");
    }
    
    println!("\nfor loop with range\n");
    
    for item in (0..=10).rev() {  // 0..=10 is 0 to 10 but 0..10 is 0 to 9
        println!("{item}");
    }
    
}