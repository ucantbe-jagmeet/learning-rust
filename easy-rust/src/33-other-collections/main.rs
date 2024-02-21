// HashMap -> A HashMap is a collection made out of keys and values. You use the key to look up the value that matches the key. You can create a new HashMap with just HashMap::new() and use .insert(key, value) to insert items.

// If you want a HashMap that you can sort, you can use a BTreeMap

// If a HashMap already has a key when you try to put it in, it will overwrite its value:
use std::collections::hash_map::*;
use std::collections::{BTreeMap, HashMap};

struct City {
    name: String,
    // population: HashMap<u32, u32>,
    population: BTreeMap<u32, u32>,
}

fn main() {
    // let mut tallinn = City {
    //     name: "Tallinn".to_string(),
    //     // population: HashMap::new(),
    //     population: BTreeMap::new(),
    // };

    // tallinn.population.insert(1372, 3_250); // insert three dates
    // tallinn.population.insert(1851, 24_000);
    // tallinn.population.insert(2020, 437_619);

    // for (year, population) in tallinn.population {
    //     println!(
    //         "In the year {} the city of {} had a population of {}.",
    //         year, tallinn.name, population
    //     );
    // };

    // let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    // let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    // let mut city_hashmap = HashMap::new();

    // for city in canadian_cities {
    //     city_hashmap.insert(city, "Canada");
    // }
    // for city in german_cities {
    //     city_hashmap.insert(city, "Germany");
    // }

    // If you are not sure that there will be a key, you can use .get() which returns an Option. If it exists it will be Some(value), and if not you will get None instead of crashing the program. That's why .get() is the safer way to get a value from a HashMap.

    // println!("{:?}", city_hashmap["Bielefeld"]);
    // println!("{:?}", city_hashmap.get("Bielefeld"));
    // println!("{:?}", city_hashmap.get("Bielefeldd"));

    // let mut book_hashmap = HashMap::new();

    // book_hashmap.insert(1, "L'Allemagne Moderne");

    // if book_hashmap.get(&1).is_none() {
    //     // is_none() returns a bool: true if it's None, false if it's Some
    //     book_hashmap.insert(1, "Le Petit Prince");
    // }

    // println!("{:?}", book_hashmap.get(&1));

    // let bool_collection = vec![
    //     "L'Allemagne Moderne",
    //     "Le Petit Prince",
    //     "Eye of the World",
    //     "Eye of the World",
    // ]; // Eye of the World appears twice

    // let mut book_hashmap = HashMap::new();
    // // HashMap has a very interesting method called .entry() that you definitely want to try out. With it you can try to make an entry and use another method like .or_insert() to insert the value if there is no key. The interesting part is that it also gives a mutable reference so you can change it if you want. First is an example where we just insert true every time we insert a book title into the HashMap.
    // for book in bool_collection {
    //     book_hashmap.entry(book).or_insert(true);
    // }

    // for (book, true_or_false) in book_hashmap {
    //     println!("Do we have {}? {}", book, true_or_false);
    // }

    // let mut book_hashmap = HashMap::new();

    // book_hashmap.insert(1, "L'Allemagne Moderne");
    // book_hashmap.insert(1, "Le Petit Prince");
    // book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    // book_hashmap.insert(1, "Eye of the World");

    // if book_hashmap.get(&1).is_none() {
    //     // is_none() returns a bool: true if it's None, false if it's Some
    //     book_hashmap.insert(1, "Le Petit Prince");
    // }

    // println!("{:?}", book_hashmap.get(&1));

    //#################### Entry #############################
    /*
        enum Entry<K, V> {
        Occupied(OccupiedEntry<K, V>),
        Vacant(VacantEntry<K, V>),
    }
        fn or_insert(self, default: V) -> &mut V {
               // 🚧
            match self {
                Occupied(entry) => entry.into_mut(),
                Vacant(entry) => entry.insert(default),
            }
        }
        */

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0); // return_value is a mutable reference. If nothing is there, it will be 0
        *return_value += 1; // Now return_value is at least 1. And if there was another book, it will go up by 1
    }

    for book in book_hashmap {
        println!("{:?}", book);
    }
}
