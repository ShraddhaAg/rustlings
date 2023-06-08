// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: this is a tuple. we can destructure the tuple like below. Both methods work.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    // let name = cat.0; 
    // let age = cat.1;

    println!("{} is {} years old.", name, age);
}
