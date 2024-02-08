fn main(){
    let mut s1 = String::from("hello");
    let len = calculate_len(&mut s1); // & sign means borrow/reference : means it doesnot giving its ownership 
    // there are some rules for borrowing 

    println!("The len of {} is {}", s1, len);
}

fn calculate_len(s: &mut String)-> usize{
    s.push_str(", world");
    s.len()
}