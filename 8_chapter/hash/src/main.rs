use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    //hash maps will take ownership once strings are inserted
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team_blue = String::from("Blue");
    //.copied return Option<i32> rather than Option<&i32>
    let score_blue = scores.get(&team_blue).copied().unwrap_or(0);
    println!("blue team score is {score_blue}");

    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    //checking if a key already exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);

    //this will not be inserted becuase "Blue" exists
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
