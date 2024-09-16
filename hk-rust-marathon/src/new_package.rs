use chrono::{ Local, Utc};

fn main(){

    let now = Utc::now();
    println!("Current date and time in Utc: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("formatted date and time in Utc: {}", formatted);

    let local = Local::now();
    println!("Current data and time in local: {}", local);
}