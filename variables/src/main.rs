
fn main() {
    println!("Hello, world!");

    let xs = [1,2,3,];
    let [x ,y , z] = xs;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let xs = (1,2,3,);
    let (x ,y , z) = xs;
    println!("x: {}, y: {}, z: {}", x, y, z);
}