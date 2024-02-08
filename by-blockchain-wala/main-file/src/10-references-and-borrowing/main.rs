// there are some rules for borrowing 
// -> At any given time , you can have either one mut reference or any number of immutable reference
// -> References must always be valid


fn main(){
    let mut s1 = String::from("hello");
    print_hellow(&mut s1);
    let len = calculate_len(&mut s1); // & sign means borrow/reference : means it doesnot giving its ownership 
    println!("The len of {} is {}", s1, len);
    
    // if one variable is mutable reference in single scope then it will not provide mut or immut reference in same scope 
    // for eg
    
    // let s = &mut s1;
    // let s2 = &mut s1; // this will give doubel mut borrowing error
    // let s2 = &s1; // this will give immut borrow occurs error
    // let len = calculate_len(&mut s1, &mut s1);  

    // println!("The len of {} is {}", s1, len);
}

// fn calculate_len(s: &mut String, p : &mut String)-> usize{
fn calculate_len(s: &mut String)-> usize{
    s.push_str(", world");
    s.len()
}

fn print_hellow(s: &mut String){
    s.push_str(", ok");
    println!("{}", s);
}