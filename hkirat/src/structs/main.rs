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

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Rect {{ width: {}, height: {}, area: {} }}",
            self.width,
            self.height,
            self.width * self.height
        )
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("ucantbe-jagmeet"),
        email: String::from("jagmeet@gmail.com"),
        sign_in_count: 1,
    };

    // print!("User 1 username : {:?}", user1.username);

    let rect = Rect {
        width: 30,
        height: 23,
    };

    println!("Area of the rectangle: {:?}", rect);
}
