// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// shraddhaag notes: 
// how to create a slice? slices are just references to the original array.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    print!("{:?}", nice_slice);
    assert_eq!([2, 3, 4], nice_slice);
}
