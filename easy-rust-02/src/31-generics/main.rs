use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}
fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}
fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    // Type T needs Display, type U needs Display
    println!(
        "I have two things to say: {} and {}",
        statement_1, statement_2
    );
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
    compare_and_display("Listen up!", 9, 8);

    say_two("Hello there!", String::from("I hate sand.")); // Type T is a &str, but type U is a String.
    say_two(
        String::from("Where is Padme?"),
        String::from("Is she all right?"),
    ); // Both types are String.
}