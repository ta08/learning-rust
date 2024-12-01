use std::{fs::File, io::ErrorKind};

fn main() {
    // println!("Hello, world!");
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("File opened successfully"),
        Err(ref error)=> println!("Error opening file: {}", error),
    }
    
    

}
