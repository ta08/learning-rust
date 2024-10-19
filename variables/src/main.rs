fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

}

fn old_main(){
    let mut x = 5;
    println!("Hello, world! {} times", x);
    x = 4;
    println!("Hello, world! {} times", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MaxPoints: {} ", MAX_POINTS);

    println!("getNumber: {} ", getNumber());


}
fn getNumber() -> u32 {
    return 3;
}
