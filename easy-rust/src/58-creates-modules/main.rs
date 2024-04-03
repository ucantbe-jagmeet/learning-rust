// Every time you write code in Rust, you are writing it in a crate. A crate is the file, or files, that go together for your code. Inside the file you write you can also make a mod. A mod is a space for functions, structs, etc. and is used for a few reasons:
// ->  Building your code: it helps you think about the general structure of your code. This can be important as your code gets larger and larger.
// -> Reading your code: people can understand your code more easily. For example, the name std::collections::HashMap tells you that it's in std inside the module collections. This gives you a hint that maybe there are more collection types inside collections that you can try.
// -> Privacy: everything starts out as private. That lets you keep users from using functions directly.

// mod print_things {
//     use std::fmt::Display;

//     pub fn prints_one_thing<T: Display>(input: T) {
//         // Print anything that implements Display
//         println!("{}", input)
//     }
// }
// You can see that we wrote use std::fmt::Display; inside print_things, because it is a separate space. If you wrote use std::fmt::Display; inside main() it wouldn't help. Also, we can't call it from main() right now. Without the pub keyword in front of fn it will stay private

/*
pub for a struct: it makes the struct public, but the items are not public. To make an item public, you have to write pub for each one too.
pub for an enum or trait: everything becomes public. This makes sense because traits are about giving the same behaviour to something. And enums are about choosing between items, and you need to see them all to choose them.
pub for a module: a top level module will be pub because if it isn't pub then nobody can touch anything in it at all. But modules inside modules need pub to be public.

*/
// fn main() {
//     use crate::print_things::prints_one_thing;

//     prints_one_thing(6);
//     prints_one_thing("Trying to print a string...".to_string());
// }

// ######################################

mod print_things {
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    pub struct Billy {
        // Billy is public
        name: String, // but name is private.
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            // That means the user needs to use new to create a Billy. The user can only change the number of times_to_print
            Self {
                name: "Billy".to_string(), // We choose the name - the user can't
                times_to_print,
            }
        }

        pub fn print_billy(&self) {
            // This function prints a Billy
            for _ in 0..self.times_to_print {
                println!("{:?}", self.name);
            }
        }
    }

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}

mod country {
    // The main mod doesn't need pub
    fn print_country(country: &str) {
        // Note: this function isn't public
        println!("We are in the country of {}", country);
    }
    pub mod province {
        // Make this mod public

        fn print_province(province: &str) {
            // Note: this function isn't public
            println!("in the province of {}", province);
        }

        pub mod city {
            // Make this mod public
            pub fn print_city(country: &str, province: &str, city: &str) {
                // This function is public though
                crate::country::print_country(country);
                crate::country::province::print_province(province);
                println!("in the city of {}", city);
            }
        }
    }
}
fn main() {
    use crate::print_things::*; // Now we use *. This imports everything from print_things

    let my_billy = Billy::new(3);
    my_billy.print_billy();

    crate::country::province::city::print_city("Canada", "New Brunswick", "Moncton");
}
