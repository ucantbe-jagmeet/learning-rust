#[derive(Debug)]
enum IpAddr{
    V4,
    V6
}

enum Gender{
    Male,
    Female,
    Other
}

fn main(){
    println!("Enums in struct");

    let v_four = IpAddr::V4;
    let v_six = IpAddr::V6;

    let male = Gender::Male;

}