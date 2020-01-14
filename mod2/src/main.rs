mod lib;
use std::io;

fn main() {


    let mut inp1 = String ::new();
    println!("***ENTER CITY'S LANDLINE CODE***");
    io::stdin().read_line(&mut inp1).expect("FAILED TO READ LINE");
    let inp2:u32 = inp1.trim().parse().expect("INVALID DATA TYPE");


    if inp2 == 051
    {
        lib::cities::capital::islamabad();
    }
    else
    {
        println!("NOT IN THE LIBRARY");
    }

}
