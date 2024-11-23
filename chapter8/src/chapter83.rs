/**
 * Create
 */


pub mod chapter83mod {
    use itertools::Itertools;

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

        let xs = vec![1,3,5,2,9,10];
    
        println!("mean result {:?}", mean(&xs));

        // let xs = vec![1,3,5,2,9,10];
        println!("median result {:?}", median(xs));
    }


    fn mean(list: &Vec<i32>) -> f64 {
        let total: i32 = list.iter().sum();
        total as f64  / list.len() as f64
    }

    fn median(mut list: Vec<i32>) -> f64 {
        list.sort();
        let mid = list.len() / 2;
        if list.len() % 2 == 0 {
            (list[mid - 1] as f64 + list[mid] as f64) / 2.0
        } else {
            list[mid] as f64
        }
    }
}
