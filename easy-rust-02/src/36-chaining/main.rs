fn main() {
    // let mut new_vec = Vec::new();
    // let mut counter = 1;

    // while counter < 11 {
    //     new_vec.push(counter);
    //     counter += 1;
    // }

    let mut my_vec = (1..=10).collect::<Vec<i32>>();

    //  let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    let new_vec = my_vec
        .into_iter() // "iterate" over the items (iterate = work with each item inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect::<Vec<i32>>(); // put them in a new Vec<i32>
    println!("{:?}", new_vec);
}
