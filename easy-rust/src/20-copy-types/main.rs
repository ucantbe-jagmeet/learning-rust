// we can easily copy intergers . boolean and floats 

fn prints_number(number: i32) { // There is no -> so it's not returning anything
                             // If number was not copy type, it would take it
                             // and we couldn't use it again
    println!("{}", number);
}
fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn get_length(input: String) { // Takes ownership of a String
    println!("It's {} words long.", input.split_whitespace().count()); // splits to count the number of words
}

fn main() {
    let my_number = 8;
    prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
    prints_number(my_number); // Prints 8 again.
                              // No problem, because my_number is copy type!

    // let country = String::from("Kiribati");
    // prints_country(country); // we have to clone it first
    // prints_country(country.clone()); 
    // prints_country(country); // this will give error

    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words"); // push the words on
        get_length(my_string.clone()); // gives it a clone every time
    }
}
