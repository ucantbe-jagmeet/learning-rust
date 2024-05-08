// refernce means giving the address of a string rather than the ownership of the string over to a function

//borrowing -> you can transfer the ownership of variables to a function,

fn main() {
    /*
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);
    */

    // let my_string = String::from("Hello, rust");
    // takes_ownership(&my_string);

    // println!("1 {}", my_string);

    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    update_word(&mut s1);
    update_word(&mut s1);
    update_word(&mut s1);
    println!("{}", s1);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
