// we can easily copy intergers . boolean and floats

fn prints_number(number: i32) {
    // There is no -> so it's not returning anything
    // If number was not copy type, it would take it
    // and we couldn't use it again
    println!("{}", number);
}

fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn get_length(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    // let my_number = 8;
    // prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
    // prints_number(my_number); // Prints 8 again.
    // // let country = String::from("Kiribati");
    // // prints_country(country); // we have to clone it first
    // // prints_country(country.clone());
    // // prints_country(country); // this will give error

    // let mut my_string = String::new();
    // for _ in 0..50 {
    //     my_string.push_str("Here are some more words "); // push the words on
    //     // get_length(my_string.clone()); // gives it a clone every time
    //     // That's 50 clones. Here it is using a reference instead, which is better:

    //     // so try this
    //     get_length(&my_string); // instead of 50 clones, it's zero.
    // }

    let my_number;

    {
        // Pretend we need to have this code block
        let number = {
            // Pretend there is code here to make a number
            // Lots of code, and finally:
            57
        };

        my_number = loop_then_return(number);
    }
    println!("{}", my_number);
}
