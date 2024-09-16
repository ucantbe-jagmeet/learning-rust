fn find_first_a(s:String) -> Option<i32> {
    for(index, character) in s.chars().enumerate(){
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None
}

fn main (){
    let my_string = String::from("rewewrrwerwaman");

    match find_first_a(my_string) {
        Some(index) => println!("The letter is found at index {}", index),
        None => println!("The letter is not found")
    }
}