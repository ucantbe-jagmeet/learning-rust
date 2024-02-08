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
    // println!("{:#?}", new_user); // we cann not print new_user with user2 because user2 is using username from new_user so instead of copying its data it is taking ownership from new_user 

    // we can print both structs if we can assign username by ourself after email
    println!("{:#?}", user2);
}

// custom data types - defined by user
// owner of the struct data types is decided by the data types used in the struct
// the variable initiates the struct object is the owner 
#[derive(Debug)]
struct User{
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// reference type struct -- this will create issue/ errors 
// struct User{
//     active:bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count:1
    }
}