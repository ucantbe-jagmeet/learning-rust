#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
enum Shape {
    Rectangle(f32, f32),
    Circle(f32),
    Square(f32)
}

fn main(){
    let my_direction = Direction::South;
    let my_rect = Shape::Rectangle(20.0, 80.0);
    move_around(my_direction);
   println!("area of shape {}",  print_area(my_rect));
}

fn move_around(direction : Direction){
    println!("person is moving in {:?}", direction);
}
fn print_area(shape : Shape)-> f32{
        match shape {
        Shape::Rectangle(a,b) => return a*b,
        Shape::Circle(r) => return 3.14*r*r,
        Shape::Square(s) => return s*s,
    };

}