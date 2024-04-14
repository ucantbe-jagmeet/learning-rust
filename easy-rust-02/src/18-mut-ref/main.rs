fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10; // Use * to change the i32 value.
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!(
        "Second_number = triple_reference? {}",
        second_number == ***triple_reference
    );

    // ##################################
    let mut number = 10;
    let number_change = &mut number; // create a mutable reference
    *number_change += 15; // use mutable reference to add 10
    let number_ref = &number; // create an immutable reference
    println!("{}", number_ref); // print the immutable reference

    // ###########################################

    let country = String::from("Austria"); // Now we have a String called country
    let country_ref = &country; // country_ref is a reference to this data. It's not going to change
    let country = 8; // Now we have a variable called country that is an i8. But it has no relation to the other one, or to country_ref
    println!("{}, {}", country_ref, country); // country_ref still refers to the data of String::from("Austria") that we gave it.
}
