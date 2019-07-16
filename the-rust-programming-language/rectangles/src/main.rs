#[derive(Debug)] //include functionality to print out debugging information for this struct
struct Rectangle {
    width: u32,
    height: u32,
}

//implementation block
impl Rectangle {
    //area method defined on Rectangle instance
    fn area(&self) -> u32 { //reading (&self), mutating (&mut self), or consuming (self)*
        //* https://doc.rust-lang.org/book/ch05-03-method-syntax.html
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { //other: immutable borrow
        self.height > other.height && self.width > other.height
    }
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

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 20
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn area(rectangle: &Rectangle) -> u32 {
    //rectangle: immutable borrow of a struct Rectangle instance
    //want to borrow the struct rather than take ownership of it so main() can continue to use rect1
    rectangle.width * rectangle.height
}