// Tuples in Rust use (). We have seen many empty tuples already, because nothing in a function actually means an empty tuple

// When you don't return anything in a function, you actually return an empty tuple.

fn main() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )

    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // call them a, b, and c
    println!("{:?}", b);


    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
}
