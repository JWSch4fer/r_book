
//NOTE: the implications for references that slices cause
//also &str is more permissive for functions that &String
fn main() {
    let example_string= String::from( "Hello, world!");
    let fw_length = first_word(&example_string);
    println!("the first word is {} char", fw_length);

    let fw_slice = first_word_w_slices(&example_string);
    println!("the first word is {} char", fw_slice);

    //slices
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    println!("{}",slice);

    //other slice type
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


//find the length of the first word in a string
fn first_word(s: &String) -> usize {
    //switch to bytes to loop through each character
    let bytes = s.as_bytes();

    //for loop with index and ref to element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;// stop when we hit the first space
        }
    }
    // if no spaces in the string return full length
    s.len()
}

fn first_word_w_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}