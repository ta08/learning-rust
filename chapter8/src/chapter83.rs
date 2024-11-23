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

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        scores.entry(String::from("Yellow")).or_insert(20);
        println!("{:?}", scores);
    }
}
