use std::cmp::PartialOrd;

struct Point <T, U> {
    x: T,
    y: U
}

fn find_largest <T: PartialOrd>( list: &[T]) -> &T{
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main(){
    let number_list = vec![ 34, 50, 25, 100, 65];
    let number_list2 = vec![ 3, 5, 2, 10, 65];

    let result1 = find_largest( &number_list);
    let result2 = find_largest( &number_list2);
    
    let char_list = vec![ 'k', 'u', 'm', 'y'];
    let result3 = find_largest( &char_list);

    println!("largest number are: {}, {} and {}", result1, result2, result3);

    let integer_num = Point { x: 5, y: 10};
    let float_num = Point { x: 13.0, y: 4.0};
    let mix_type_num = Point { x: 13, y: 4.0}; // extra case

}
