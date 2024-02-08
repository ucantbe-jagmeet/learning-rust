fn main(){
    let mut s: String = String::from("hello world");
    let x = first_word(&s); // x will get the value of 5    
    s.clear();

    println!("{}", x); // x still shows 5 which is odd behaviour


}

// usize depend on system for example 32 bit , 64 bit

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("{:?}", bytes); // :? is required to print bytes 

    // iter means to go on every element in array 

    for ( i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}