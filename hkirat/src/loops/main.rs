fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}
fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn main() {
    // let x = 99;

    // let is_even = is_even(x);

    // if is_even {
    //     println!("{} is even", x);
    // } else {
    //     println!("{} is odd", x);
    // }

    // for i in 0..10 {
    //     println! {"{}", i};
    // }

    let sentence = String::from("Saturday name is jagmeet");
    let first_word = get_first_word(sentence);
    println! {"{}", first_word};
}
