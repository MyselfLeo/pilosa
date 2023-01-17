mod big_num;
mod core;

use big_num::BigNum;









fn main() {
    let b1 = vec![0, 5, 3, 4];
    let b2 = vec![0, 6];

    println!("{:?}", core::ub_div(b1, b2));
}