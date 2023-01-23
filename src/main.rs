mod big_num;
mod core;

use big_num::BigNum;


use std::env;







/*
fn main() {
    let b1 = BigNum::from_string("123456789123456789123456789").unwrap();
    let b2 = BigNum::from_string("1992113").unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
} */



fn main() {
    let args: Vec<String> = env::args().collect();

    let b1 = BigNum::from_string(&args[1]).unwrap();
    let b2 = BigNum::from_string(&args[2]).unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
}