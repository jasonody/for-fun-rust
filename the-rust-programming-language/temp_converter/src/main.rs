use std::io;

fn main() {
    loop {
        println!("Enter a value for temperature converstion (type \"Q\" to exit):");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == "Q" {
            break;
        }

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("{}°C is {}°F", input, convert_c_to_f(input));
        println!("{}°F is {}°C", input, convert_f_to_c(input));
    }
}

fn convert_c_to_f(input: f32) -> f32 {
    (input * 9f32/5f32) + 32f32
}

fn convert_f_to_c(input: f32) -> f32 {
    (input - 32f32) * 5f32/9f32
}