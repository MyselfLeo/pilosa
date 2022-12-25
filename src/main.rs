mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;

fn main() {
    let n1 = BigNum::from_string(".12304").unwrap();
    let n2 = BigNum::from_string("134453.345").unwrap();

    println!("{n1} * {n2} = {}", BigNum::mul(&n1, &n2))
}
