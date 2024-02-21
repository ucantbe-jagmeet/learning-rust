/*
There are three other macros that are similar to panic! that you use a lot in testing. They are: assert!, assert_eq!, and assert_ne!.

Here is what they mean:

assert!(): if the part inside () is not true, the program will panic.
assert_eq!(): the two items inside () must be equal.
assert_ne!(): the two items inside () must not be equal. (ne means not equal)
*/

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?; // Add a ? each time to check and pass it on
    Ok(parsed_number)
}
fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items") // will panic if the length is not 3
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}
fn main() {
    // let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];

    // for item in str_vec {
    //     let parsed = parse_str(item);
    //     println!("{:?}", parsed);
    // }

    // let my_vec = vec![8, 9, 10, 10, 55, 99];
    // prints_three_things(my_vec);

    // let my_name = "Loki Laufeyson";

    // assert!(my_name == "Loki Laufeyson");
    // assert_eq!(my_name, "Loki Laufeyson");
    // assert_ne!(my_name, "Mithridates");
    // // These messages will only display if the program panics. So if you run this:
    // assert!(
    //     my_name == "Loki Laufeyson",
    //     "{} should be Loki Laufeyson",
    //     my_name
    // );
    // assert_eq!(
    //     my_name, "Loki Laufeyson",
    //     "{} and Loki Laufeyson should be equal",
    //     my_name
    // );
    // assert_ne!(
    //     my_name, "Loki Laufeyson",
    //     "You entered {}. Input must not equal Loki Laufeyson",
    //     my_name
    // );
}
