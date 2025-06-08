use std::io;

fn main() {

    // immutable by default
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing is possible and useful for type conversion
    // note the scope but y is technically immutable
    // every let y = ... creates a new variable
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "    ";
    // spaces = spaces.len() will fail
    let spaces = spaces.len();
    println!("there were {} spaces", spaces);

    // f64 and i32 are default in rust
    let z = 2.0; // f64

    let z: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let diff = 95.8 - 4.0;
    // let diff = 95.8 - 4;// type error
    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3; // = -1
    //remainder 
    let remainder = 43 % 5;

    //tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // access elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {six_point_four}");

    // arrays in rust are fixed length and only allow one type
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!(" the fourth month is {}", months[3]);

    // runtime error if you index outside of the array
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
