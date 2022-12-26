mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;
use big_uint::BigUInt;

fn main() {
    let b1 = BigUInt::from_string("1222").unwrap();
    let b2 = BigUInt::from_string("234").unwrap();

    println!("{b1} - {b2} = {}", BigUInt::sub(&b1, &b2));
}