mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;

fn main() {
    let n1 = BigNum::from_string(".12304").unwrap();
    let n2 = BigNum::from_string("134453.345").unwrap();

    println!("n1: {:?}", n1);
    println!("n2: {:?}", n2);

    println!("n1 + n2 = {}", BigNum::add(&n1, &n2))
}
