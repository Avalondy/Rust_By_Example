// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(rust_by_example::add(3, 2), 5);
}
