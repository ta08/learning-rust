use std::env;
fn main() {
    
}

fn main32(){
    let guess:u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
}

fn main31(){
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

    println!("get_number: {} ", get_number());


}
fn get_number() -> u32 {
    return 3;
}
