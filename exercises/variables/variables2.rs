// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// rust requires (will throw an error otherwise) variables to be initialised, unlike golang
// there is no default, unlike golang
// can not do boolean operations if not the same type

fn main() {
    let x = 5;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
