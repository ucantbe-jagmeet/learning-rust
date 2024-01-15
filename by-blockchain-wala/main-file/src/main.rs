// statement -> it returns nothing
// expression -> it returns something



fn main() {
    println!("Hello, world!");
    another_function1();
    println!("Sum of x and y is : {} and {}",sum_of_xandy(30, 50), sum_of_xandy(20, 100) );

}

fn sum_of_xandy(x : u32, y: u32) -> u32{
    let sum = x+ y;
    sum

}

fn another_function1(){
    println!("Another Function");
}
