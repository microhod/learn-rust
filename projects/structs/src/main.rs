fn main() {
    // mutate struct fields
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // struct update syntax
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2: username={} email={}", user2.username, user2.email);
    // we can't use user1.username as this was moved to user2
    // println!("user1.username: {}", user1.username);
    // we can use the email though, as that wasn't moved
    println!("user1.email: {}", user1.email);
    // we can also use active, as this was copied (trivial stavck copy) not moved
    println!("user1.active: {}", user1.active);
    println!("user1.sign_in_count: {}", user1.sign_in_count);

    // we can also have structs of just tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// This wouldn't work as &str are references to variables which aren't owned by User (in contrast to 
// String types which are owned by User).
// To make this work, you need to use lifetimes to ensure the compiler knows that it can destroy the 
// underlying data when User goes out of scope.
/*
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
