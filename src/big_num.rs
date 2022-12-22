use crate::big_uint::BigUInt;


pub struct BigNum {
    negative: bool,
    abs: BigUInt,
    power: u32
}



impl BigNum {


    pub fn mul(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;

        
    }
}