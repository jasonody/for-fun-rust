fn main() {
    //---Ownership---
    let s1 = String::from("some string");
    let _s2 = s1;

    //println!("{}, {}", s1, _s2); //cannot do this: ownership of s1 has transfered to s2
    //s1 has an invalidated reference

    let s2 = _s2.clone(); //create a copy on the heap
    println!("{}, {}", _s2, s2);

    let s3 = String::from("some other string");
    takes_ownership(s3);
    //println!("{}", s3); //cannot do this: ownership of s3 has transfered to the takes_ownership function

    let x1: i32 = 5;
    makes_copy(x1); //i32 has Copy trait so you can still use it after makes_copy()
    println!("Can print x1 again: {}", x1);

    //---Borrowing---
    //What if you want to still use a variable after passing it to a function?
    //Pass a reference instead (refer to a value without taking ownership of is):
    let s4 = String::from("hi");
    let len = calculate_length(&s4);
    println!("length of \"{}\" is {}",s4, len);

    //---Mutable References---
    let mut s5 = String::from("hi");
    change_string(&mut s5);
    println!("changed string: {}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", person!");
}