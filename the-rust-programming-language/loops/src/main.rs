fn main() {
    loop_demo();
    while_demo();
    for_demo();
}

fn loop_demo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
    println!("The counter is {}", counter);
}

fn while_demo() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!!!");
}

fn for_demo() {
    let a  = [10, 20, 30, 50, 50];

    for element in a.iter() {
        println!("the value from the array is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}