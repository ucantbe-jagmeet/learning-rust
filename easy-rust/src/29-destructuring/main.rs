struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

struct City {
    name: String,
    country: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, country: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            country,
            population,
            date_founded,
        }
    }
}

fn process_city_values(city: &City) {
    let City {
        name,
        country,
        population,
        date_founded,
    } = city;

    let two_names = vec![name, country];
    println!("The city {} is in {} country", two_names[0], two_names[1]);
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 180,
        happiness: false,
    };

    let Person {
        // destructure papa_doc
        name: a,
        real_name: b,
        height: c,
        happiness: d,
    } = papa_doc;

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );

    let tallinn = City::new("Tallinn".to_string(), "Estonia".to_string(), 426_444, 1223);
    process_city_values(&tallinn);
}
