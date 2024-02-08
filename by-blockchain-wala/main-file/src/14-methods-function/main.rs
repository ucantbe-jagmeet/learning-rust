fn main(){
    println!("methods and functions");
}

struct Aadhar{
    name: String,
    gender: String,
    address: String
}

// this is called methods
impl Aadhar{
    fn create_user(user: Aadhar){}
    fn update_user(user: Aadhar){}
}