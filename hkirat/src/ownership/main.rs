// memory is managed through a system of ownership with a set of rules that the compiler checks.

// if any of these rules are violated, the program won't compile.

//ownership meant more for heap features

fn main() {
    let x = 1; // created on stack, owner is main fn
    let y = 4; // created on stack, owner is main fn

    println!("{}", sum(x, y));
    println!("Hello, World");

    /*
    let x = 1;
    {
        let y = 4 // store on stack
        // this is nothing to do with ownership

    }

    println!("{}", y); // error

    */
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b; //  owner is sum fn
    return c;
}
