mod big_uint;
mod digit;
mod big_num;

use big_num::BigNum;
use rand::random;



const TRIES: i32 = 100000;

fn main() {

    let mut success = 0;
    let mut lengths = Vec::new();

    for _ in 0..TRIES {
        let n1_num: f64 = random();
        let n2_num: f64 = random();

        let n1 = BigNum::from_f64(n1_num).unwrap();
        let n2 = BigNum::from_f64(n2_num).unwrap();

        let expected = n1_num < n2_num;
        let result = BigNum::is_lower(&n1, &n2);

        if expected == result {success += 1}
        else {
            println!("Wrong result on   {}   <   {}", n1, n2);
            if !lengths.contains(&n1.abs.digits.len()) {lengths.push(n1.abs.digits.len())}
        }
    }

    println!("Result: {success}/{TRIES} ({}%)", (success as f64 / TRIES as f64) * 100.0);
    println!("Lengths: {:?}", lengths);
}