#[derive(Debug)]

struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle{
    // the first parameter of function in methods will be always secure for self
    // (self : Self )
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn peri(self : &Self) -> u32{
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    // returning output as a self
    fn square(size : u32) -> Self {
        Self {
            width : size, 
            height : size 
        }
    }
}

fn main(){

    let rect1 = Rectangle {
        width : 15,
        height: 35
    };

    let rect2 = Rectangle{
        width: 12,
        height: 34
    };

    println!(
        "The area of rectangle is : {}",
        rect1.area()
    );
    println!(
        "The perimeter of rectangle is : {}",
        rect1.peri()
    );
    println!(
        "can hold rect2 ? {}",
        rect1.can_hold(&rect2)
    );

    let new_square = Rectangle::square(10);

    println!(
        "Area of square is: {}",
        new_square.area()
    )
}
