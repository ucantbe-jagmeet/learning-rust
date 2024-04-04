// use std::num::ParseIntError;

// fn give_number(input: &str) -> Result<i32, ParseIntError> {
//     input.parse::<i32>()
// }

// fn main() -> Result<(), ParseIntError> {
//     println!("{:?}", give_number("88")?);
//     println!("{:?}", give_number("5")?);
//     Ok(())
// }

use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // let mut file = fs::File::create("myfilename.txt")?; // Create a file with this name.
    //                                                     // CAREFUL! If you have a file with this name already,
    //                                                     // it will delete everything in it.
    // file.write_all(b"Let's put this in the file")?; // Don't forget the b in front of ". That's because files take bytes.
    // Ok(())

    fs::File::create("myfilename.txt")?.write_all(b"Let's put this in the file")?;
    Ok(())
}
