//dbg! is a very useful macro that prints quick information. It is a good alternative to println! because it is faster to type and gives more information
//.inspect is a bit similar to dbg! but you use it like map in an iterator. It gives you the iterator item and you can print it or do whatever you want.

fn main() {
    /*
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);
    */

    /*
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {}", first_item))
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {}", next_item))
        .collect::<Vec<i32>>();
    */

    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is {}", first_item);
            match **first_item % 2 {
                // first item is a &&i32 so we use **
                0 => println!("It is Even."),
                _ => println!("It is Odd."),
            }
            println!("In binary it is {:b}", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}
