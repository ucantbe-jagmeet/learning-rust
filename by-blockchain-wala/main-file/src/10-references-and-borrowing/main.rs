fn main(){
    let mut s1 = String::from("hello");
    // let len = calculate_len(s1);
    let len ;
    (len, s1) = calculate_len(s1);

    println!("The len of {} is {}", s1, len);
}

fn calculate_len(s:String)->( usize, String){
    (s.len(), s)
}