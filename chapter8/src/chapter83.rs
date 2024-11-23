/**
 * Create
 */

pub mod chapter83mod {
    pub fn chapter83function() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Yellow"), 30);

        println!("{:?}", scores);
    }
}
