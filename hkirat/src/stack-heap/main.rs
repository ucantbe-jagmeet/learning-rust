/*
    stack -> fast allocation and deallocation.
            rust uses the stack for most primitive data types and for data where the size is known at compile time (i.e numbers)
    heap -> used for data that can grow at runtime, such as vectors orr strings.


*/

fn main() {
    stack_fn(); // call the function that uses the stack memory
}

fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;

    println!("Stack function: sum is {}", c);
}
