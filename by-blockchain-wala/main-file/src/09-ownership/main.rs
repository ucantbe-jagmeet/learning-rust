fn main(){
    
    // println!("Hello wOrld");
    // {
    //     // scope of s is fixed
    //     let s: &str = "hello";
    //     let p: &str = s;
    //     // s is of type string literal and its size is fixed 
    //     println!("value of s is : {s}");
    //     println!("value of p is : {p}");
    // }
    // {
    //     // scope of s is fixed
    //     let s1: String = String::from("hello");
    //     let p1: String = s1.clone();
    //     println!("value of s1 is : {s1}");
    //     println!("value of p1 is : {p1}");

    //     // string type variables is store is heap so its is dynamic in size 
    // }

    let s = String::from("hello");
    println!("before s value is : {s}");
    // takes_responsibility(s);
    takes_responsibility(s.clone());
    println!("after s value is : {s}"); // value of s will not print bcause ownership is transfer in takes_Res function 
    // and s becomes shadowing
    // ## if we pass s.clone() in takes_repsonsibility then both afer and before value will be printed
    
    
    let x = 5;
    println!("before x value is : {x}");
    makes_copy(x); // static data type mei copy hota hai hamesha
    println!("after x value is : {x}");


    //############## more about ownership #######################
    let s1 = give_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
} 

fn takes_responsibility(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn give_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}

// this function takes a string and returns one
fn takes_and_gives_back(a_string: String) -> String{
    a_string
}