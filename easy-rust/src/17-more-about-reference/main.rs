fn return_str() -> &str {
    let country = String::from("Australia");
    let country_ref = &country;
    country_ref  
}

fn main() {
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_two);

    //  let country1 = return_str(); // this function will give error because we cant return address of a variable . Rust cleans the memory of the variable when it goes out of scope.
}
