fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world! {:?}", four);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}