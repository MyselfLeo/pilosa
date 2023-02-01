pub mod big_num;
mod core;

use big_num::BigNum;
use crate::core::ub_div;










fn main() {

    let n1 = BigNum::from_string("1224.235").unwrap();
    let n2 = BigNum::from_string("12").unwrap();

    //println!("{:?}", ub_div(vec![5, 3, 2, 4, 2, 2, 1], vec![2, 1]));

    println!("{}", BigNum::bn_div(&n1, &n2).unwrap());
}