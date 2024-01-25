// data types are of two types one is scaler and other is compound

// Scaler -> integer, float, boolean, characters
// Compound -> tuples and arrays

fn main(){
        println!("Hello there");

        let integer:i32 = 42; // signed 32-bit integer
        let unsigned_integer:u64 = 10_000; //unsigned 64-bit integer

        let float: f64 = 3.14236;

        let is_true: bool = true;
        let is_false: bool = false;

        let char1: char = 'A';

        // Compound types

        let tup = (500, 6.4, false);
        let (x, y, z) = tup;

        println!("value of x, y, z is {} , {}, {}", x , y, z);

        let x: (i32, f64, u8) = (600, 10.4, 2);
        let five_hundred = x.0;
        let ten_point_four = x.1;


        // array types

        let a = [1, 2,3,4,5];
        let first = a[0];
        let second = a[1];

        let days = ["Sunday", "Monday"];
}