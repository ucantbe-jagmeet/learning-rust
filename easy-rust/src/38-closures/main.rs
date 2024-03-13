// Closures are like quick functions that don't need a name. Sometimes they are called lambdas
use std::collections::HashMap;

#[derive(Debug)]
struct Names {
    one_word: Vec<String>,
    two_words: Vec<String>,
    three_words: Vec<String>,
}
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

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

    /*
    let number_word_hashmap = some_numbers
        .into_iter() // now it is an iter
        .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together.
        .collect::<HashMap<_, _>>();

    println!(
        "For key {} we get {}.",
        2,
        number_word_hashmap.get(&2).unwrap()
    );
    */

    // ####### |_| in a closure ###########
    /*
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
    */

    /*
        let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
        let all_the_ceos = company_vec
            .into_iter()
            .filter_map(|company| company.get_ceo())
            .collect::<Vec<String>>();

        println!("{:?}", all_the_ceos);
    */

    /*
        Since .filter_map() needs an Option, what about Result? No problem: there is a method called .ok() that turns Result into Option. It is called .ok() because all it can send is the Ok result (the Err information is gone). You remember that Option is Option<T> while Result is Result<T, E> with information for both Ok and Err. So when you use .ok(), any Err information is lost and it becomes None.
    */

    /*
    let new_vec = vec![8, 9, 0]; // just a vec with numbers

    let number_to_add = 5; // use this in the math later
    let mut empty_vec = vec![]; // results go in here

    for index in 0..5 {
        empty_vec.push(
            new_vec
                .get(index) // getting the index
                .and_then(|number| Some(number + 1)) // number geting increamented
                .and_then(|number| Some(number + number_to_add)), // adding 5 to each and pushed in empty_vec
        );
    }
    println!("{:?}", empty_vec);
    */

    /*
     let first_try = vec![
         Some("success!"),
         None,
         Some("success!"),
         Some("success!"),
         None,
     ];
     let second_try = vec![
         None,
         Some("success!"),
         Some("success!"),
         Some("success!"),
         Some("success!"),
     ];
     let third_try = vec![
         Some("success!"),
         Some("success!"),
         Some("success!"),
         Some("success!"),
         None,
     ];

     for i in 0..first_try.len() {
         println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
     }
    */
    /*
     let mut big_vec = vec![6; 1000];
     big_vec.push(5);

     let mut iterator = big_vec.iter().rev();
     println!("{:?}", iterator.next());
     println!("{:?}", iterator.next());
    */

    // We were right: there is one Some(5) and then the 1000 Some(6) start. So we can write this:
    /*
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));
    */

    // With .cycle() you can create an iterator that loops forever. This type of iterator works well with .zip() to create something new, like this example which creates a Vec<(i32, &str)>:

    /*
    let even_odd = vec!["even", "odd"];

    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);
     */

    /*
    let a_string = "I don't have any dashes in me.";

    println!(
        "{}",
        a_string
            .chars() // Now it's an iterator
            .fold("-".to_string(), |mut string_so_far, next_char| { // Start with a String "-". Bring it in as mutable each time along with the next char
                string_so_far.push(next_char); // Push the char on, then '-'
                string_so_far.push('-');
                string_so_far} // Don't forget to pass it on to the next loop
            ));
     */

    /*
    let vec_of_names = vec![
        "Caesar",
        "Frodo Baggins",
        "Bilbo Baggins",
        "Jean-Luc Picard",
        "Data",
        "Rand Al'Thor",
        "Paul Atreides",
        "Barack Hussein Obama",
        "Bill Jefferson Clinton",
    ];

    let mut iter_of_names = vec_of_names.iter().peekable();

    let mut all_names = Names { // start an empty Names struct
        one_word: vec![],
        two_words: vec![],
        three_words: vec![],
    };

    while iter_of_names.peek().is_some() {
        let next_item = iter_of_names.next().unwrap(); // We can use .unwrap() because we know it is Some
        match next_item.match_indices(' ').collect::<Vec<_>>().len() { // Create a quick vec using .match_indices and check the length
            0 => all_names.one_word.push(next_item.to_string()),
            1 => all_names.two_words.push(next_item.to_string()),
            _ => all_names.three_words.push(next_item.to_string()),
        }
    }

    println!("{:?}", all_names);
     */
}
