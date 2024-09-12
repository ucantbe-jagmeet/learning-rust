// Q. write a function fib that finds the fibbonacci of a number it take a input

fn main(){
    let num:u32 = 4;

    println!("{}", fib(num));

}

fn fib(_num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if _num == 0 {
        return first
    }

    if _num == 1 {
        return 1
    }

    for _ in 0..(_num - 1){
        let temp = second ;
        second = second + first;
        first = temp;
    }

    return second
}

// mut variables are variables whose values you can change