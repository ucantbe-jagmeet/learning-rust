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
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // let mut file = fs::File::create("myfilename.txt")?; // Create a file with this name.
    //                                                     // CAREFUL! If you have a file with this name already,
    //                                                     // it will delete everything in it.
    // file.write_all(b"Let's put this in the file")?; // Don't forget the b in front of ". That's because files take bytes.
    // Ok(())

    // fs::File::create("myfilename.txt")?.write_all(b"Let's put this in the file")?;
    // Ok(())

    // fs::write("calvin_with_dad.txt",
    // "Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
    // Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
    // Calvin: Really?
    // Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    // let mut calvin_file = File::open("calvin_with_dad.txt")?; // Open the file we just made
    // let mut calvin_string = String::new(); // This String will hold it
    // calvin_file.read_to_string(&mut calvin_string)?; // Read the file into it

    // calvin_string
    //     .split_whitespace()
    //     .for_each(|word| print!("{} ", word.to_uppercase())); // Do things with the String now

    // Ok(())

    /*
        append(): This means "add to the content that's already there instead of deleting".
    create(): This lets OpenOptions create a file.
    create_new(): This means it will only create a file if it's not there already.
    read(): Set this to true if you want it to be able to read a file.
    truncate(): Set this to true if you want to cut the file content to 0 (delete the contents) when you open it.
    write(): This lets it write to a file.
        */

fs::write("calvin_with_dad.txt", 
"Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
Calvin: Really?
Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    let mut calvin_file = OpenOptions::new()
        .append(true) // Now we can write without deleting it
        .read(true)
        .open("calvin_with_dad.txt")?;
    calvin_file.write_all(b"And it was a pretty grainy color for a while too.\n")?;
    write!(&mut calvin_file, "That's really weird.\n")?;
    write!(&mut calvin_file, "Well, truth is stranger than fiction.")?;

    println!("{}", fs::read_to_string("calvin_with_dad.txt")?);

    Ok(())
}
