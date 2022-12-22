mod big_uint;
mod digit;
//mod big_num;

use big_uint::BigUInt;

fn main() {
    let n1 = BigUInt::from_string("0");
    let n2 = BigUInt::from_string("1224");

    println!("n1 = {n1}");
    println!("n2 = {n2}");
    println!("n1 * n2 = {}", BigUInt::mul(&n1, &n2));
}
