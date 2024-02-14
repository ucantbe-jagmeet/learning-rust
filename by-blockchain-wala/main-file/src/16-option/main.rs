// Option - example 
// enum Option<T>{
//     None,
//     Some(T)
//  }

fn find_index( vec: Vec<i32>, element:i32) -> Option<usize>{
    for (index, value) in vec.iter().enumerate() {
        if *value == element {
            return Some(index);
        }
    }
    None
}

fn main(){
    let numbers = vec![ 1, 2, 3, 4, 5];
    let index = find_index( numbers, 4);

    match index {
        Some(i) => println!("Element found at index {}", i),
        None => println!("Element not found"),
    }
}
