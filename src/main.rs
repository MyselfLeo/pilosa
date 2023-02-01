pub mod big_num;
mod core;

use big_num::BigNum;
use crate::core::ub_div;










fn main() {

    let n1 = BigNum::from_string("53643.368359").unwrap();
    let n2 = BigNum::from_string("24872398247.24982").unwrap();


    println!("{}", BigNum::bn_sub(&n1, &n2));
}