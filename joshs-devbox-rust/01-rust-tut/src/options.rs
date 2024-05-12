use std::collections::HashMap;

// None to indicate failure or lack of value , and
// Some(value), a tuple struct that wraps a value with type T

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
fn main() {
    let divide1 = divide(4, 2);
    let divide2 = divide(4, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    println!("{:?} unwraps to {}", divide2, divide2.unwrap()); // Unwrapping 'None' ,this will panic
}
