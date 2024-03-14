use std::{
    io::{self, Write},
    num::ParseIntError,
};

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn main() -> Result<(), String> {
    // we have for loops
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let limit = match read_limit("input the number to count to") {
        Ok(num) => num,
        Err(err) => return Err(err.to_string())
    };

    let mut counter = 0;
    // you can return from an if condition 🙀
    let increment = if limit % 2 == 0 {2} else {1};

    let result = 'counting_up: loop {
        println!("counter = {counter}");
        counter += increment;

        let mut remaining = limit;
        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if counter == 3 {
                // loop labels allow you to break out of outer loops
                break 'counting_up counter * 3;
            }
            remaining -= increment;
        }

        if counter == limit {
            // you can return values from a loop on breaks 🤨
            break counter * 2;
        }
    };

    println!("The result is {result}");
    return Ok(())
}

fn read_limit(prompt: &str) -> Result<usize, String> {
    print!("{prompt}: ");
    if let Err(err) = io::stdout().flush() {
        return Err(err.to_string());
    }

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    return index
        .trim()
        .parse()
        .map_err(|err: ParseIntError| err.to_string());
}
