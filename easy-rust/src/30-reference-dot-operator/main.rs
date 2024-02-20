// references and dot operators

struct Item {
    number: u8,
}

impl Item{
    fn compare_number(&self, other_number: u8) -> {
        println!("are {} and {} equal? {}", self.number, other_number, self.number == other_number);
    }
}
fn main() {
    let my_number = 8;
    let reference_number = &my_number;

    println!("The value of my_number is {}", my_number);
    println!("{}", my_number == reference_number);
    // So we change line 5 to println!("{}", my_number == *reference); and now it prints true because it's now i32 == i32, not i32 == &i32. This is called dereferencing.

    let item = Item { number: 8 };

    // let ref_number = &item.number;
    let ref_item = &item;
    // println!("{}", ref_number == 10); // &u8 and u8 cannot be compared
    // println!("{}", ref_item.number);
}
