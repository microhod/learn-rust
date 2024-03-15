// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    let s1 = String::from("hello");

    // pass the 'reference' (which is like a normal pointer, 
    // but it's guaranteed to point to a valid value of the referenced type)
    let len = calculate_length(&s1);

    // because the reference was passed, ownership didn't move from s1,
    // so we can still use that variable.
    println!("The length of '{}' is {}.", s1, len);

    // this also works because s2 is not a heap allocated variable
    let s2 = &s1;
    let len = calculate_length(s2);
    println!("The length of '{}' is {}.", s2, len);

    // to change a variable it must be mutable on declaration
    let mut changable = String::from("I can be changed");
    // to change a mutable variable, you must pass a mutable reference
    change(&mut changable);
    // but you can still pass an immutable reference to a mutable variable
    print_string(&changable);
    println!("Changed value = '{}'", changable);

    // once you have one mutable reference no other references can be taken for that value
    let r1 = &mut changable;
    // let r2 = &mut changable; // no 2nd mutable reference allowed
    // let r3 = &changable;     // no immutable reference allowed 
    println!("{}", r1);         // add r2 or r3 to print to break compilation

    // similarly, once you have an immutable reference you can't take a mutable one
    let r1 = &changable;
    let r2 = &changable;        // multiple immutable references are fine
    // let r3 = &mut changable; // mixed references are not allowed
    println!("{} | {}", r1, r2); // add r3 to print to break compilation

    // but this use of mixed references is okay, as the time the references are taken are distinct
    let r1 = &changable; // no problem
    let r2 = &changable; // no problem
    println!("{} | {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut changable; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// change must request a mutable reference to be able to change
// the underlying variable
fn change(s: &mut String) {
    s.push_str(", and I have been changed");
}

// print_string is not mutating, so it can simply take a reference
fn print_string(s: &String) {
    println!("{}", s)
}

// this fails because there's no value for the borrow to be taken from in the return value
// as s is out of scope once dangle() returns
// fn dangle() -> &String {
//     let s = String::from("hello");
//     return &s
// }
