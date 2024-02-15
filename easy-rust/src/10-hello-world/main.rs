fn number() -> i32 {
    8
}
fn multiply(number_one: i32, number_two: i32) { 
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}
fn main() {
    println!("Hello, world!");
    println!("Hello, world number {}!", 8);
    println!("Hello, worlds number {} and {}!", 8, 9);
     println!("Hello, world number {}!", number());
}
