// $expr are macro fragments specifiers
// Fragment specifiers are placeholders that are used to refer to specific parts of the input to the macro
macro_rules! my_rule {
    ($arg1:expr) => {{
            let x = $arg1; 
            x
    }};
}

#[test]
fn check() {
    let val = my_rule!(3);
    assert_eq!(val,3);
}
