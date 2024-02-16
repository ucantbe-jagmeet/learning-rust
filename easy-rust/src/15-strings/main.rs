/*
&str is a simple string. When you write let my_variable = "Hello, world!", you create a &str. A &str is very fast.
String is a more complicated string. It is a bit slower, but it has more functions. A String is a pointer, with data on the heap.
*/

fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    let mut my_string: &str = "Try to make this a String".into();
    println!("{}", my_string);
}
