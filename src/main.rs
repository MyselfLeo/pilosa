mod big_num;
mod core;

use big_num::BigNum;









fn main() {
    let b1 = vec![9, 5, 3, 0];
    let b2 = vec![0, 1];

    println!("{:?}", core::ub_div(b1, b2));
}