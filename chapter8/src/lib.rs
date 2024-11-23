use chapter82::chapter82mod::chapter82function;
use chapter83::chapter83mod::chapter83function;

mod chapter82;

mod chapter83;

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

        // not work this one
        // for y in &ys {
        //     *y *= 10.0;
        //     println!("{}", y);
        // }

        for y in &mut ys {
            *y *= 10.0;
            println!("{}", y);
        }

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];


        for r in &row {
            match r {
                SpreadsheetCell::Int(x) => {println!("Int! {}", x);}
                SpreadsheetCell::Float(x) => {println!("Float! {}", x);}
                SpreadsheetCell::Text(x) => {println!("Textttt! {}", x);}
            }
        }

        let zs = {
            let x = 3;
            let mut ds = vec![1, 2, 3];
            ds.push(x);
            ds.iter().map(|x| x * 2).collect::<Vec<i32>>()
        };

        for z in &zs {
            println!("{}", z);
        }

    }
}
pub fn function_chapter82(){
    chapter82function();
}

pub fn function_chapter81() {
    chapter81::sample81();
}

pub fn function_chapter83(){
    chapter83function();
}
