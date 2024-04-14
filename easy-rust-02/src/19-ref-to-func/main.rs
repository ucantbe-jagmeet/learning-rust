fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_country1(country_name: String) -> String {
    println!("{}", country_name);
    country_name // return it here
}

fn print_ref_country(country_name: &String) {
    println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) {
    // first we say that the function takes a mutable reference
    country_name.push_str("-Hungary"); // push_str() adds a &str to a String
    println!("Now it says: {}", country_name);
}

fn mut_hungary(mut country: String) {
    // Here's how: adds_hungary takes the String and declares it mutable!
    country.push_str("-Hungary");
    println!("{}", country);
}
fn main() {
    let country = String::from("Austria");
    print_country(country); // We print "Austria"
                            // print_country(country); // ownership lost here

    let country = print_country(country);
    print_country1(country);

    print_ref_country(&country); // We print "Austria"

    let mut country = String::from("Austria");
    add_hungary(&mut country);

    mut_hungary(country);
}

/*
    So to conclude:

    fn function_name(variable: String) takes a String and owns it. If it doesn't return anything, then the variable dies inside the function.
    fn function_name(variable: &String) borrows a String and can look at it
    fn function_name(variable: &mut String) borrows a String and can change it

*/
