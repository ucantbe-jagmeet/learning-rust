// Closures are like quick functions that don't need a name. Sometimes they are called lambdas
use std::collections::HashMap;
fn main() {
    /*
        let my_closure = |x: i32| println!("{}", x);
        my_closure(5);
        my_closure(5+5);
    */
    /*

    let number_one = 6;
    let number_two = 10;

    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);
    */

    /*
    let my_vec = vec![8, 9, 10];

    let fourth = my_vec.get(3).unwrap_or_else(|| {
        // try to unwrap. If it doesn't work,
        if my_vec.get(0).is_some() {
            // see if my_vec has something at index [0]
            &my_vec[0] // Give the number at index 0 if there is something
        } else {
            &0 // otherwise give a &0
        }
    });

    println!("{}", fourth);
     */

    /*
    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec
    .iter() // iter over it
    .map(|number| number * 2) // for each item, multiply by two
    .collect::<Vec<i32>>(); // then make a new vec from this

    println!("{:?}", double_vec);
    */

    /*
    let num_vec = vec![10, 9, 8];

    num_vec
        .iter() // iterate over num_vec
        .enumerate() // get (index, number)
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number));
    // do something for each one

    // this prints
    // Index number 0 has number 10
    // Index number 1 has number 9
    // Index number 2 has number 8 */

    // In this case we use for_each instead of map. map is for doing something to each item and passing it on, and for_each is doing something when you see each item. Also, map doesn't do anything unless you use a method like collect.

    //Actually, this is the interesting thing about iterators. If you try to map without a method like collect, the compiler will tell you that it doesn't do anything. It won't panic, but the compiler will tell you that you didn't do anything.

    // let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    // let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>

    // let number_word_hashmap = some_numbers
    //     .into_iter() // now it is an iter
    //     .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together.
    //     .collect::<HashMap<_, _>>();

    // println!(
    //     "For key {} we get {}.",
    //     2,
    //     number_word_hashmap.get(&2).unwrap()
    // );

    // ####### |_| in a closure ###########

    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let filtered_months = months
        .into_iter() // make an iter
        .filter(|month| month.len() < 5) // We don't want months more than 5 bytes in length.
        // We know that each letter is one byte so .len() is fine
        .filter(|month| month.contains("u")) // Also we only like months with the letter u
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);
}
