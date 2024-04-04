use std::env::args;
use std::env::args;
use std::io;

enum Letters {
    Capitalize,
    Lowercase,
    Nothing,
}

fn main() {
    // println!("Please type something, or x to excape:");
    // let mut input_string = String::new();

    // while input_string.trim() != "x" {
    //     input_string.clear();
    //     io::stdin().read_line(&mut input_string).unwrap();
    //     println!("you wrote {}", input_string);
    // }
    // loop will not exited because , This is because the keyboard input is actually not just something, it is something and the Enter key. There is an easy method to fix this called .trim(), which removes all the whitespace.
    // println!("see you later");

    // let input = args();
    // for entry in input {
    //     println!("You entered: {}", entry);
    // }

    let mut changes = Letters::Nothing;
    let input = args().collect::<Vec<_>>();

    if input.len() > 2 {
        match input[1].as_str() {
            "capital" => changes = Letters::Capitalize,
            "lowercase" => changes = Letters::Lowercase,
            _ => {}
        }
    }

    for word in input.iter().skip(2) {
        match changes {
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowercase => println!("{}", word.to_lowercase()),
            _ => println!("{}", word),
        }
    }
}
