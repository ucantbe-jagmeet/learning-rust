fn main() {
    const THREE_HOURS_IN_SECONDS :u32 = 60 * 60 * 3;
    
    let x = 5;
    let y = 10;
    println!("The value of y is : {y} and x is {}", x);
    
    {
        let y = 20; // shadowing aka blockscope 
        println!("The value of y is : {y} and x is {}", x);
    }
    
    // let mut a = 20; // mutability with mut keyword so that programmer does not change value of this
    // println!("The value of a is : {a}");


}
