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

    let s1 = String::from("Hello");
    let s2 = s1; // here s2 doesn't point the copy of Hello , This means S2 also points to s1 variable
                 // creating/copying in the heap is very expensive
                 // two variables pointing to same string , if one them is die , then string will also die
                 // so ownersip moved from s1 to s2 , and s1 is no longer owning that string

    // println!("{}", s1);// no longer valid

    create_str();
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
