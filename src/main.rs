mod big_num;
mod core;

use big_num::BigNum;


use std::env;
use std::process::exit;




fn main() {
    let b1 =  BigNum::from_string("208752").unwrap();
    println!("{}", BigNum::bn_tenpow_div(&b1, 5));
}





/*
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: requires 2 numbers");
        exit(1);
    }

    let b1 = BigNum::from_string(&args[1]).unwrap();
    let b2 = BigNum::from_string(&args[2]).unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
}*/