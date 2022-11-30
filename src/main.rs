fn main() {
    println!("Hello control flow");

    let number = 59;

    if number < 10 {
        println!("Number is less than 10");
    } else if number < 22 {
        println!("Also number is less thant 12");
    } else {
        println!("number is larger than we expected");
    }
}
