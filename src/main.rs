mod big_num;
mod core;

use big_num::BigNum;









fn main() {
    let b1 = vec![0, 0, 1];
    let b2 = vec![5, 0, 0];

    println!("{:?}", core::ub_sub(b1, b2));
}