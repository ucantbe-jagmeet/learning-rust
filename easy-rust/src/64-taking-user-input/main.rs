use std::io;

fn main() {
    println!("Please type something, or x to excape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("you wrote {}", input_string);
    }
    // loop will not exited because , This is because the keyboard input is actually not just something, it is something and the Enter key. There is an easy method to fix this called .trim(), which removes all the whitespace.
    println!("see you later");
}
