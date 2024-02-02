static mut COUNT : u32 = 0;
const PI:f32 = 3.14159;

fn main(){
    // Accessing static variable
    
    unsafe {
        println!("Initial count : {}", COUNT);
        COUNT += 1; 
        println!("Updated count : {}", COUNT);
    }

    // calling a function that modifies the static variable
    increment_count();
    // Accessing the static variable again

    unsafe{
        println!("Final count : {}", COUNT);
    }
    
    println!("area of the circle is with radius 10 is {}", area_of_circle(10.0));

}

fn increment_count(){
    unsafe{
        COUNT += 1;
    }
}

fn area_of_circle(x: f32) -> f32{
    let area = PI*x*x;
    return area;
}