fn main() {
    // let mut counter = 0; // set a counter to 0
    // loop {
    //     counter += 1; // increase the counter by 1
    //     println!("The counter is now: {}", counter);
    //     if counter == 5 {
    //         // stop when counter == 5
    //         break;
    //     }
    // }

    // let mut counter = 0;
    // let mut counter2 = 0;
    // println!("Now entering the first loop");

    // 'first_loop: loop {
    //     counter += 1;
    //     println!("The counter is now: {}", counter);

    //     if counter > 9 {
    //         println!("Now entering the second loop");

    //         'second_loop: loop {
    //             println!("The second counter is now:{}", counter2);
    //             counter2 += 1;

    //             if counter2 == 3 {
    //                 break 'first_loop;
    //             }
    //         }
    //     }
    // }

    let mut counter = 0;

    while counter < 5 {
        counter += 1;
        println!("The counter is now: {}", counter);
    }
}
