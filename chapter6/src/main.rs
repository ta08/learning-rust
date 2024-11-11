fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world! {:?}", four);

    let guess = Some(5);

    let maybeSquare = match guess {
        Some(x) => x * x,
        None => 0,
    };
    println!("The square of the guess is: {}", maybeSquare);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(u8,u8,u8,u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}