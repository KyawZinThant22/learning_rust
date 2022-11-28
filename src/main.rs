fn main() {
    //scalar data types and compount dataTypes
    //scalar data types represents single value
    //compount data types represents group of value

    // rust have fout main scalar data types
    //Integers  singes integers could be postive or negative numbers and unsinges numbers can only be positive number
    //floating point numbers
    //Boolean
    //Character

    //compound types

    let tup = ("Let's Learn Rust", 3);
    let (learning, count) = tup;
    let language_count = tup.1;

    let err_codes = [212, 123, 133];

    // let not_found = err_codes[5];

    //you can also declare array like this
    let byte = [0; 8];
}
