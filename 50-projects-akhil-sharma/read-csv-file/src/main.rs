use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {}

fn main() {
    if let Err(e) = read_from_file("../customers.csv") {
        eprintln!("{}", e);
    }
}
