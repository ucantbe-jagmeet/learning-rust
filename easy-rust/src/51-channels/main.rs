//  channel is an easy way to use many threads that send to one place. They are fairly popular because they are pretty simple to put together. You can create a channel in Rust with std::sync::mpsc. mpsc means "multiple producer, single consumer", so "many threads sending to one place". To start a channel, you use channel(). This creates a Sender and a Receiver that are tied together.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;

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

    /*
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
     */

    let (sender, receiver) = channel();
    let hugevec = vec![0; 1_000_000];
    let mut newvec = vec![];
    let mut handle_vec = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();
        let mut work: Vec<u8> = Vec::with_capacity(hugevec.len() / 10); // new vec to put the work in. 1/10th the size
        work.extend(&hugevec[i * 100_000..(i + 1) * 100_000]); // first part gets 0..100_000, next gets 100_000..200_000, etc.

        let handle = spawn(move || {
            for number in work.iter_mut() {
                // do the actual work
                *number += 1;
            }

            sender_clone.send(work).unwrap(); // use the sender_clone to send the work to the receiver
        });
        handle_vec.push(handle);
    }

    for handle in handle_vec {
        // stop untill thread are done
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        newvec.push(results); // push the results from receiver.recv() into the vec
    }

    // now we have a Vec< Vec<u8>>, to put it together we can use .flatten()
    let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>();

    println!(
        "{:#?}, {:#?}, total length: {}",
        &newvec[0..10],
        &newvec[newvec.len() - 10..newvec.len()],
        newvec.len(),
    );

    for number in newvec {
        if number != 1 {
            panic!();
        }
    }
}
