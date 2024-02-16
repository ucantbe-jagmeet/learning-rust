fn main() {
    // let my_number = 8;
    // // my_number = 10; // this will gives error , we have to write mut keyword


    // let mut num = 10;
    // // num =  12;
    // num =  "Hello world "; // this will gives error, can override int to string


    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    {
        let my_number = 9.2; // This is an f64. It is not my_number - it is completely different!
        println!("{}", my_number) // Prints 9.2
                                  // But the shadowed my_number only lives until here.
                                  // The first my_number is still alive!
    }
    println!("{}", my_number); // prints 8
}
