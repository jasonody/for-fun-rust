enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(_four);
    route(IpAddrKind::V6);

    let _home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", _home);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(_ip_kind: IpAddrKind) { }