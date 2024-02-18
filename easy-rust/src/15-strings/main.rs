/*
&str is a simple string. When you write let my_variable = "Hello, world!", you create a &str. A &str is very fast.
String is a more complicated string. It is a bit slower, but it has more functions. A String is a pointer, with data on the heap.
*/

fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";
    // The format! macro in Rust is used to create a String with formatted text. In your provided code, format! takes a format string and a list of arguments, then it interpolates the arguments into the format string's placeholders ({}) and returns a new String that contains the result. 
    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    let mut my_string: &str = "Try to make this a String".into();
    println!("{}", my_string);
}
