fn main(){
    let jagmeet = User {
        active: true,
        username: String ::from("jagmeet_singh"),
        email: String ::from("jagmeet@mail.com"),
        mobile_no: 8989898989
    };
    println!("{:#?}", jagmeet);
    
    // tuples
    let dog:(&str, &str, u64)= ("Bruno", "Mars", 10);
    
    println!("info ---- {}", dog.2); // we have to write number to get values from tupples
}

// custom data types - defined by user
#[derive(Debug)]
struct User{
    active:bool,
    username: String,
    email: String,
    mobile_no: u64,
}
