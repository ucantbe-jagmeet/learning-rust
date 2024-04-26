// Rc means "reference counter". You know that in Rust, every variable can only have one owner. That is why this doesn't work:

// we have to use .clone() , it does not means the clone of the whole string , cloning of the Rc just clone the initial pointer

// fn takes_a_string(input: String) {
// println!("It is: {}", input)
// }

// fn also_takes_a_string(input: String) {
// println!("It is: {}", input)
// }

// fn main() {
// let user_name = String::from("User MacUserson");
// takes_a_string(user_name);
// also_takes_a_string(user_name); // error here

// After takes_a_string takes user_name, you can't use it anymore. Here that is no problem: you can just give it user_name.clone(). But sometimes a variable is part of a struct, and maybe you can't clone the struct. Or maybe the String is really long and you don't want to clone it. These are some reasons for Rc, which lets you have more than one owner. An Rc is like a good office worker: Rc writes down who has ownership, and how many. Then once the number of owners goes down to 0, the variable can disappear.
// }
use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    // city_history: String,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

fn main() {
    // let calgary = City {
    //     name: "Calgary".to_string(),
    //     population: 1_200_000, // pretend that this string is very very long
    //     city_history: "Calgary began as a fort called fort calgary that...".to_string(),
    // };

    // let canada_cities = CityData {
    //     names: vec![calgary.name], // this is using calgary.name , which is short
    //     histories: vec![calgary.city_history], // this String is very long
    // };

    // println!("Calgary's history is: {}", calgary.city_history); // Of course, it doesn't work because canada_cities now owns the data and calgary doesn't

    //We can clone the name: names: vec![calgary.name.clone()] but we don't want to clone the city_history, which is long. So we can use an Rc.

    /*
        To add a new reference, you have to clone the Rc. But hold on, didn't we want to avoid using .clone()? Not exactly: we didn't want to clone the whole String. But a clone of an Rc just clones the pointer - it's basically free. It's like putting a name sticker on a box of books to show that two people own it, instead of making a whole new box.

    You can clone an Rc called item with item.clone() or with Rc::clone(&item). So calgary.city_history has 2 owners. We can check the number of owners with Rc::strong_count(&item)
         */

    //########################################################
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // Pretend that this string is very very long
        city_history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()), // Rc::new() to make the Rc
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.city_history.clone()], // .clone() to increase the count
    };

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history)); // this prints 2
    println!("{}", Rc::weak_count(&calgary.city_history));
    let new_owner = calgary.city_history.clone();
    println!("{}", Rc::strong_count(&calgary.city_history)); // this prints 3

    // note -> So if there are strong pointers, are there weak pointers? Yes, there are. Weak pointers are useful because if two Rcs point at each other, they can't die. This is called a "reference cycle". If item 1 has an Rc to item 2, and item 2 has an Rc to item 1, they can't get to 0. In this case you want to use weak references. Then Rc will count the references, but if it only has weak references then it can die. You use Rc::downgrade(&item) instead of Rc::clone(&item) to make weak references. Also, you use Rc::weak_count(&item) to see the weak count.
}
