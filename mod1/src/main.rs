use std::io;

mod sports {
    pub mod badminton {
        pub fn score (one:u32,two:u32) {

            let total = one + two;
            println!("TOTAL SCORE IS: {}",total);

        }
        
    }
}


use crate::sports::badminton;
//use crate::hr::interview::intr; //idiomatic path do not use this
//use crate::hr::attendance;

fn main() {

    let mut round1 = String ::new();
    println!("****ENTER SCORE FOR ROUND 1****");
    io::stdin().read_line(&mut round1).expect("FAILED TO READ LINE");
    let round_1:u32 = match round1.trim().parse()  //.expect("INVALID DATA TYPE");
    {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut round2 = String ::new();
    println!("\n****ENTER SCORE FOR ROUND 2****");
    io::stdin().read_line(&mut round2).expect("FAILED TO READ LINE");
    let round_2:u32 = match round2.trim().parse()  //.expect("INVALID DATA TYPE");
    {
        Ok(num) => num,
        Err(_) => 0,
    };
    badminton::score(round_1,round_2);
}

