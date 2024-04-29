use rand::seq::SliceRandom; // Use this for .choose over slices
use std::fmt::Display;

fn print_and_return_thing<T: Display>(input: T) -> T {
    println!("You gave me {} and now I will give it back.", input);
    input
}

fn main() {
    // let my_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    // let mut rng = rand::thread_rng();
    // for _ in 0..6 {
    //     print!("{} ", my_letters.choose(&mut rng).unwrap());
    // }

    let my_name = print_and_return_thing("Windy");
    let small_number = print_and_return_thing(9.0);
}
