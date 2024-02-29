/*
An iterator is a construct that can give you the items in the collection, one at a time. Actually, we have already used iterators a lot: the for loop gives you an iterator. When you want to use an iterator other times, you have to choose what kind:

.iter() for an iterator of references
.iter_mut() for an iterator of mutable references
.into_iter() for an iterator of values (not references)

*/

fn main() {
    let vector1 = vec![1, 2, 3]; // we will use .iter() and .into_iter() on this one
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

    let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
    vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", vector1_a);
    println!("{:?}", vector2);
    println!("{:?}", vector1_b);
}
