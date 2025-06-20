fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

//this will fail because the compiler doesn't know how to deal with the 
// lifetime of x and y. the restul only references one of these variables
//fn longest(x: &str, y: &str) -> &str {
//    if x.len() > y.len() { x } else { y }
//}

fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}