//todo!() is actually the same as another macro: unimplemented!(). Programmers were using unimplemented!() a lot but it was long to type, so they created todo!() which is shorter.

//But careful, because it only compiles - you can't use the function. If you call a function with todo!() inside it, it will panic.

// Also, todo!() functions still need real input and output types. If you just write this, it will not compile:

struct Book {} // Okay, first I need a book struct.
               // Nothing in there yet - will add later

enum BookType {
    // A book can be hardcover or softcover, so add an enum
    HardCover,
    SoftCover,
}

// fn get_book(book: &Book) -> Option<String> {} // ⚠️ get_book should take a &Book and return an Option<String>

// fn delete_book(book: Book) -> Result<(), String> {} // delete_book should take a Book and return a Result...
//                                                     // TODO: impl block and make these functions methods...

fn get_book(book: &Book) -> Option<String> {
    todo!() // todo means "I will do it later, please be quiet"
}
fn delete_book(book: Book) -> Result<(), String> {
    todo!() // todo means "I will do it later, please be quiet"
}

fn check_book_type(book_type: &BookType) {
    // Let's make sure the match statement works
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}

fn main() {
    let book_type = BookType::HardCover;
    check_book_type(&book_type); // Okay, let's check this function!
}
