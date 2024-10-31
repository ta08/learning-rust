#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 20,
    };

    println!("rectangle is {:?}", rectangle);
}

fn area(width: usize, height: usize) -> usize {
    width * height
}
