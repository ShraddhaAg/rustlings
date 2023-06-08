// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// 1. how do I define an array? [type annotation; size of array]
// 2. how to intialise arrays with a certain size? [<value to be repeated>; size of array]

fn main() {
    let a:[i32; 4] = [1, 2, 3, 4]; 
    let a = ["hello"; 100];

    print!("{:?}", a);
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
