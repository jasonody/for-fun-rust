#[derive(Debug)] //include functionality to print out debugging information for this struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    //rectangle: immutable borrow of a struct Rectangle instance
    //want to borrow the struct rather than take ownership of it so main() can continue to use rect1
    rectangle.width * rectangle.height
}