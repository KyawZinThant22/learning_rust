fn main() {
    let sum = my_function(3, 5);
    println!("Sum of the number is : {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is :{} ", x);
    println!("The value of y is :{}", y);
    let sum: i32 = x + y;
    return sum;
}
