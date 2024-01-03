fn main() {
    println!("Hello, world!");
    another_function1();
    println!("Sum of x and y is : {}",sum_of_xandy(30, 50) );

}

fn sum_of_xandy(x : u32, y: u32) -> u32{
    let sum = x+ y;
    return  sum;

}

fn another_function1(){
    println!("Another Function");
}
