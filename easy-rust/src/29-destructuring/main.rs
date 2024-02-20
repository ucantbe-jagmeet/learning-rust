struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

fn main(){
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 180,
        happiness: false
    }

    let Person { // destructure papa_doc
        name: a,
        real_name: b,
        height: c,
        happiness: d
    } = papa_doc;

    println!("They call him {} but his real name is {}. He is {} cm tall and is he happy? {}", a, b, c, d);
}