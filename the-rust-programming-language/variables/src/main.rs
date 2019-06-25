const MAX_POINTS: u32 = 100_000; //same as 100,000

fn main() {
    //Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The values of x is: {}", x);

    //Constants
    println!("Max Points: {}", MAX_POINTS);

    //Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    //y = 10; //This will not compile as y is immutable

    let spaces = "     ";
    let spaces = spaces.len(); //Can chance the type of a variable when shadowing
    println!("The value of space is: {}", spaces);

    //let mut more_spaces = "     ";
    //more_spaces = more_spaces.len(); //This will not compile as len() and more_spaces are mismatched types. Only can do this when the first is shadowed by the second
}
