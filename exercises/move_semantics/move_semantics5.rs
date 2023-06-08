// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: at first glance, I can't see any problem with this. 
// running it we get this error: cannot borrow `x` as mutable more than once at a time
// fix is simply to move the usage of first borrow before we define the second borrow. 
//  
// but WHY can't we borrow a variable more than once?? 
// they have described it in detail here (rust book is AMAZE!): https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
// multiple mutable ref not allowed because this can cause data races. 
// both mutable and immutable ref not allowed as the immutable ref is not expecting the data to change but it CAN change because a mutable ref also exists. 
// multiple immutable ref are allowed. 
// discuss with sjb

fn main() {
    let mut x = 100;

    let y = &mut x;
    *y += 100;

    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
