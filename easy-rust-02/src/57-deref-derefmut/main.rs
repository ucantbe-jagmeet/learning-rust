//Deref is the trait that lets you use * to dereference something.
// Deref is sometimes called a "smart pointer". A smart pointer can point to its item, has information about it, and can use its methods

/*
use std::ops::{Deref, DerefMut};

struct HoldsANumber(u8);
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    // let x = DerefExample { value: 'a' };
    // assert_eq!('a', *x);

    let my_number = HoldsANumber(20);
    println!("{:?}", my_number.checked_sub(100)); // This method comes from u8
    my_number.prints_the_number_times_two();
}

*/

use std::ops::Deref;

struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: i8,
    alignment: Alignment,
}

enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Character {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        health: u8,
        intelligence: u8,
        wisdom: u8,
        charm: u8,
        hit_points: i8,
        alignment: Alignment,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            health,
            intelligence,
            wisdom,
            charm,
            hit_points,
            alignment,
        }
    }
}

impl Deref for Character {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

fn main() {
    let billy = Character::new("Billy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good);
    let brandy = Character::new(
        "Brandy".to_string(),
        9,
        8,
        7,
        10,
        19,
        19,
        5,
        Alignment::Good,
    );

    let mut hit_points_vec = vec![];
    hit_points_vec.push(*billy);
    hit_points_vec.push(*brandy);

    println!("{:?}", hit_points_vec);
}
