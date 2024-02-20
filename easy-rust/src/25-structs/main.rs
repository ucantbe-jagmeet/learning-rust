struct Colour(u8, u8, u8);
struct SizeAndColour {
    size: u32,
    colour: Colour, // And we put it in our new named struct
}
fn main() {
    let my_colour = Colour(50, 0, 50); // Make a colour out of RGB (red, green, blue)

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour,
    };
    println!("The second part of the colour is: {}", my_colour.1);
}
