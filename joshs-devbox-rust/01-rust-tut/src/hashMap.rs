use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "h1");
    map.insert(1, "h2");

    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't Exist"),
    }
    match map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't Exist"),
    }

    map.remove(&0);
    println!("{:?}", map);
}
