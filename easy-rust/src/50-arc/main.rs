//Arc means "atomic reference counter". Atomic means that it uses the computer's processor so that data only gets written once each time. This is important because if two threads write data at the same time, you will get the wrong result
use std::sync::{Arc, Mutex};
fn main() {
    /*
    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1 is working!")
        }
    });

    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working!")
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exiting the program");
    */

    /*
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number); // This clone goes into Thread 1
    let my_number2 = Arc::clone(&my_number); // This clone goes into Thread 2
    */

    /*
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || {
        // Only the clone goes into Thread 1
        for _ in 0..10 {
            *my_number1.lock().unwrap() += 1; // Lock the Mutex, change the value
        }
    });

    let thread2 = std::thread::spawn(move || {
        // Only the clone goes into Thread 2
        for _ in 0..10 {
            *my_number2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {:?}", my_number.lock().unwrap());
    println!("Exiting the program");
    */

    // Then we can join the two threads together in a single for loop, and make the code smaller.

    // We need to save the handles so we can call .join() on each one outside of the loop. If we do this inside the loop, it will wait for the first thread to finish before starting the new one.

    /*
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![]; // JoinHandles will go in here

    for _ in 0..2 { // do this twice
        let my_number_clone = Arc::clone(&my_number); // Make the clone before starting the thread
        let handle = std::thread::spawn(move || { // Put the clone in
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle); // save the handle so we can call join on it outside of the loop
                                 // If we don't push it in the vec, it will just die here
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // call join on all handles
    println!("{:?}", my_number);
    */
}
