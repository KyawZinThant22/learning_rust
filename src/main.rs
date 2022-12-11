fn main() {
    // ---- OwnerShip rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time .
    // 3. When the owner goes out of scope , the value will be dropped.

    let x = 5;
    let y = x;

    println!("The value of y is : {y}");
}
