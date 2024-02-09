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
    IfElse(animal);

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