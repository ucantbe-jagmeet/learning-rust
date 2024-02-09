// Rust group Error divided into two major categories
// -> Recoverable := such as file not found error, we most likely just want to report the problem to the user and retry the operation
// -> Unrecoverable := those errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program

// Result<T, E> type is use for recoverable errors
/*
enum Result < T, E> {
        Ok(T),
        Err(E),
    }

*/
// panic! type is use for unrecoverable errors

/*
    panic -> By default when a panic occurs, the program starts unwinding, which means rust walks back up the stack and cleans up the data from each function it ecounters. However, this walking back and clean up is alot of work . Rust , therefore , allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
*/
use std::fs::File;
use std::io::ErrorKind;

pub enum Result < T, E> {
        Ok(T),
        Err(E),
    }
fn main(){
    // panic!("crash and burn");

    // let v = vec![ 1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:#?}", error),
            },
            other_error => {
                panic!("Problem opening the file : {:?}", other_error);
            }
        },
    };
}