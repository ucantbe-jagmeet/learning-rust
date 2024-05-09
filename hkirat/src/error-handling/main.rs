use std::fs;
use std::io::Error;

fn main() {
    // there is a fn that can error out/stop thr thread

    // let res: Result<String, Error> = fs::read_to_string("example.txt");
    let res: Result<String, String> = read_from_file_unsafe("example.txt".to_string());

    // match res {
    //     Ok(content: String) =>{
    //         println!("File content: {}", content);
    //     },
    //     Err(err: Error) => {
    //         println!("Error : {}", Error);
    //     }
    // }

    println!("Hi there");
}

fn read_from_file_unsafe(file_content: String) -> Result<String, String> {
    let res = fs::read_to_string(file_content);
    match res {
        Ok(content: String) => Ok(content),
        Err(err: Error) => Err("Error reading file".to_string())
    }
}
// fn read_from_file_unsafe(file_content: String) -> String {
//     let res = fs::read_to_string(file_content);
//     return res.unwrap();
// }
