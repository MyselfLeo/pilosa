mod big_num;
mod core;

use big_num::BigNum;









fn main() {
    let b1 = BigNum::from_string("1875649871641587948568765213546681").unwrap();
    let b2 = BigNum::from_string("1564685").unwrap();

    println!("{}", BigNum::bn_div(&b1, &b2));
}