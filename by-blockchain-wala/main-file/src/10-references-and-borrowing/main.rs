fn main(){
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // & reference sign means borrow 

    println!("The len of {} is {}", s1, len);
}

fn calculate_len(s: &String)-> usize{
    s.len()
}