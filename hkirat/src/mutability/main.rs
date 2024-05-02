/*
-> Immutable data is inheretly thread safe because if not thread can alter the data , then no synchronization is needed when data is accessed currently.
-> knowing that certain data will not change allows the compiler to optimize code better.

*/

fn main() {
    // let x = 1; // immutable -> can not be change
    let x = String::from("Hit here");
    x.push_str("ash");
    println!("{}", x);
}
