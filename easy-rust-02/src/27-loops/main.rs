fn main() {
    let mut counter = 0; // set a counter to 0
    loop {
        counter += 1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 {
            // stop when counter == 5
            break;
        }
    }
}
