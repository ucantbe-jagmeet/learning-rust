// HashMap -> A HashMap is a collection made out of keys and values. You use the key to look up the value that matches the key. You can create a new HashMap with just HashMap::new() and use .insert(key, value) to insert items.

// If you want a HashMap that you can sort, you can use a BTreeMap

// If a HashMap already has a key when you try to put it in, it will overwrite its value:

// A HashSet is actually a HashMap that only has keys . A hash set implemented as a HashMap where the value is (). So it's a HashMap with keys, no values.

// You often use a HashSet if you just want to know if a key exists, or doesn't exist.
use std::collections::hash_map::*;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

struct City {
    name: String,
    // population: HashMap<u32, u32>,
    population: BTreeMap<u32, u32>,
}

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
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

    // let book_collection = vec![
    //     "L'Allemagne Moderne",
    //     "Le Petit Prince",
    //     "Eye of the World",
    //     "Eye of the World",
    // ];

    // let mut book_hashmap = HashMap::new();

    // for book in book_collection {
    //     let return_value = book_hashmap.entry(book).or_insert(0); // return_value is a mutable reference. If nothing is there, it will be 0
    //     *return_value += 1; // Now return_value is at least 1. And if there was another book, it will go up by 1
    // }

    // for book in book_hashmap {
    //     println!("{:?}", book);
    // }

    //#################### use of entry #############################
    // let data = vec![
    //     // This is the raw data
    //     ("male", 9),
    //     ("female", 5),
    //     ("male", 0),
    //     ("female", 6),
    //     ("female", 5),
    //     ("male", 10),
    // ];

    // let mut survey_hash = HashMap::new();

    // for item in data {
    //     // This gives a tuple of (&str, i32)
    //     survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1); // This pushes the number into the Vec inside
    // }

    // for (male_or_female, numbers) in survey_hash {
    //     println!("{:?}: {:?}", male_or_female, numbers);
    // }

    // ####################### Hashset ###############################

    /*
        let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!(
        "There are {} unique numbers, so we are missing {}.",
        hashset_length,
        100 - hashset_length
    );

    let mut missing_vec = vec![];

    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    println!("It does not contain");
    for number in missing_vec {
        print!("{} ", number);
    }
     */

    // ################## Binary heap #########################
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // These numbers are in order

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        // .pop() returns Some(number) if a number is there, None if not. It pops from the front
        println!(
            "Popped off {}. Remaining numbers are: {:?}",
            number,
            show_remainder(&my_heap)
        );
    }
}
