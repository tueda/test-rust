extern crate test_rust;

use test_rust::foo::add_two;

#[test]
fn test_add_two() {
    assert_eq!(5, add_two(2, 3));
}
