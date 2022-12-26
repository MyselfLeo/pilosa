mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;
use rand::random;

const TRIES: i32 = 100000;

fn main() {

    let mut success = 0;

    for _ in 0..TRIES {
        let n1_i32: i32 = random();
        let n2_i32: i32 = random();

        let n1 = BigNum::from_i32(n1_i32).unwrap();
        let n2 = BigNum::from_i32(n2_i32).unwrap();

        let expected = n1_i32 < n2_i32;
        let result = BigNum::is_lower(&n1, &n2);

        if expected == result {success += 1}
        else {
            println!("Wrong result on   {}   <   {}", n1, n2);
        }
    }

    println!("Result: {success}/{TRIES} ({}%)", (success as f64 / TRIES as f64) * 100.0);
}
