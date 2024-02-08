fn main(){
    
    let jagmeet = User {
        active: true,
        username: String ::from("jagmeet_singh"),
        email: String ::from("jagmeet@mail.com"),
        sign_in_count: 9999
    };
    // println!("{:#?}", jagmeet);
    // tuples
    let dog:(&str, &str, u64) = ("Bruno", "Mars", 10);
    
    // println!("info ---- {}", dog.2); // we have to write number to get values from tuples

    // #########################
    let new_user = build_user(String::from("jag@gmail.com"), String::from("ucantbe"));

    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..new_user
    };
    // println!("{:#?}", new_user);
    println!("{:#?}", user2);
}

// custom data types - defined by user
#[derive(Debug)]
struct User{
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count:1
    }
}