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
