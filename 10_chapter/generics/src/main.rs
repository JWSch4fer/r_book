//we also don't write these functions separately
//fn largest(list: &[i32]) -> &i32 {
    //let mut largest = &list[0];

    //for item in list {
        //if item > largest {
            //largest = item;
        //}
    //}
    //largest
//}

//fn largest_char(list : &[char]) -> &char {
    //let mut largest = &list[0];

    //for item in list {
        //if item > largest {
            //largest = item;
        //}
    //}
    //largest
//}

//use a generic function
fn largest<T: std::cmp::PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list =vec![34,50,25,100,65];

    //we dont want this here because it will reused
    //let mut largest = &number_list[0];

    //for number in &number_list {
        //if number > largest {
            //largest = number;
        //}
    //}

    //println!("The largest number is {largest}");

    //generics help prevent duplicate code  
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
