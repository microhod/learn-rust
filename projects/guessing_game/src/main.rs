use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() -> Result<(), String> {
    println!("guess the number!");

    let secret_number = generate_number();
    println!("the secret number is {secret_number}");

    loop {
        let guess;
        match read_guess("please input your guess:") {
            Ok(number) => guess = number,
            Err(err) => return Err(err),
        }

        let guess = match guess.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a positive integer");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("you win!");
                break
            },
        }
    }
    return Ok(());
}

fn read_guess(prompt: &str) -> Result<String, String> {
    print!("{} ", prompt);
    if let Err(err) = io::stdout().flush() {
        return Err(err.to_string());
    }

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        return Err(err.to_string());
    };

    return Ok(input.trim().to_string());
}

fn generate_number() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}
