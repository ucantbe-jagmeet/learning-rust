// $expr are macro fragments specifiers
// Fragment specifiers are placeholders that are used to refer to specific parts of the input to the macro

#[macro_export]
macro_rules! makevec {
    () => {
        Vec::new()
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

