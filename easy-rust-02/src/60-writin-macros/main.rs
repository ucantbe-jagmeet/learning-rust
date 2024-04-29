//it's not the same as a match statement, because a macro actually doesn't compile anything. It just takes an input and gives an output. Then the compiler checks to see if it makes sense. That's why a macro is like "code that writes code".
macro_rules! six_or_print {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me 6.");
    };
}
macro_rules! might_print {
    /*
       item: an Item
       block: a BlockExpression
       stmt: a Statement without the trailing semicolon (except for item statements that require semicolons)
       pat: a Pattern
       expr: an Expression
       ty: a Type
       ident: an IDENTIFIER_OR_KEYWORD
       path: a TypePath style path
       tt: a TokenTree (a single token or tokens in matching delimiters (), [], or {})
       meta: an Attr, the contents of an attribute
       lifetime: a LIFETIME_TOKEN
       vis: a possibly empty Visibility qualifier
       literal: matches -?LiteralExpression
    */
    ($input:expr) => {
        println!("You gave me: {:?}", $input);
    };
}

macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}
macro_rules! print_anything {
    //To give a macro more than one item at a time, we have to use a different syntax. Instead of $input, it will be $($input1),*. This means zero or more (this is what * means), separated by a comma. If you want one or more, use + instead of *
    // ($input:tt) => {
    //     let output = stringify!($input);
    //     println!("{}", output);
    // };

    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}

fn main() {
    // let my_number = six_or_print!(6);
    // six_or_print!();

    // might_print!(6); // The $input:expr part is important. It means "for an expression, give it the variable name $input". In macros, variables start with a $. In this macro, if you give it one expression, it will print it.
    // might_print!(()); // give it a ()
    // might_print!(6); // give it a 6
    // might_print!(vec![8, 9, 7, 10]); // give it a vec

    // let x = 6;
    // let my_vec = vec![7, 8, 9];
    // check!(x, 6);
    // check!(my_vec, vec![7, 8, 9]);
    // check!(x, 10);

    print_anything!(ththdoetd, rcofe);
    print_anything!();
    print_anything!(87575oehq75onth, ntohe, 987987o, 097);
}
