fn main() {
    let width = 30;
    let height = 20;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: usize, height: usize) -> usize {
    width * height
}
