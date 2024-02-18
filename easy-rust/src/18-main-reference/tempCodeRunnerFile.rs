fn main() {
    let mut my_number = 8; // don't forget to write mut here!
    let num_ref = &mut my_number;
    my_number=10;

    println!("nunm: {}", num_ref);
}
