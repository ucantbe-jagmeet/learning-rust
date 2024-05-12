fn main() {
    // ##### printing to console #########
    // let a = 10;
    // let b = 15;
    // println!("{} {}", a, b);
    // ##### printing to console ends #########

    // ##### primitive variables #########
    /*
    let unsigned: u8 = 10;
    let signed: i8 = -10;
    let float: f32 = 1.20;
    // println!("{} {} {}", unsigned, signed, float);

    let letter = "c12312";
    let emoji = "\u{1F600}";

    // println!("letter: {}, emoji:{}", letter, emoji);

    let is_true = true;
    // println!("isTrue:{}", is_true);
    */
    // ##### primitive variables ends #########

    // ##### array, tuples #########
    /*
    let arr: [u32; 3] = [10, 20, 30];
    let other_arr: [u32; 5] = [100; 5];

    println!("index {}, length: {}", arr[0], other_arr.len());

    println!("{:?}", other_arr); // print structure f arrya and other objects

    let tuple = (4, true, 2.1);
    let tuple2 = (3, 5);

    // print structure of array and other objects
    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("tuple2 {:?}", tuple2);

    let (a, b, c) = tuple;
    println!("value of b:  {}", b)
    */
    // ##### array, tuples ends#########

    //####### functions starts#######
    // println!("is even {}", is_even(2));
    //####### functions ends######

    //####### mutability ######
    /*
    let mut num = 5;
    num = 3;
    println!("num :{}", num);
    */
    //####### mutability ends######

    //#######arrays and slice ######
    /*
    let arr: [u8; 6] = [0, 1, 2, 3, 4, 5];
    let slice = &arr[3..5];

    borrowing_slice(arr, slice);
    */
    //#######arrays and slice ends ######
}

// fn is_even(num: u8) -> bool {
//     let digit = num % 2;
//     digit == 0
// }

// fn borrowing_slice(arr: [u8; 6], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length : {}", slice.len());
//     println!("{} {}", slice[0], slice[1]);
// }
