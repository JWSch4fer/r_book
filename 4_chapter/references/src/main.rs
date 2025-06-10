fn main() {
    let s1 = String::from("hello");

    //we are not giving s1 ownership to calculate_length
    //we are passing a reference to s1, borrowing
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");

    change(&mut s2);
    println!("{s2}");

    let _dangle_danger = dangle();
}
//NOTE: Just as variables are immutable by default, so are references.
// We’re not allowed to modify something we have a reference to.
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
    //s.push_str(", world"); //<- this will fail
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the value is not dropped.


//mutable references exist but you have to explicitly write them
//NOTE: you cannot have multiple mutable references to the same data simultaneously

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//rust compiler prevents dangling references
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s
} //Here, s goes out of scope, and is dropped, so its memory goes away.
  // Danger!