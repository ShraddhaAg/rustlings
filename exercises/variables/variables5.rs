// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// you can redeclare a variable in a scope, unlike golang. this is called 'shadowing'.
// usecase - transforming a variable but can use the same variable name + also preserving immutablity 
// difference from mut - can change type



fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
