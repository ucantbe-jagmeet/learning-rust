fn main() {
    // Sometimes you have too many " and escape characters, and want Rust to ignore everything. To do this, you can add r# to the beginning and # to the end.
    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."); // We used \ five times here
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#);
    // This prints the same thing, but using r# makes it easier for humans to read.

     let my_string = "'Ice to see you,' he said."; // single quotes
    let quote_string = r#""Ice to see you," he said."#; // double quotes
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!("{}\n{}\n{}\n{}\n", my_string, quote_string, hashtag_string, many_hashtags);


    let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    // r# has this function because older versions of Rust had fewer keywords than Rust now. So with r# you can avoid mistakes with variable names that were not keywords before.


    //  println!("{:?}", b"This will look like numbers"); // This will look like numbers

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref); // {:p} is a format specifier that tells the compiler to print the address of the variable.
    // or to print the pointer address

    // {:b} binary
    // {:x} hexadecimal
    // {:o} octal
}
