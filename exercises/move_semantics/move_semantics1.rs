// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: so this one is about ownership. 
// first, lets fix the error `cannot borrow `vec1` as mutable, as it is not declared as mutable`. 
// now, vec0 must be not accessible after we call fill_vec because its ownership is "moved" to the func. 
// we get error: `borrow of moved value: `vec0``. (rust uses the term "moved" when ownership is transfered from one scope to another)


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // println!("{} has content `{:?}`", "vec1", vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
