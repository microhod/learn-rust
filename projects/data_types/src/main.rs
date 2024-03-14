use std::io::{self, Write};
use std::num::ParseIntError;

// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn main() -> Result<(),String>{
    // tuple
    let tup = (1.1, "hello rusty", -1000);
    println!("tup = {}, {}, {}", tup.0, tup.1, tup.2);


    // array
    let a: [i64; 5] = [1, 2, 3, 4, 5];
    let index = match read_index("please input an array index") {
        Ok(num) => num,
        Err(err) => return Err(err.to_string()),
    };

    if index >= a.len() {
        return Err("index out of bounds".to_string())
    }
    println!("The value of the element at index {index} is: {}", a[index]);

    return Ok(())
}

fn read_index(prompt: &str) -> Result<usize, String> {
    print!("{prompt}: ");
    if let Err(err) = io::stdout().flush() {
        return Err(err.to_string());
    }

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    return index
        .trim()
        .parse()
        .map_err(|err: ParseIntError| err.to_string());
}
