use library;
use std::io;

fn main() {
    println!("***ENTER YOUR MOVIE GENRE CHOICE***");
    println!("1. ACTION");
    println!("2. SCIENCE FICTION");

    let mut op = String ::new();
    //println!("***ENTER CITY'S LANDLINE CODE***");
    io::stdin().read_line(&mut op).expect("FAILED TO READ LINE");
    let op2:u32 = match op.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 0,
    };
    

    //let one = library::movie::genre::actn();
    //let two = library::movie::genre::sci_fi();
    match op2
    {
        1 => library::movie::genre::actn(),
        2 => library::movie::genre::sci_fi(),
        _ => println!("UNVALID OPTION")
    };
}
