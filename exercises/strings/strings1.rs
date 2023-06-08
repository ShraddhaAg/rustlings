// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: from() converts a &str (ie a slice) to String (ie owned string).
// &str - immutable refernce, dynamic length, 
// TODO - understand this answer: https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str  

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
