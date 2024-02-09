#[derive(PartialEq)]
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String)
}

fn main(){
    let animal = Animal::Cat( String::from("Roxy"));
    IfLet(animal);
}

fn MatchIt(animal : Animal) {
    match animal {
        Animal::Dog(name) => println!("It's a Dog named {}", name),
        Animal::Cat(name) => println!("It's a Cat named {}", name),
        Animal::Bird(name) => println!("It's a Bird named {}", name),
    }
}

fn IfLet(animal : Animal) {
    if let Animal::Dog(name) = animal {
        println!("Its a Dog name {}!", name);
    }
    else if let Animal::Cat(name) = animal {
        println!("Its a Cat name {}!", name);
    }
    else if let Animal::Bird(name) = animal {
        println!("Its a Bird name {}!", name);
    }
}

// if else will not fulfill this situation
fn IfLet(animal : Animal) {
    if animal == Animal::Cat(String::from("Roxy")) {
        println!("Its a Dog name {}!", name);
    }
    else if animal == Animal::Cat(String::from("Kitty")) {
        println!("Its a Cat name {}!", name);
    }
    // else if let Animal::Bird(name) = animal {
    //     println!("Its a Bird name {}!", name);
    // }
}