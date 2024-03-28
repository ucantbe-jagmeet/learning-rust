// If you use multiple threads, you can do many things at the same time. Modern computers have more than one core so they can do more than one thing at the same time, and Rust lets you use them. Rust uses threads that are called "OS threads". OS thread means the operating system creates the thread on a different core. (Some other languages use "green threads", which are less powerful)

// You create threads with std::thread::spawn and then a closure to tell it what to do. Threads are interesting because they run at the same time, and you can test it to see what happens

/*
    FnOnce: takes the whole value
    FnMut: takes a mutable reference
    Fn: takes a regular reference
*/
fn main() {
    // for _ in 0..10 {
    //     // set up ten threads
    //     std::thread::spawn(|| {
    //         println!("I am printing something");
    //     });
    // } // now the thread starts
    // this loop will not finish before closing main
    // so we have to create a JoinHandle
    // the better way is to bind the threads to a variable. If you add let, then you will create a JoinHandle. You can see this in the signature for spawn

    /*
    for _ in 0..10 {
        // set up ten threads
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        handle.join();
    }
    */

    /*
    let mut my_string = String::from("I will go into the closure");
    // let my_closure = || println!("{}", my_string); // String is not Copy, so my_closure() is Fn: it takes a reference.
    // If we change my_string, it will be FnMut.
    let mut my_closure = || {
        my_string.push_str(" now");
        println!("{}", my_string);
    };
    my_closure();
    my_closure();
    */

    //And if you take by value, then it will be FnOnce.

    /*
    let my_vec: Vec<i32> = vec![8, 9, 10];
    let my_closure = || {
        my_vec
            .into_iter() // into_iter takes ownership
            .map(|x| x as u8) // turn it into u8
            .map(|x| x * 2) // multiply by 2
            .collect::<Vec<u8>>() // collect into a Vec
    };
    let new_vec = my_closure();
    println!("{:?}", new_vec);
    */

    let mut my_string = String::from("Can I go inside the thread?");

    let handle = std::thread::spawn(move || {
        println!("{}", my_string); // now my_string is being used as a reference
    });

    // std::mem::drop(my_string); // ⚠️ We try to drop it here. But the thread still needs it.

    handle.join().unwrap()
}
