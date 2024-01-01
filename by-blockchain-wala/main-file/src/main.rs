fn main() {
    println!("Hello, world!");
    let sum = sum_of_xandy(30, 50);
    another_function1();
    println!("Sum of x and y is : {sum}");

}

fn sum_of_xandy(x : u32, y: u32) -> u32{
    let sum = x+ y;
    return  sum;

}

fn another_function1(){
    println!("Another Function");
}
