// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

// shraddhaag notes: so the problem is, we want vec0 to be available even after we 'move' it to fill_vec. 
// so solution should be to send a copy of vec0 instead or to send the reference of vec0 to the func ie borrow. 
// 1. since the result expects vec0 to also have the values, we want to borrow vec0 and it should be a mutable borrow. 
// 2. if we want to borrow vec0 as mutable, we need to define vec0 as mutable. 
// 3. now that the func param is a mutable borrow, param type should change
// 4. since it is a mutable borrow, we don't need to define a new vec, we can make changes in place
// 5. now we can return a mutable reference from the func, but this creates problems. namely: 
// `cannot borrow `vec0` as immutable because it is also borrowed as mutable`. 
// immutable borrows: vec0.len() | mutable borrows: fill_vec(&mut vec0), vec1.push(88) 
// the last one is interesting. because we returned a mutable reference, vec1 is essentially is a ref to vec0 itself. 
// so to modify vec1 later, we need to pass a value which does not reference to vec0. 
// so the func's return type should just be Vec<i32> not &mut Vec<i32>. to do the same, we use to_vec()
// 
// to_vec() essentially copies the passed object (can be array, vec, etc) to a new vector.


fn main() {
    // if we want to borrow vec0 as mutable, we need to define vec0 as mutable.
    let mut vec0 = Vec::new();

    // Do not move the following line!
    // since the result expects vec0 to also have the values, we want to borrow vec0 and it should be a mutable borrow.
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// now that the func param is a mutable borrow, param type should change mutable ref
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // since it is a mutable borrow, we don't need to define a new vec, we can make changes in place
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

// now we can return a mutable reference from the func, but this creates problems. namely: 
// `cannot borrow `vec0` as immutable because it is also borrowed as mutable`. 
// immutable borrows: vec0.len() | mutable borrows: fill_vec(&mut vec0), vec1.push(88) 
// the last one is interesting. because we returned a mutable reference, vec1 is essentially is a ref to vec0 itself. 
// so to modify vec1 later, we need to pass a value which does not reference to vec0. 
// so the func's return type should just be Vec<i32> not &mut Vec<i32>. to do the same, we use to_vec()
    vec.to_vec()
}
