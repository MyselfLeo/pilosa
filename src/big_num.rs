use crate::big_uint::BigUInt;
use crate::digit::Digit;



const IMPLICIT_SIGN: bool  = false;



#[derive(Clone, Debug)]
pub struct BigNum {
    negative: bool,
    pub abs: BigUInt,
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


    pub fn from_i32(n: i32) -> Result<BigNum, String> {
        BigNum::from_string(&n.to_string())
    }
    pub fn from_f64(n: f64) -> Result<BigNum, String> {
        BigNum::from_string(&n.to_string())
    }



    

    /// Modify the given bignums so they have the same power
    fn same_power(n1: &mut BigNum, n2: &mut BigNum) {
        if n1.power < n2.power {n1.with_power(n2.power)}
        else {n2.with_power(n1.power)}
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



    /// Return the opposite of this BigNum
    pub fn opposite(&self) -> BigNum {
        BigNum { negative: !self.negative, abs: self.abs.clone(), power: self.power }
    }



    /// Return true if n1 == n2
    /// Will not work if both BigNums are not cleaned
    /// => BigNums MUST be cleaned after each operation
    pub fn are_equal(n1: &BigNum, n2: &BigNum) -> bool {
        n1.negative == n2.negative && n1.abs == n2.abs && n1.power == n2.power
    }


    /// Return true if n1 < n2
    pub fn is_lower(n1: &BigNum, n2: &BigNum) -> bool {
        // easy cmp of signs
        //println!("1");
        if n1.negative && !n2.negative {return true}
        else if !n1.negative && n2.negative {return false}
        //println!("2");

        // if both are negative, calculations may vary
        let neg = n1.negative && n2.negative;

        //println!("3: {neg}");

        // easy cmp with the number of whole digits
        if n1.abs.digits.len() - n1.power as usize != n2.abs.digits.len() - n2.power as usize {
            //println!("4");
            if neg {return (n1.abs.digits.len() - n1.power as usize) > (n2.abs.digits.len() - n2.power as usize)}
            else {return (n1.abs.digits.len() - n1.power as usize) < (n2.abs.digits.len() - n2.power as usize)}
        }

        //println!("5");

        // Same amount of digits before the '.', so we can compare each digit one by one
        let min_len = std::cmp::min(n1.abs.digits.len(), n2.abs.digits.len());

        //println!("6: {min_len}");

        let len_n1 = n1.abs.digits.len();
        let len_n2 = n2.abs.digits.len();

        for i in 0..min_len {

            //println!("7.{i}.1");

            let d1 = &n1.abs.digits[len_n1 - i - 1];
            let d2 = &n2.abs.digits[len_n2 - i - 1];


            //println!("7.{i}.2: {:?} {:?}", d1, d2);


            if neg {
                //println!("7.{i}.3");
                if d1 < d2 {return false}
                if d1 > d2 {return true}
            }
            else {
                //println!("7.{i}.4");
                if d1 < d2 {return true}
                if d1 > d2 {return false}
            }
        }

        //println!("8");

        // at this point we reach the end of at least one BigNum
        if neg {
            //println!("9");
            n2.abs.digits.len() - 1 == min_len && n1.abs.digits.len() - 1 != min_len
        }
        else {
            //println!("10");
            n1.abs.digits.len() - 1 == min_len && n2.abs.digits.len() - 1 != min_len
        }
    }


    /// Return true if n1 > n2
    pub fn is_greater(n1: &BigNum, n2: &BigNum) -> bool {
        !BigNum::are_equal(n1, n2) && !BigNum::is_lower(n1, n2)
    }




    /// Return the multiplication of 2 BigNums
    pub fn mul(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;
        let abs = BigUInt::mul(&n1.abs, &n2.abs);
        let pow = n1.power + n2.power;

        let mut res = BigNum { negative: sign, abs: abs, power: pow };
        res.clean();

        res
    }



    /// Return the sum of 2 BigNums of the same sign.
    pub fn inner_add(n1: &BigNum, n2: &BigNum) -> BigNum {
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



    /// Return the diff of 2 positive BigNums.
    /// panics if n1 < n2
    pub fn inner_sub(n1: &BigNum, n2: &BigNum) -> BigNum {
        todo!()
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