#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {
    remainder: &str,
    delimiter: &str,
}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// let x : StrSplit;
// for part in x {}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter){
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[ (next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } self.remainder.is_empty()_{
            None
        } else_{
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters =  StrSplit::new(haystack," ");

    let result = letters.collect();
    assert_eq!(result, vec!["a", "b", "c", "d", "e"].into_iter());
}