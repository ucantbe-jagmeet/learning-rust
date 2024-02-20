// You use Option when you have a value that might exist, or might not exist. When a value exists it is Some(value) and when it doesn't it's just None, Here is an example of bad code that can be improved with Option.

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[5])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(num) => println!("Got a number: {}", num),
            None => println!("Got nothing"),
        }
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5, 6];
    println!(
        "{:?} {:?}",
        take_fifth(new_vec.clone()),
        take_fifth(bigger_vec.clone())
    );

    //  we can use .unwrap() to get the value inside Some(value) , but we can apply it on None
    //  so try it with match

    let mut option_vec = Vec::new(); // Make a new vec to hold our options
                                     // The vec is type: Vec<Option<i32>>. That means a vec of Option<i32>.

    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger_vec));

    handle_option(option_vec);
}
