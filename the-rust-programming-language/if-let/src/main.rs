fn main() {
    let some_u8_value = Some(0u8);

    //match has exhaustive checking
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => () //do nothing
    }

    //if let does not have exhaustive checking
    //hink of if let as syntax sugar for a match that runs code when the value 
    //matches one pattern and then ignores all other values
    if let Some(3) = some_u8_value {
        println!("Three");
    }

    if let Some(3) = some_u8_value {
        println!("Three")
    } else { //optional else (same as "_ => ..." in a match expression)
        println!{"Not three"}
    }
}
