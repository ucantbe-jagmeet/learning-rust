enum CustomOption {
    Some(i32),
    None,
}

fn find_first_a(s:String) -> CustomOption {
    for(index, character) in s.chars().enumerate(){
        if character == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None
}

fn main (){
    let my_string = String::from("rewewrrwerwaman");

    match find_first_a(my_string) {
        CustomOption::Some(index) => println!("The letter is found at index {}", index),
        CustomOption::None => println!("The letter is not found")
    }
}