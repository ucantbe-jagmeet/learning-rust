// &i32          // a reference
// &`a i32       // a reference with an explicit lifetime
// &`a mut i32   // a mutable reference with an explicit lifetime


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz"; // Changed to make the strings of different lengths for demonstration
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}
