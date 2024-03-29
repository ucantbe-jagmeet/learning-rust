// Box is a very convenient type in Rust. When you use a Box, you can put a type on the heap instead of the stack. To make a new Box, just use Box::new() and put the item inside.

// fn just_takes_a_variable<T>(item: T) {} // Takes anything and drops it.

// fn main() {
//     let my_number = 1; // This is an i32
//     just_takes_a_variable(my_number);
//     just_takes_a_variable(my_number); // Using this function twice is no problem, because it's Copy

//     let my_box = Box::new(1); // This is a Box<i32>
//     just_takes_a_variable(my_box.clone()); // Without .clone() the second function would make an error
//     just_takes_a_variable(my_box); // because Box is not Copy
// }

struct List {
    item: Option<Box<List>>,
}

impl List {
    fn new() -> List {
        List {
            item: Some(Box::new(List { item: None })),
        }
    }
}

fn main() {
    let mut my_list = List::new();
}
