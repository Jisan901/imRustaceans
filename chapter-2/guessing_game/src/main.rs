use rand::Rng;
use std::cmp::Ordering;
use std::io;

// rust way

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


// my way

fn main2() {
    println!("\n\n\n\n\n******** Let's guess ********");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut solved: bool = false;

    while !solved {
        solved = solve(&secret_number);
    }
}

fn solve(secret_number: &i32) -> bool {
    let guess = get_input();
    if guess == 0 {
        println!("Enter a number between 1-100.");
        return false;
    } else if guess == 101 {
        println!("Quitting....");
        return true;
    }
    let solved = check_value(&guess, &secret_number);
    return solved;
}

fn check_value(a: &i32, b: &i32) -> bool {
    if a > b {
        println!("\n...[to big]...");
        return false;
    } else if a < b {
        println!("\n...[to low]...");
        return false;
    } else {
        println!("\n*** correct ***\n");
        return true;
    }
}

fn get_input() -> i32 {
    let mut guess = String::new();
    println!("\nEnter your guess between 1-100: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("[ERROR]: Failed to read input!");

    if guess == "q\n" {
        return 101;
    }

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    // .expect("[ERROR]: guess should be an integer: i32");
    return guess;
}
