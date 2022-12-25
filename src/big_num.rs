use crate::big_uint::BigUInt;
use crate::digit::Digit;



const IMPLICIT_SIGN: bool  = false;



#[derive(Clone, Debug)]
pub struct BigNum {
    negative: bool,
    abs: BigUInt,
    power: u32
}



impl BigNum {
    fn new(negative: bool, abs: BigUInt, power: u32) -> BigNum {
        BigNum { negative, abs, power }
    }


    /// Takes a number as a string (ex: -512.3245)
    /// and return the corresponding BigNum
    pub fn from_string(origin_string: &str) -> Result<BigNum, String> {
        let mut string = origin_string.replace(" ", "");
        if string.is_empty() {return Err("Empty string".to_string())}

        // some => sign specified (false or true), none => sign not specified (IMPLICIT_SIGN)
        let negative = match string.chars().nth(0) {
            Some('-') => Some(true),
            Some('+') => Some(false),
            _ => None
        };

        if negative.is_some() {string.remove(0);}

        // find a potential dot, and from its position in the string compute the power
        let mut power = 0;
        let mut dot_found = false;
        for (i, c) in string.chars().enumerate() {
            if c == '.' && dot_found {return Err("Invalid format".to_string())}
            else if c == '.' {
                power = string.len() - i - 1;
                dot_found = true;
            }
        }
        string = string.replace(".", "");

        let abs = BigUInt::from_string(&string).or(Err(format!("Invalid format: {origin_string}")))?;
        Ok(BigNum::new(negative.unwrap_or(IMPLICIT_SIGN), abs, power as u32))
    }



    




    /// Modify the given bignums so they have the same power
    fn same_power(n1: &mut BigNum, n2: &mut BigNum) {
        if n1.power < n2.power {n1.with_power(n2.power)}
        else {n2.with_power(n1.power)}
    }


    pub fn mul(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;
        let abs = BigUInt::mul(&n1.abs, &n2.abs);
        let pow = n1.power + n2.power;

        let mut res = BigNum { negative: sign, abs: abs, power: pow };
        res.clean();

        res
    }



    pub fn add(n1: &BigNum, n2: &BigNum) -> BigNum {
        // values must be of the same type
        if n1.negative != n2.negative {unimplemented!()}

        let mut n1 = n1.clone();
        let mut n2 = n2.clone();
        BigNum::same_power(&mut n1, &mut n2);

        // Create the new value
        let sum = BigUInt::sum(&n1.abs, &n2.abs);
        let mut res = BigNum {
            negative: n1.negative, // n2.negative would work too (as n1.negative == n2.negative)
            abs: sum,   
            power: n1.power        // idem
        };

        res.clean();
        res
    }



    /// Remove decimal zeroes, reducing the power in the same time
    fn clean(&mut self) {
        if self.abs.digits.is_empty() {return}
        let check = |x: &mut BigNum| x.abs.digits.first().is_some() && x.abs.digits.first().unwrap().as_u8() == 0 && x.power > 0;
        while check(self) {
            self.power -= 1;
            self.abs.digits.remove(0);
        }
    }

    
    /// Increase the power of the BigNum to the required value, add zeroes to match
    fn with_power(&mut self, n: u32) {
        if self.power >= n {return;}

        while self.power != n {
            self.power += 1;
            self.abs.digits.insert(0, Digit::from_u8(0));
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