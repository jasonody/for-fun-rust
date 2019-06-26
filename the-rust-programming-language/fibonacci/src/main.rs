use std::io;

fn main() {
    loop {
        println!("What Fibonacci number do you want? (Type \"q\" to exit)");

        let mut value = String::new();

        io::stdin().read_line(&mut value)
            .expect("Unable to read line");

        if value.trim() == "q" {
            break;
        }
        
        let value:i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("{}th Fibonacci (iterative) number is: {}", value, fib_iterative(value));
        println!("{}th Fibonacci (iterative_1) number is: {}", value, fib_iterative_1(value));
        println!("{}th Fibonacci (recursive) number is: {}", value, fib_recursive(value)); //visibly slower at ~39+
    }
}

fn fib_recursive(n:i32) -> i64 {
    if n > 1 {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    } else {
        n.into()
    }
}

fn fib_iterative(n:i32) -> i64 {
    if n > 1 {
        let mut sum = 0;
        let mut last = 0;
        let mut curr = 1;

        for _ in 1..n {
            sum = last + curr;
            last = curr;
            curr = sum;
        }
        sum
    } else {
        n.into()
    }
}

fn fib_iterative_1(n:i32) -> i64 {
    if n > 1 {
        let mut fib_pair = [0, 1];
        let mut temp: i64;

        for _ in 1..n {
            temp = fib_pair[1];
            fib_pair[1] = fib_pair[0] + fib_pair[1];
            fib_pair[0] = temp;
        }
        fib_pair[1]
    } else {
        n.into()
    }
}