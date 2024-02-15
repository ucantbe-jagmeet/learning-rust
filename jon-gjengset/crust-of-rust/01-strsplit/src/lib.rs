#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {}

impl StrSplit {
    pub fn new( haystack: &str, delimiter: &str) -> Self {}
}

// let x : StrSplit;
// for part in x {}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {}
}

#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters =  StrSplit::new(haystack," ");

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}