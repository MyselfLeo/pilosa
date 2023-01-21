mod big_num;
mod core;

use big_num::BigNum;









fn main() {
    let b1 = BigNum::from_string("123456789123456789123456789").unwrap();
    let b2 = BigNum::from_string("1992113").unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
}