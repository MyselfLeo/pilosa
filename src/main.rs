mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;

fn main() {
    let b1 = BigNum::from_string("1242005").unwrap();
    let b2 = BigNum::from_string("0.23352").unwrap();

    println!("{b1} + {b2} = {}", BigNum::add(&b1, &b2));
    println!("{b1} - {b2} = {}", BigNum::sub(&b1, &b2));
    println!("{b1} * {b2} = {}", BigNum::mul(&b1, &b2));
}