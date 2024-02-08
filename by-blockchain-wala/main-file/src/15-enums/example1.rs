#[derive(Debug)]
enum Gender{
    Male(u8),
    Female(u8),
    Other(u8)
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // method body would be defined here
        if let Message::Move {x, y} = self{
            println!("x: {}, y: {}", x, y);
        }

    }
}

fn main(){
    let komal = Gender::Female(20);
    println!("Hello, {:?}", komal);

    let xyz = Message::Move{x: 8, y:100};

    println!("{:?}", xyz);

    xyz.call();


    
}