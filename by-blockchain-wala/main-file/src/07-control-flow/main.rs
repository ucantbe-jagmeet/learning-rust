// use std::{thread, time};

fn main() {
    let num = 7;
    if num < 5 {
        println!("True")
    } else {
        println!("false")
    }

    let condition = false;
    let number = if condition { 5 } else {10};

    println!("the value is {number}");


    let mut x =0;
    loop{
        println!("again!");
        x+=1;
        println!("value of x = {x}");
        
        if x>=5 {
            break
        }
        
        // thread::sleep(time::Duration::from_secs(1))
    }
    
    let mut y =0;
    while y <5 {
        println!("again!");
        y+=1;
        println!("value of y = {y}");

        // thread::sleep(time::Duration::from_secs(1))

    }


    let arr = [10, 20, 30];

    let mut index = 0;
 
    while index < 3{
        println!("the value is: {}", arr[index]);
        index+=1;
    }
    for element in arr {
            println!("the value of elem is : {element}");
    }

}


