fn main(){
    let mut s: String = String::from("hello world this is slice type");
    // let x = first_word(&s); // x will get the value of 5    
    // s.clear();
    
    // println!("{}", x); // x still shows 5 which is odd behaviour
    
    //  ############### using slice ###############
    
    // let hello = &s[0..5];
    // let world = &s[6..11];
    
    // println!("{world}")
    
    let first = first_word(&s);
    
    // s.clear(); // now this gives error that "mutable borrow occurs here" , this is the correct way 
    println!{"first World{} is: {}", s, first}

}

// usize depend on system for example 32 bit , 64 bit

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // println!("{:?}", bytes); // :? is required to print bytes 

    // iter means to go on every element in array 

    for ( i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i]
        }
    }
    &s[.. s.len()]
    
}