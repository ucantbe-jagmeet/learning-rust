fn main(){
    let jagmeet = User {
        active: true,
        username: String ::from("jagmeet_singh"),
        email: String ::from("jagmeet@mail.com"),
        mobile_no: 8989898989
    };
    println!("{:#?}", jagmeet)
}

// custom data types - defined by user
#[derive(Debug)]
struct User{
    active:bool,
    username: String,
    email: String,
    mobile_no: u64,
}
