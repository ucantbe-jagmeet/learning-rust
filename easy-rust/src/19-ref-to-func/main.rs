fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_country1(country_name: String) -> String {
    println!("{}", country_name);
    country_name // return it here
}

fn main() {
    let country = String::from("Austria");
    print_country(country); // We print "Austria"
    // print_country(country); // ownership lost here
    
    let country = print_country(country);
    print_country1(country); 
}
