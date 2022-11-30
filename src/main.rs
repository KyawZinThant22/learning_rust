fn main() {
    println!("Hello loop in rust");

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter;
    //     }
    // };
    // println!("The result is {}", result);

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("Loop stop")

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}", element)
    }

    for number in 1..11 {
        println!("{}!", number)
    }
}
