use std::fs::read_to_string;


fn main (){
    let greeting_file_result = read_to_string("./helllo.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(err) => {
            println!("Error occured: {:?}", err);
        },
    }

}

