fn main() {
    //these are not the same type
    let data = "initial Contents";

    let s = data.to_string();

    //the is equivalent to the above two lines
    let s2 = String::from("initial contents");

    let mut s3 = String::from("foo");
    s3.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //note s1 is now moved and can no longer be used
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;

    //format is a beter option when we want to do something like the above
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}
