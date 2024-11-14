
pub mod chapter82mod {
    pub fn chapter82function(){
        // let s = String::from("aiueo");
        // println!("{:?}", s);
        //
        // let something = "something desu";
        //
        // let _y = something.to_string();
        //
        // let d = something.find(|x|  true);
        //
        // let mut double_something = something.to_string();
        // double_something.push_str(something);
        // println!("{:}", something);
        // println!("{:}", &double_something);
        // println!("{:}", _y);
        //
        //

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s4 = s1 + &s2 + &s3;
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("ver print {}", s);
        // println!("ver print {}", s4);
    }
}