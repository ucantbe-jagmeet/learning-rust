//  channel is an easy way to use many threads that send to one place. They are fairly popular because they are pretty simple to put together. You can create a channel in Rust with std::sync::mpsc. mpsc means "multiple producer, single consumer", so "many threads sending to one place". To start a channel, you use channel(). This creates a Sender and a Receiver that are tied together.

use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    /*
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();

    sender.send(5).unwrap();
    println!("{}", receiver.recv().unwrap());
     */

    /*
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::spawn(move || {
        // move sender in
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::spawn(move || {
        // move sender_clone in
        sender_clone.send("And here is another &str").unwrap();
    });

    println!("{}", receiver.recv().unwrap());
    */

    // The two threads start sending, and then we println!. It might say Send a &str this time or And here is another &str, depending on which thread finished first. Let's make a join handle to make them wait.

    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    let mut handle_vec = vec![]; // Put our handles in here
    let mut results_vec = vec![];

    handle_vec.push(std::thread::spawn(move || {
        // push this into the vec
        sender.send("Send a &str this time").unwrap();
    }));

    handle_vec.push(std::thread::spawn(move || {
        // and push this into the vec
        sender_clone.send("And here is another &str").unwrap();
    }));

    for _ in handle_vec {
        // now handle_vec has 2 items. Let's print them
        results_vec.push(receiver.recv().unwrap());
    }

    println!("{:?}", results_vec);
}
