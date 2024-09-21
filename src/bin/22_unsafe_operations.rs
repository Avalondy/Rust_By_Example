// There are four primary things that unsafe is used for:
// 1. Dereferencing raw pointers
// 2. Calling functions or methods which are unsafe (including calling a function over FFI, see a previous chapter of the book)
// 3. Accessing or modifying static mutable variables
// 4. Implementing unsafe traits

use std::slice;

fn raw_pointers() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
        println!("{}", *raw_p);
    }
}

fn unsafe_functions() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
        println!("{:?}", my_slice);
    }
}

fn main() {
    raw_pointers();
    unsafe_functions();
}
