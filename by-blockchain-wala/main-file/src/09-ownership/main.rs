fn main(){
    
    println!("Hello wOrld");


    {
        // scope of s is fixed
        let s: &str = "hello";
        let p: &str = s;
        // s is of type string literal and its size is fixed 
        println!("value of s is : {s}");
        println!("value of p is : {p}");
    }
    {
        // scope of s is fixed
        let s1: String = String::from("hello");
        let p1: String = s1.clone();
        println!("value of s1 is : {s1}");
        println!("value of p1 is : {p1}");

        // string type variables is store is heap so its is dynamic in size 
    }
} 