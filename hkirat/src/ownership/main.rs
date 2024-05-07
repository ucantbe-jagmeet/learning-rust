// memory is managed through a system of ownership with a set of rules that the compiler checks.

// if any of these rules are violated, the program won't compile.

//ownership meant more for heap features

fn main() {
    // ##### STack Variable ###########
    // let x = 1; // created on stack, owner is main fn
    // let y = 4; // created on stack, owner is main fn

    // println!("{}", sum(x, y));
    // println!("Hello, World");

    /*
    let x = 1;
    {
        let y = 4 // store on stack
        // this is nothing to do with ownership
    }

    println!("{}", y); // error

    */

    // ######### Heap Variable ###########

    // let s1 = String::from("Hello");
    // let s2 = s1; // here s2 doesn't point the copy of Hello , This means S2 also points to s1 variable
    // creating/copying in the heap is very expensive
    // two variables pointing to same string , if one them is die , then string will also die
    // so ownersip moved from s1 to s2 , and s1 is no longer owning that string

    // println!("{}", s1);// no longer valid

    // create_str();

    // let my_string = String::from("hello");
    // takes_ownership(my_string); // error
    // takes_ownership(my_string.clone()); // if we clone the string , it means it creating another string on heap, which in an expensive task

    // println!("{}", my_string);

    // if lets assume my_String is a number  i32 , then it will create another copy on the stack , but here in the case of string it is different

    let mut my_string = String::from("Hello");
    let my_string1 = takes_ownership(my_string);

    println!("{}", my_string1)
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}

fn create_str() {
    let s1 = String::from("Hello");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b; //  owner is sum fn
    return c;
}
