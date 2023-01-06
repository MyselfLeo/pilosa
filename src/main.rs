mod big_uint;
mod digit;
mod big_num;

mod core;

use big_num::BigNum;
use big_uint::BigUInt;










fn main() {
    let b1 = vec![1, 2, 3, 5, 6, 3, 7, 1, 9, 0, 0, 4];
    let b2 = vec![1];

    println!("{:?}", core::u_mul(b2, b1));
}


/*
fn main() {
    let b1 = BigNum::from_string("99").unwrap();
    let b2 = BigNum::one();

    println!("{b1} + {b2} = {}", &b1 + &b2);
}*/