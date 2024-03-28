// A type alias means "giving a new name to another type".
use std::iter::{Skip, Take};
use std::slice::Iter;

type CharacterVec = Vec<char>;

type SkipFourTakeFive<'a> = Take<Skip<Iter<'a, char>>>;
fn returns<'a>(input: &'a CharacterVec) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}

struct File(String); // File is a wrapper around String

enum MapDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
fn give_direction(direction: &MapDirection) {
    use MapDirection::*; // Import everything in MapDirection
    let m = "You are heading";

    match direction {
        North => println!("{} north.", m),
        NorthEast => println!("{} northeast.", m),
        // This is a bit better
        // ⚠️
    }
}

fn main() {
    // let my_file = File(String::from("I am file contents"));
    // let my_string = String::from("I am file contents");

    // println!("{}", my_file.0 == my_string);

    //######### Importing and renaming inside a function ##########
}

enum FileState {
    CannotAccessFile,
    FileOpenedAndReady,
    NoSuchFileExists,
    SimilarFileNameInNextDirectory,
}

fn give_filestate(input: &FileState) {
    use FileState::{
        CannotAccessFile as NoAccess, FileOpenedAndReady as Good, NoSuchFileExists as NoFile,
        SimilarFileNameInNextDirectory as OtherDirectory,
    };
    match input {
        NoAccess => println!("Can't access file."),
        Good => println!("Here is your file"),
        NoFile => println!("Sorry, there is no file by that name."),
        OtherDirectory => println!("Please check the other directory."),
    }
}
