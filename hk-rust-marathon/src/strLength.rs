fn main(){
    let my_string = String::from("Hello World!");
    println!("The Number of characters in the string is: {}", get_string_length(&my_string));
}

fn get_string_length(s: &str) -> usize  {
    s.chars().count()
}

// difference between &str (atstr) and String is