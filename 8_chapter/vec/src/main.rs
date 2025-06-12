use std::ffi::os_str::Display;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    //another option for initializing
    let v = vec![1,2,3,4,5,6];

    //you can index into items
    let third: &i32 = &v[2];
    println!("the third element is {third}");


    //you can also use the get method (safer)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third element"),
    }

    //this fails becuase vectors are allocated on the heap.
    //if v ever grows enough to move its location in memory
    //the reference to the first element would lose its meaning.
    //the borrowing rules prevent this mistake.
    //let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0];

    //v.push(6);

    //println!("The first element is: {first}");

    //vectors are restriced to a single type
    //combine vectors with enums to get around this

    #[derive(Debug)]
    enum SpreadsheetCells {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCells::Int(3),
        SpreadsheetCells::Float(3.14),
        SpreadsheetCells::Text(String::from("turtle")),
    ];

    println!("row = {row:?}");
}
