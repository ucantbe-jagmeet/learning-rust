struct Rectangle{
    width: u32,
    height: u32
}

fn main(){
    println!("Example struct");
    // simple calc
    let width1 = 30;
    let height1 = 50;

    // using tuples
    let rect = ( 40, 90);

    // using commposite types
    let rect2 = Rectangle{
        width: 100,
        height: 10
    };

    println!(
        "The area of the rectangle having height {} and width {} is {} square pixels", rect2.height, rect2.width,
        area_struct(&rect2)
    );


}

fn area ( width: u32, height: u32) -> u32{
    width * height
}

fn area_tuple ( dimensions : (u32, u32) ) -> u32{
    dimensions.0 * dimensions.1
}

fn area_struct ( rect: &Rectangle ) -> u32{
    rect.height * rect.width
}
