const LYRICS: [&'static str; 15] = [
    "On the",
    "day of Christmas",
    "My true love gave to me",
    "12 drummers drumming",
    "Eleven pipers piping",
    "Ten lords a leaping",
    "Nine ladies dancing",
    "Eight maids a milking",
    "Seven swans a swimming",
    "Six geese a laying",
    "Five gold rings, badam-pam-pam",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "And a partridge in a pear tree"
];

const ORDINALS: [&'static str; 12] = [
    "First",
    "Second",
    "Third",
    "Forth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eighth",
    "Ninth",
    "Tenth",
    "Eleventh",
    "Twelfth"
];

fn main() {
    println!("{} {} {}", LYRICS[0], ORDINALS[0], LYRICS[1]);
    println!("{}", LYRICS[2]);
    println!("A partridge in a pear tree\n");

    for i in 1..12 {
        println!("{} {} {}", LYRICS[0], String::from(ORDINALS[i]).to_lowercase(), LYRICS[1]);
        println!("{}", LYRICS[2]);

        for j in 14-i..15 {
            println!("{}", LYRICS[j]);
        };
        
        println!();
    }
}
