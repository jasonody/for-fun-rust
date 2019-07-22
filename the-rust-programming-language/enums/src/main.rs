enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        //define method body here
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(_four);
    route(IpAddrKind::V6);

    let _home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", _home);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Say hello"));
    m.call();

    //Option enum
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None; //must tell the compiler the type as it cannot infer from None

    let x: i8 = 8;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;
    //let sum = x + y; //Compiler won't allow you to add x to y because y could be None (aka null)
    let sum = x + match y { Some(i) => i, None => 0 };
    println!("Sum is: {}", sum);
    let none_sum = x + match z { Some(i) => i, None => 0};
    println!("None sum is: {}", none_sum);
}

fn route(_ip_kind: IpAddrKind) { }