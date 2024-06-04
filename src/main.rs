use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn generateSecretNumber() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn handleInput() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = match guess.trim().parse::<u32>() {
        Ok(num) => {
            if num < 1 || num > 100 {
                println!("Please type a number between 1 and 100");
                return handleInput();
            }
            num
        },
        Err(_) => {
            println!("Please type a number!");
            return handleInput();
        }
    };

    guess
}

fn compareNumbers(guess: u32, secret_number: u32) {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}

fn main() {
    let secret_number = generateSecretNumber();
    let guess = handleInput();
    compareNumbers(guess, secret_number);
}

