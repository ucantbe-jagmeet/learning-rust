fn main() {
    // let mut x: i8 = 10;
    // let y: u32 = 5;
    // let z: f32 = 1000.21;

    // println!("{}, {}, {}", x, y, z);

    // for _ in 0..10000 {
    //     // runtime logic
    //     x = x + 100;
    // }

    // println!("{}", x);

    // ######## boolean ############

    // let is_male = true;
    // let is_above_18 = true;

    // if is_male {
    //     println!("You are a male");
    // } else {
    //     println!("You are a female");
    // }

    // if is_male && is_above_18 {
    //     println!("You are a legal male");
    // }

    // ##### string ############

    // let axx = "ahkahkdkahdkasd";

    let greeting = String::from("Hello World");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(1000);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000"),
    }
}
