// You have seen code like #[derive(Debug)] before: this type of code is called an attribute. These attributes are small pieces of code that give information to the compiler. They are not easy to create, but they are very easy to use. If you write an attribute with just # then it will affect the code on the next line. But if you write it with #! then it will affect everything in its own space.

#![allow(dead_code)]
#![allow(unused_variables)]

struct Struct1 {} // Create five structs
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}

#[derive(Clone, Copy)] // You also need Clone to use Copy
struct NumberAndBool {
    number: i32,         // i32 is Copy
    true_or_false: bool, // bool is also Copy. So no problem
}

fn does_nothing(input: NumberAndBool) {}

fn main() {
    let char1 = 'ん'; // and four variables. We don't use any of them but the compiler is quiet
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];

    let number_and_bool = NumberAndBool {
        number: 8,
        true_or_false: true,
    };

    does_nothing(number_and_bool);
    does_nothing(number_and_bool); // If it didn't have copy, this would make an error
}
