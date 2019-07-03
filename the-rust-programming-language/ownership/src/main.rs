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
    //let r0 = &mut s4; //cannot borrow `s4` as mutable, as it is not declared as mutable
    let len = calculate_length(&s4);
    println!("length of \"{}\" is {}",s4, len);

    //---Mutable References---
    let mut s5 = String::from("hi");
    change_string(&mut s5);
    println!("changed string: {}", s5);

    //Mutation in a controller manner (prevent data races)
    let mut s6 = String::from("hey ya");
    let _r1 = &mut s6;
    _r1.push('y');
    //let _r2 = &mut s6; //cannot borrow `s6` as mutable more than once at a time
    //println!("{} {}", _r1, _r2);

    {
        let _r3 = &mut s6;
    }// _r3 goes out of scope here so a new reference can be made with no problem
    // both will not be in-scope at the same time
    let _r4 = &mut s6;
    //let _r5 = &mut _r4; //cannot borrow `_r4` as mutable, as it is not declared as mutable

    //Combining mutable and immutable references
    let mut _s7 = String::from("yo yo");
    let _r5 = &_s7; //OK (immutable reference--read only)
    let _r6 = &_s7; //OK (immutable reference--read only)
    //let _r7 = &mut s7; //NOT OK:cannot borrow `s7` as mutable because it is also borrowed as immutable
    //println!("{} {} {}", _r5, _r6, _r7);

    let mut s8 = String::from("hey hey");
    let r7 = &s8; //OK (immutable reference--read only)
    let r8 = &s8; //OK (immutable reference--read only)
    println!("{} {}", r7, r8); 
    //r7 and r8 are not used again
    let r9 = &mut s8; //OK as scopes of r7 & r8 don't overlap with r9
    println!("r9:{}", r9);
    r9.push('9');
    //println!("r9:{}, s8:{}", r9, s8); //cannot borrow `s8` as immutable because it is also borrowed as mutable
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