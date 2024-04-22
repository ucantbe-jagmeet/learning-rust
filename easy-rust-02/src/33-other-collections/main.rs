use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>,
}

fn main() {
    // let mut tallin = City {
    //     name: "Tallinn".to_string(),
    //     population: HashMap::new(),
    // };

    // tallin.population.insert(1372, 3_250);
    // tallin.population.insert(1851, 24_000);
    // tallin.population.insert(2020, 437_619);

    // for (year, population) in tallin.population {
    //     println!(
    //         "In the year {} the city of {} had a population of {}.",
    //         year, tallin.name, population
    //     );
    // }

    let canadian_cities = vec!["calgary", "vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "German");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));
}
