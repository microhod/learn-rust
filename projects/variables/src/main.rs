// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn main() {
    let mut x = 5;

    x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
