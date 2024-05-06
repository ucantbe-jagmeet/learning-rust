/*
    stack -> fast allocation and deallocation.
            rust uses the stack for most primitive data types and for data where the size is known at compile time (i.e numbers)
    heap -> used for data that can grow at runtime, such as vectors orr strings.


*/

fn main() {
    stack_fn(); // call the function that uses the stack memory
    heap_fn(); // call the function that uses the heap memory

    update_string(); // call the function that changes the size of vairbale at runtime
}

fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;

    println!("Stack function: sum is {}", c);
}
fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);

    println!("heap function: Combined string is {}", combined);
}
fn update_string() {
    let mut s = String::from("Initial String");

    s.push_str(" and sone additional string");

    println!("update function: After updated: {}", s);
}
