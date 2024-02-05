fn main(){
    // stack allocated variable a and b
    let a = 10;
    let b = 20;

    // stack allocated variable sum to store the result of adding a and b
    let sum = add_numbers( a, b);

    // heap allocated string
    let heap_value = String::from("Hello, World!");

    println!("Sum : {}", sum);
    println!("Heap value : {}", heap_value);
}

fn add_numbers(x : i32, y:i32) -> i32{
    // stack allocated variable result to store the result of adding x and y
    let result = x + y;

    //  heal allocated vector
    let heap_value2 = Box::new( vec![ 1, 2, 3, 4, 5]);

    println!("Heap value 2 : {:?}", *heap_value2);
    result
}