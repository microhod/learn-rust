// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    stack_copy();
    heap_clone();

    let s1 = String::from("hello world");
    takes_ownership(s1);
    // this won't work as s1 was moved when ownership was passed to takes_ownership
    // println!("{}", s1);

    let s2 = String::from("hello rusty");
    let s3 = takes_and_gives_back(s2);
    println!("got back = {}", s3);
}

fn stack_copy() {
    let x = 5;

    // doing a trivial copy of data on the stack
    // stack allocated variables use the Copy trait
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn heap_clone() {
    let s1 = String::from("hello");
    
    // would stop us using s1 as the ownership has moved
    // let s2 = s1;
    
    // this copies the actual data in the heap
    // heap allocated variables use the Clone trait
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(s: String) {
    println!("took ownership of = {}", s);
}

fn takes_and_gives_back(s: String) -> String {
    println!("took ownership, but will give it back = {}", s);
    return s
}
