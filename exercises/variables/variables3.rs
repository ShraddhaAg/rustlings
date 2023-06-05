// variables3.rs
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// variables need to be initialised, but.... 
// both declaration and initialisation need not happen in the same line.
// so, from the previous and this exercise - 
// we need to EXPLICITLY tell the compiler to initialise a variable otherwise the allocated 
// memory for the variable is "uninitalised memory". 
// uninitialised memory just means undefined behaviour. Memory has been allocated to the variable but not yet initalised
// ie previous data has been not been overwritten by a default value.
// lots of good answers here: https://stackoverflow.com/questions/73837626/what-is-uninitialized-memory-and-why-isnt-it-initialized-when-allocating

// use std::mem::size_of_val;

fn main() {
    let x: i32;
    // print!("{}", size_of_val(&x));
    x = 5;
    println!("Number {}", x);
}