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

    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("Nothing to do")
    }

    if let Some(x) = some_u8_value {
        println!("Some number {}", x);
    } else {
        println!("Nothing to do 2");
    }

    if let Some(3) = some_u8_value {
        println!("Some number ");
    } else {
        println!("Nothing to do 2");
    }

}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
