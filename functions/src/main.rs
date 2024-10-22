fn main() {
    println!("Hello, world!");
    another_function(20);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("five() returns: {}", five());
}

fn another_function(x: i32) {
    println!("Another Hello, world!, {} century", x);
}
fn five() -> i32 {
    5
}