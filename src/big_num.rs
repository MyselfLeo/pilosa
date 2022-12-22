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

        let mut res = BigNum { negative: sign, abs: abs, power: pow };
        res.clean();

        res
    }



    /// Remove decimal zeroes, reducing the power in the same time
    pub fn clean(&mut self) {
        if self.abs.digits.is_empty() {return}
        let check = |x: &mut BigNum| x.abs.digits.first().is_some() && x.abs.digits.first().unwrap().as_u8() == 0 && x.power > 0;
        while check(self) {
            self.power -= 1;
            self.abs.digits.remove(0);
        }
    }
}




impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negative {write!(f, "-")?};
        
        let nb_digits = self.abs.digits.len(); 
        let dot_pos = nb_digits - self.power as usize;

        // leading 0 if |self| < 1
        if dot_pos == 0 {write!(f, "0")?;}

        for i in 0..nb_digits {
            if i == dot_pos {write!(f, ".")?};
            write!(f, "{}", self.abs.digits[nb_digits - i - 1].as_char())?;
        };

        Ok(())
    }
}