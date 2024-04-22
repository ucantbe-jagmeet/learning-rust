// You use Option when you have a value that might exist, or might not exist. When a value exists it is Some(value) and when it doesn't it's just None, Here is an example of bad code that can be improved with Option.

// Result
// Option is about Some or None (value or no value),
// Result is about Ok or Err (okay result, or error result).

/*
    enum Option<T> {
        None,
        Some(T),
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

*/

// Remember, the four methods to easily check are .is_some(), is_none(), is_ok(), and is_err().

//Using a match with Option and Result sometimes requires a lot of code. For example, the .get() method returns an Option on a Vec.
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[5])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(num) => println!("Got a number: {}", num),
            None => println!("Got nothing"),
        }
    }
}

fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()),
    }
}
fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5, 6];
    // println!(
    //     "{:?} {:?}",
    //     take_fifth(new_vec.clone()),
    //     take_fifth(bigger_vec.clone())
    // );

    //  we can use .unwrap() to get the value inside Some(value) , but we can apply it on None
    //  so try it with match

    // let mut option_vec = Vec::new(); // Make a new vec to hold our options
    // The vec is type: Vec<Option<i32>>. That means a vec of Option<i32>.

    // option_vec.push(take_fifth(new_vec));
    // option_vec.push(take_fifth(bigger_vec));
    // handle_option(option_vec);

    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            // .is_some() returns true if we get Some, false if we get None
            println!("We got: {}", inside_number.unwrap()); // now it is safe to use .unwrap() because we already checked
        } else {
            println!("We got nothing.");
        }
    }

    if give_result(5).is_ok() {
        println!("It's Okay, guys");
    } else {
        println!("It's an error, guys");
    }

    let mut result_vec = Vec::new(); // Create a new vec for the results

    for number in 2..7 {
        result_vec.push(check_if_five(number)); // push each result into the vec
    }

    println!("{:?}", result_vec);

    // Using a match with Option and Result sometimes requires a lot of code. For example, the .get() method returns an Option on a Vec.

    let my_vec = vec![2, 3, 4];
    // let get_one = my_vec.get(0); // 0 to get the first number
    // let get_two = my_vec.get(10); // Returns None
    // println!("{:?}", get_one);
    // println!("{:?}", get_two);

    // for index in 0..10 {
    //     match my_vec.get(index) {
    //         Some(number) => println!("The number is: {}", number),
    //         None => {}
    //     }
    // }

    // This is good, but we don't do anything for None because we don't care. Here we can make the code smaller by using if let. if let means "do something if it matches, and don't do anything if it doesn't". if let is when you don't care about matching for everything.

    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}:", city[0]); // In our data, every first item is the city name
        while let Some(information) = city.pop() {
            // This means: keep going until you can't pop anymore
            // When the vector reaches 0 items, it will return None
            // and it will stop.
            if let Ok(number) = information.parse::<i32>() {
                // Try to parse the variable we called information
                // This returns a result. If it's Ok(number), it will print it
                println!("The number is: {}", number);
            } // We don't write anything here because we do nothing if we get an error. Throw them all away
        }
    }
}
