/*
An iterator is a construct that can give you the items in the collection, one at a time. Actually, we have already used iterators a lot: the for loop gives you an iterator. When you want to use an iterator other times, you have to choose what kind:

.iter() for an iterator of references
.iter_mut() for an iterator of mutable references
.into_iter() for an iterator of values (not references)


#### working

An iterator works by using a method called .next(), which gives an Option. When you use an iterator, Rust calls next() over and over again. If it gets Some, it keeps going. If it gets None, it stops.
*/

/*
fn main() {
//     let vector1 = vec![1, 2, 3]; // we will use .iter() and .into_iter() on this one
//     let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//     let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

//     let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
//     vector2.iter_mut().for_each(|x| *x += 100);

//     println!("{:?}", vector1_a);
//     println!("{:?}", vector2);
//     println!("{:?}", vector1_b);
// }

*/

#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}
#[derive(Debug, Clone)]
enum LibraryType {
    // libraries can be city libraries or country libraries
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String; // this is must to write type
                        // This is the associated type. Our iterator will be for our list of books, which is a Vec<String>. When we call next, it will give us a String. So we will write type Item = String;. That is the associated item.
    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found"), // rust allows to add String and &str
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new(); // make a new library
    my_library.add_book("The Doom of the Darksword"); // add some books
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    // println!("{:?}", my_library.books); // we can print our list of books

    for item in my_library.clone() {
        // we can use a for loop now. Give it a clone so Library won't be destroyed
        println!("{}", item);
    }
}
