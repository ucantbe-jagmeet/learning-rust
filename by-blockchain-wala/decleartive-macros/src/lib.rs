// $expr are macro fragments specifiers
// Fragment specifiers are placeholders that are used to refer to specific parts of the input to the macro
#[macro_export]
macro_rules! my_rule {
    ($arg1:ty , $arg2:expr, $arg3:path) => {};
}

#[macro_export]
macro_rules! makevec {
    () => {
        Vec::new()
    };
    ($element:expr) => {
        vec![$element] 
    };
    ($element:expr) => {

      {  let mut x = Vec::new();
        x.push($element);
        x}
    };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = makevec!();
    assert!(x.is_empty());
}

#[test]
fn single() {
    let mut x = makevec![];
    x.push(4);
    assert_eq!(x.len(), 1);
}

#[test]
fn new_func(){
    let x= makevec![50];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 50);
}