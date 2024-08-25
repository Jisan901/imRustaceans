fn main() {
    infinite_loop();
}

fn infinite_loop() -> u8 {
    let mut i: u8 = 0;

    let result = 'label: loop {
        i = i + 1;
        println!("{i}");

        let value = loop {
            break 5;
        };

        println!("Value from inner loop: {value}");

        if i == 255 {
            break 'label i;  // Return `i` when the loop exits
        }
    };

    result // The result is returned from the function
}



fn while_loop() {
    
}

fn for_loop() {
    
}