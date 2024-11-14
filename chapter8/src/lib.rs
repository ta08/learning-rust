mod chapter81 {
    pub fn sample81() {
        // let xs: Vec<i32> = Vec::new();
        let mut ys = vec![1.2, 2.0, 3.3, 4.0, 5.9];

        for y in &ys {
            println!("{}", y);
        }

        ys.push(1.2);
        println!("{:?}", ys);

        for y in &ys {
            println!("{}", y);
        }

    }
}

pub fn function_chapter81() {
    chapter81::sample81();
}