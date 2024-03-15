// https://doc.rust-lang.org/book/ch04-03-slices.html
fn main() {
    let s = String::from("Hello, world!");

    let word = first_word(&s);

    // this is not allowed as it would invalidate the slice 'word'. This is
    // limited by the fact that clear() requires a mutable reference, which 
    // is not allowed as we already took an immutable reference in the call to first_word()
    // s.clear();

    println!("{}", word);

    // slices work with other types too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}
