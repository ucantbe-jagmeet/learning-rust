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
fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).expect("Input vector needs at least 4 items");
    *fourth
}
fn try_two_unwraps(input: Vec<Option<i32>>) {
    println!(
        "Index 0 is: {}",
        input[0].expect("The first unwrap had a None!")
    );
    println!(
        "Index 1 is: {}",
        input[1].expect("The second unwrap had a None!")
    );
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

    //############### unwrap, expect and unwrap_or ############
    let my_vec = vec![9, 0, 10];
    // let fourth = get_fourth(&my_vec);
    // print!("{}", fourth);

    let vector = vec![None, Some(1000)];
    // try_two_unwraps(vector);

    let fourth = my_vec.get(3).unwrap_or(&0);
    // If .get doesn't work, we will make the value &0.
    // .get returns a reference, so we need &0 and not 0
    // You can write "let *fourth" with a * if you want fourth to be
    // a 0 and not a &0, but here we just print so it doesn't matter

    println!("{}", fourth);
}
