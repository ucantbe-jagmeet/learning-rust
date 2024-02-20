// The important part is the <T> after the function name. Without this, Rust will think that T is a concrete (concrete = not generic) type, like String or i8.
use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}
fn return_number<T: Debug>(number: T) -> T {
    println!("The number is: {:?}", number);
    number
}

fn main() {
    let number = return_number(5);

    let charlie = Animal {
        name: "JAgmeet".to_string(),
        age: 10,
    };

    print_item(charlie);
    print_item(number);
}
