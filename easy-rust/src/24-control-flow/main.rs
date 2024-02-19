fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}

fn match_number(input: i32) {
    match input {
        number @ 4 => println!(
            "{} is an unlucky number in China (sounds close to 死)!",
            number
        ),
        number @ 13 => println!(
            "{} is unlucky in North America, lucky in Italy! In bocca al lupo!",
            number
        ),
        _ => println!("Looks like a normal number"),
    }
}

fn main() {
    let my_number = 2;

    // if my_number == 7 {
    //     println!("It's seven");
    // } else if my_number == 6 {
    //     println!("It's six");
    // } else {
    //     println!("It's a different number");
    // }

    // if my_number % 2 == 1 && my_number > 0 {
    //     println!("It's a positive odd number");
    // } else if my_number == 6 {
    //     println!("It's six");
    // } else {
    //     println!("It's a different number");
    // }

    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    };

    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };

    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    };

    // ########################################

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => {
            println!("Not married with {} children", children);
        }
        (children, married) if children == 0 && married == true => {
            println!("Married but no children");
        }
        _ => println!("Married? {}. Number of children: {}.", married, children),
    };

    //#################################################

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);

    let some_variable = match my_number {
        10 => 8,
        _ => "Not ten", // gives error
    };
    // use if else loop for this situation

    match_number(50);
    match_number(13);
    match_number(4);
}
