#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
 //   let width1 = 30;
 //   let height1 = 50;

//    let dimensions: (u32, u32) = (30, 50);

    let rect1 = Rectangle{
        width: 32,
        height: 50,
    };

    //printing custom structs is not a defualt behaviour
    //you have to opt in
    println!("The rectangle is {rect1:?}");
    //another option for printing is dbg! but
    //NOTE: this prints to stderr and takes ownership
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1));

}

//not great becuase it is not clear that width and height are related
//This function could be used to multiply arbitrary numbers
//fn area(width: u32, height: u32) -> u32 {
//    width *height
//}

//better becuase now you need a tuple which prevents arbitrary use
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}

//best option for clarity and to avoid improper function calls
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}