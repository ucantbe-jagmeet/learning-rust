// We have seen traits before: Debug, Copy, Clone are all traits. To give a type a trait, you have to implement it.

//The important thing to remember about traits is that they are about behaviour. To make a trait, write trait and then create some functions.

/*
#[derive(Debug)]
struct Animal {
    // simple struct an animal only has a name
    name: String,
}

trait Dog {
    // a dog trait gives some functionality
    fn bark(&self);
    fn run(&self);
}

impl Dog for Animal {
    // now Animal has the trait dog
    fn run(&self) {
        println!("{} stop barking!", self.name);
    }
    fn bark(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
*/
/*
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("Mr. Mantle is a {:?}", mr_mantle)
}
*/

use std::fmt;

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}
fn print_cats(pet: String) {
    println!("{}", pet);
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    print_cats(mr_mantle.to_string());
    println!(
        "Mr. mantle's string is {} letters long",
        mr_mantle.to_string().chars().count()
    );
}
