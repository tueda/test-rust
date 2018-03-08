use std::ops::Add;

pub fn add_two<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
