enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}
enum Shape{
    Circle( f64),
    Rectangle( f64, f64),
    Triangle( f64, f64, f64)
}

fn value_in_cents( coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn main(){
    let mut number: i32 = 13;
    number = 2;

    match number {
        1 => println!("The number is one !"),
        2 => println!("The number is two !"),
        3 => println!("The number is three !"),
        _ => println!("The number is something else !")
    }


    let xyz = Coin::Nickel;
    let value = value_in_cents(xyz);
    println!("value is {}", value);


    // ####### Shape Example #######
    let shape = Shape::Circle( 144.0);

    match shape {
        Shape :: Circle(radius) => {
            let area = 3.14 * radius * radius;
            println!("It's a circle with radius {} and {}", radius, area);
        },
        Shape :: Rectangle( width, height ) => {
            println!("It's a Rectangle whose width and height are {} and {}", width, height);
        },
        Shape :: Triangle(s1, s2, s3) => {
            println!("It's a Triangle whose sides are {}, {} and {}", s1, s2, s3);
        }
    }



}