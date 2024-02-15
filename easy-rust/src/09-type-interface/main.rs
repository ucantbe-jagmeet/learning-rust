// type interface - it means that if you don't tell the compiler the type, but it can decide by itself, it will decide. 

fn main (){
    // let small_number: u8 = 10;
    let small_number = 10u8;

    let small_number2 = 10_u8; // This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _
    let number = 0________u8;
    let number2 = 1___6______2____4______i32; //This prints 1624.
    // The _ does not change the number. It is only to make it easy for you to read. And it doesn't matter how many _ you use

     let my_float1 = 5.; // Rust sees . and knows that it is a float

     // example
    let my_float: f64 = 5.0; // This is an f64
    let my_other_float: f32 = 8.5; // This is an f32

    let third_float = my_float + my_other_float; // shows error

     let third_float1 = my_float + my_other_float as f64; // my_other_float as f64 = use my_other_float like an f64
}