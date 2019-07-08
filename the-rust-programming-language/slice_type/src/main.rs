fn main() {
    //The slice does not take ownership
    //Slices let you reference a contiguous sequesnce of elements in a collection
    //rather than the whole collection

    //write a function that takes a string and returns the first word it finds in that string. 
    //If the function doesn’t find a space in the string, the whole string must be one word, 
    //so the entire string should be returned:

    let mut s = String::from("hello Joe");
    let first_word_index = first_word_index(&s);
    s.clear();
    println!("{}", first_word_index);
    //first_word_index works in the above example, but it's not great
    //as the example shows, s is set to "", which means the value in first_word_index is no longer accurate
    //this is because s has no connection to first_word_index

    //use string slicing
    //a string slice is a reference to part of a string
    let example = String::from("hello world");
    let hello = &example[0..5]; //(or [..5]) position 0 to 5, not including 5
    let world = &example[6..11]; //(or [6..]) position 6 to 10, not including 11 (note: last position is 10)
    //&example[..] would be a slice of the whole string
    println!("{} {}", hello, world);

    let mut _s1 = String::from("hello joe");
    let first_word = first_word(&_s1); //first_word has borrowed s1 as immutable
    //s1.clear(); //cannot borrow `s1` as mutable because it is also borrowed as immutable
    println!("{}", first_word);

    //other slices
    let array = [1, 2, 3, 4, 5];
    let _array_slice = &array[..3]; //this slice has type &[i32]
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { //enumerate returns a tuple: (index, ref to item)
        if item == b' ' { //byte literal syntax
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str { //&str indicates a string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        } 
    }

    &s[..]
}