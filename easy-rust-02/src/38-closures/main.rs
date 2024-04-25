fn main() {
    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {} and {}.", number, other_number);
        // This closure can be as long as we want, just like a function.
    };

    my_closure();
}
