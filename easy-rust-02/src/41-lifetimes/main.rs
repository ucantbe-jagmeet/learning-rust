//A lifetime means "how long the variable lives". You only need to think about lifetimes with references. This is because references can't live longer than the object they come from.

#[derive(Debug)]
struct City {
    // It means "please only take an input for name if it lives at least as long as City". It does not mean: "I will make the input for name live as long as City".
    // So now we will try what the compiler suggested before. It said to try writing struct City<'a> and name: &'a str. This means that it will only take a reference for name if it lives as long as City.
    name: &'static str, // this gives error
    // Rust needs a lifetime for &str because &str is a reference. What happens when the value that name points to is dropped? That would be unsafe.
    date_founded: u32,
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points", self.name, self.hit_points)
    }
}

fn return_str() -> &'static str {
    let my_string = String::from("I am a string");
    // &my_string // ⚠️
    "I am a string"
}

fn main() {
    // let my_str = return_str();
    // println!("{}", my_str);

    /*

    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
    */
    // that works. And maybe this is what you wanted for the struct. However, note that we can only take "string literals", so not references to something else. So this will not work

    /*
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()]; // city_names does not live for the whole program

    let my_city = City {
        name: &city_names[0], // ⚠️ This is a &str, but not a &'static str. It is a reference to a value inside city_names
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
    */

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
