fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(5, 'h');

    // expresions versus statements
    // expressions return a value that can be bound to something
    // statements DO NOT return a value

    //expressions:
    let y = {
        let x = 3;
        x + 1// an ending ; here turns this into a statement...
        };

    //statement:
    let x = 7;
    // let x = let y = 7; this will fail

    println!("The value of y is: {y}");

    println!("the value is {}",five())
}

fn another_function() {
    println!("Another function.");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // adding a semicolon to the five will make it a statement
    // and will cause an error
    5
}