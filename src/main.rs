mod big_uint;
mod digit;

use big_uint::BigUInt;

fn main() {
    let n1 = BigUInt::from_string("00001234");
    let n2 = BigUInt::from_string("45670000");

    println!("n1 = {n1}");
    println!("n2 = {n2}");
    println!("n1 + n2 = {}", BigUInt::sum(&n1, &n2));
}
