mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;
use big_uint::BigUInt;

fn main() {
    let n1 = BigNum::new(false, BigUInt::from_string("123"), 2); // 1.23
    let n2 = BigNum::new(true, BigUInt::from_string("100"), 0);  // -100

    println!("n1 * n2 = {}", BigNum::mul(&n1, &n2))
}
