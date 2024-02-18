fn main() {
    let array1 = ["One", "Two"]; // This one is type [&str; 2]
    let array2 = ["One", "Two", "Five"]; // But this one is type [&str; 3]. Different type!

    let my_array = ["a"; 10];
    // println!("{:?}", my_array); //This prints ["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"].

    let my_numbers = [0, 10, -20];
    // println!("{}", my_numbers[1]); // prints 10

    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!(
        "Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
