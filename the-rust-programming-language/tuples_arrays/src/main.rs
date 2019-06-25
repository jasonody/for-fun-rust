fn main() {
    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); //type annotations are optional
    let (x, y, z) = tup; //destructuring (because it breaks the single tuple into three parts)
    println!("x: {}, y: {}, z: {}", x, y, z);

    println!("tuple direct access: first: {}, second: {}, third: {}", tup.0, tup.1, tup.2);

    //Arrays
    //Fixed in size--can't grow or shrink
    let a: [i32; 5] = [-2, -1, 0, 1, 2]; //optional type and length annotation
    println!("{}", a[0]);

    let b = [3; 5]; // 5 elements set to the value of 3: [3, 3, 3, 3, 3, 3]
    println!("{}", b[4]);
}
