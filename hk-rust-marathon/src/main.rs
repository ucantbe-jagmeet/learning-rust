// q. write a function is_even that takes a number as an input and returs true if it is even

fn main() {
    println!("{}", is_even(-20))
}

// make sure when to use unsignend and signed integers

fn is_even(num: i32) -> bool {
    if num %2 ==0 {
        return true
    }
    return  false;
}