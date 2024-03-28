//  channel is an easy way to use many threads that send to one place. They are fairly popular because they are pretty simple to put together. You can create a channel in Rust with std::sync::mpsc. mpsc means "multiple producer, single consumer", so "many threads sending to one place". To start a channel, you use channel(). This creates a Sender and a Receiver that are tied together.

use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    /*
     let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();

    sender.send(5).unwrap();
    println!("{}", receiver.recv().unwrap());
     */
}
