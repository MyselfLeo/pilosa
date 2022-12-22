use crate::big_uint::BigUInt;


pub struct BigNum {
    negative: bool,
    abs: BigUInt,
    power: u32
}



impl BigNum {
    pub fn new(negative: bool, abs: BigUInt, power: u32) -> BigNum {
        BigNum { negative, abs, power }
    }


    pub fn mul(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;
        let abs = BigUInt::mul(&n1.abs, &n2.abs);
        let pow = n1.power + n2.power;

        BigNum { negative: sign, abs: abs, power: pow }
    }
}




impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negative {write!(f, "-")?};
        
        let nb_digits = self.abs.digits.len(); 
        let dot_pos = nb_digits - self.power as usize;
        for i in 0..nb_digits {
            if i == dot_pos {write!(f, ".")?};
            write!(f, "{}", self.abs.digits[nb_digits - i - 1].as_char())?;
        };

        Ok(())
    }
}