fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1); // Now it knows: it's Vec<String>
    my_vec.push(name2);
}
