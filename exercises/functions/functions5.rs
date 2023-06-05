// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: a line in a func without a semi colon (an expression) is equivalent to a return statement
// statements - tell computers what to do 
// expressions - anything that evaluates to a value

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // notice no semi colon at the end. this is an expression not a statement
    // the below line is equivalent to return num * num;
    num * num
}
