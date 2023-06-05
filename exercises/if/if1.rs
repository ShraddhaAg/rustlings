// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// if we use only an if statement and not add the else clause, and try to return values using expressions, rust complains. 
// the error thrown is: `if` expressions without `else` evaluate to `()`
// 
// from Control Flow section of chapter 3: https://doc.rust-lang.org/book/ch03-05-control-flow.html 
// "the values that have the potential to be results from each arm of the if must be the same type"
//  by default the return value of an if expression is (), ie the unit type. https://doc.rust-lang.org/std/primitive.unit.html
// 
// so the answer to the above question, why we encountered the error: 
// because it is an expression, it must return value of a single type. if we don't specify the else branch, 
// if branch is returning a i32 type and else branch is returning a unit ie () type. 
// 
// another thing to note after researching about this, if-else are expressions, not statement.
// almost everything in rust is an expression. 
// TODO: learn more about why this is so.

pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {a} else {b}
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
