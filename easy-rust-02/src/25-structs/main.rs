struct Colour(u8, u8, u8);
struct SizeAndColour {
    size: u32,      //  No comma here gives gives error
    colour: Colour, // And we put it in our new named struct
}
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}
fn main() {
    let my_colour = Colour(50, 0, 50); // Make a colour out of RGB (red, green, blue)

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour,
    };
    println!("The second part of the colour is: {}", my_colour.1);
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population: population,
        capital: capital,
        leader_name: leader_name,
    };
}
