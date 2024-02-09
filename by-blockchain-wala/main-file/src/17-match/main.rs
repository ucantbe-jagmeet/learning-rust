enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents( coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => { 10},
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


    let xyz = Coin::Dime;
    let value = value_in_cents(xyz);
    println!("value is {}", value);
}