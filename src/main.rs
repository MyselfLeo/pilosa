mod big_num;
mod core;

use big_num::BigNum;


use std::env;







fn main() {
    // case where it should be normalized
    // let b1 = vec![0, 0, 3];
    // let b2 = vec![2, 1];

    // case normalized
    let b1 = vec![9, 9, 9];
    let b2 = vec![5, 3];

    println!("{:?}", core::ub_div(b1, b2));
}


/*
fn main() {
    let args: Vec<String> = env::args().collect();

    let b1 = BigNum::from_string(&args[1]).unwrap();
    let b2 = BigNum::from_string(&args[2]).unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
} */