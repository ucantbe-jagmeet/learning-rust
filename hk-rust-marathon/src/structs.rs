#[warn(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let _user1 = User{ 
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@123"),
        sign_in_count: 1
    };

    // println!("{:?}", user1.username);

    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!("The area of the reactangle is {}", rect.area())
}