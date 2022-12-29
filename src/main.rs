mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;
use big_uint::BigUInt;



fn main() {
    let b1 = BigNum::from_string("12124").unwrap();
    let b2 = BigNum::from_string("34").unwrap();

    let (quo, res) = BigNum::euclidian(&b1, &b2);

    println!("{b1} // {b2} = {quo}");
    println!("{b1} % {b2} = {res}");
}


/*
fn main() {
    let b1 = BigNum::from_string("99").unwrap();
    let b2 = BigNum::one();

    println!("{b1} + {b2} = {}", &b1 + &b2);
}*/