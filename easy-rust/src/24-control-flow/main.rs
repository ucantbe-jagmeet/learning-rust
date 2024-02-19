fn main() {
    let my_number = 2;

    // if my_number == 7 {
    //     println!("It's seven");
    // } else if my_number == 6 {
    //     println!("It's six");
    // } else {
    //     println!("It's a different number");
    // }

    // if my_number % 2 == 1 && my_number > 0 {
    //     println!("It's a positive odd number");
    // } else if my_number == 6 {
    //     println!("It's six");
    // } else {
    //     println!("It's a different number");
    // }

    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }

    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
}
