// https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn main() {
    for day in 1..=12 {
        println!();
        print_verse_for_day(day);
    }
}

fn print_verse_for_day(day: usize) {
    let nth = match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    println!("On the {day}{nth} day of Christmas");
    println!("my true love gave to me");

    let presents = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "partridge in a pear tree!"
    ];

    let start: usize = 11 - (day - 1);
    for i in start..11 {
        println!("{}", presents[i])
    }

    if day > 1 {
        println!("And a partridge in a pear tree!");
        return;
    }
    println!("A partridge in a pear tree!");
}
