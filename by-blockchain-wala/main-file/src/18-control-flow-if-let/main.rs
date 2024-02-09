fn main(){
    // println!("control flow with if let");
    
    let config_max = Some(3u32);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), 
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}