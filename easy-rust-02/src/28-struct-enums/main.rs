// implementing structs and enums in rust
// introduction to methods
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}
#[derive(Debug)]
enum Mood {
    Good,
    Bad,
    Sleepy,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new() -> Self {
        // Self means animal
        // you can also write Animal istead of Self

        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // because we are inside Animal,&mut self means &mut Animal
        // use .change_to_dog() to change the cat to the dog
        // with &mut self ,we can change it

        println!("Change type to Dog");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_cat() to change the dog to a cat
        // with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        // we want to read self
        use self::AnimalType::*;
        match self.animal_type {
            Dog => println!("The animal is a dog"),
            Cat => println!("The animal is a cat"),
        }
    }
}

impl Mood {
    fn check(&self) {
        use self::Mood::*;
        match self {
            Good => println!("Feeling good"),
            Bad => println!("Eh! feeling not good"),
            Sleepy => println!("Need sleep NOW"),
        }
    }
}

fn main() {
    let mut new_animal = Animal::new();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    let my_mood = Mood::Sleepy;
    my_mood.check();
}
