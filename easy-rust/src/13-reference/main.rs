/*
    -> The stack is very fast, but the heap is not so fast. It's not super slow either, but the stack is always faster. But you can't just use the stack all the time, because:
    
    -> Rust needs to know the size of a variable at compile time. So simple variables like i32 go on the stack, because we know their exact size. You always know that an i32 is going to be 4 bytes, because 32 bits = 4 bytes. So i32 can always go on the stack.
    -> But some types don't know the size at compile time. But the stack needs to know the exact size. So what do you do? First you put the data in the heap, because the heap can have any size of data. And then to find it a pointer goes on the stack. This is fine because we always know the size of a pointer. So then the computer first goes to the stack, reads the pointer, and follows it to the heap where the data is.

*/
fn main() {
    let my_number = 15; // This is an i32
    let single_reference = &my_number; //  This is a &i32
    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32
}
