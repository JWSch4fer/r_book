use std::option;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}
#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    //etc.
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        },
    }
}

//match statements and optional values allow you to keep track of 
//missing values in the program
//NOTE rust does not have NULL
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i +1),
    }
}

fn main() {
    println!("Hello, world!");

    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsStates::Alabama);

    println!("Your change is {}",value_in_cents(dime));
    println!("Your change is {}",value_in_cents(quarter));

    //using Some(5) means that this is of type Option<i32>
    //not i32
    let five = Some(5);
    let six = plus_one(five);
    //still works becuase we made the function generic
    let none = plus_one(None);

    println!("{five:?} {six:?} {none:?}");

    //there is also a catchall for for situations like this
    //this satisfies the match expression
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    //This is also valid
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    //Another example
    let coin =Coin::Quarter(UsStates::Alabama);
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
