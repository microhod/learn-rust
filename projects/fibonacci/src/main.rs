use std::{io::{self, Write}, num::ParseIntError};

fn main() -> Result<(),String> {
    let n = match read_number("input the required fibonacci number") {
        Ok(num) => num,
        Err(err) => return Err(err.to_string())
    };

    let fibn = fib(n);

    let nth = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    println!("the {n}{nth} fibonacci number is: {fibn}");
    return Ok(());
}

fn fib(n: u64) -> u64 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn read_number(prompt: &str) -> Result<u64, String> {
    print!("{prompt}: ");
    if let Err(err) = io::stdout().flush() {
        return Err(err.to_string());
    }

    let mut index = String::new();
    if let Err(err) = io::stdin().read_line(&mut index) {
        return Err(err.to_string());
    }

    return index
        .trim()
        .parse()
        .map_err(|err: ParseIntError| err.to_string());
}
