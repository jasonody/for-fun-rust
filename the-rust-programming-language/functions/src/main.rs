fn main() {
    another_function(5, "Booyah!");
    fun_expression();

    println!("ten(): {}", ten());
    println!("6 plus_one(): {}", plus_one(6));
}

fn another_function(x: i32, message: &str) {
    println!("The value of x is: {}", x);
    println!("The message is: {}", message);
}

fn fun_expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 //no ";" indicates this value is being returned (making this an expression and not a statement)
    };

    println!("The fun expression value of y is: {}", y);
    println!("The fun expression value of x is: {}", x);
}

fn ten() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
}