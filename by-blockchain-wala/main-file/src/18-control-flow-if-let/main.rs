#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
    Bird
}

fn main(){
    // println!("control flow with if let");

    let config_max = Some(3u32);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), 
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let animal = Animal::Cat;
    // IfElse(animal);
    // MatchIt(animal);
    IfLet(animal);


}

fn IfElse(animal : Animal) {
    if animal == Animal::Dog {
        println!("It's a dog!");
    }
    else if animal == Animal::Cat {
        println!("It's a Cat!");
    }
    else if animal == Animal::Bird {
        println!("It's a Bird!");
    }
    else {
        println!("Unknown Creature");
    }
}

fn MatchIt(animal : Animal) {
    match animal {
        Animal::Dog => println!("It's a Dog!"),
        Animal::Cat => println!("It's a Cat!"),
        Animal::Bird => println!("It's a Bird!"),
    }
}

fn IfLet(animal : Animal) {
    if let Animal::Dog = animal {
        println!("Its a Dog!");
    }
    else if let Animal::Cat = animal {
        println!("Its a Cat!");
    }
    else if let Animal::Bird = animal {
        println!("Its a Bird!");
    }
}