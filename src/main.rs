fn main() {
    // ---- OwnerShip rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time .
    // 3. When the owner goes out of scope , the value will be dropped.


    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}",s);

    // let s1 = String::from("Hello");
    // let s2 = s1.clone();

    // println!("{} , world" , s1);

//     let s = String::from("Hello");
//     takes_ownerwhip(s);
//     let x = 5;

//     makes_integer(x);


// }


// fn takes_ownerwhip(some_string : String){
//     println!("{}",some_string);
// }

// fn makes_integer(some_integer : i32) {
//     println!("{}", some_integer);
// }


// let s1 = give_owership();
// let s2 = String::from("World");
// let s3 = take_and_gives_back(s2);
// println!("s1={} , s3={}", s1,s3);

// }

// fn give_owership() -> String {
//     let some_string = String::from("Yours");
//     some_string
// }

// fn take_and_gives_back (some_string : String) ->String {
//     some_string
// }

// Rust does let us return multiple values using a tuple

let s1 = String::from("Hello");

let (s2 , len) = calculate_length(s1);

println!("s3={} , len={}" , s2,len);

}


fn calculate_length  (str : String) -> (String,usize) {
    let length = str.len();

    (str,length)
}