fn print_country(country_name: String) -> String {
    println!("country is {}", country_name);
    country_name
}

fn main() {
    let country = String::from("Australia");
    let country = print_country(country);
    print_country(country);
}
