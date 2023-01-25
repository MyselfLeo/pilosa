use std::ops::{Add, Sub, Mul};

use crate::core::{self, ub_clean};



const IMPLICIT_SIGN: bool  = false;
const FLOAT_PRECISION: i64 = 10;



#[derive(Clone, Debug)]
pub struct BigNum { // todo: remove pub
    pub negative: bool,
    pub abs: Vec<u8>,
    pub power: u32
}



impl BigNum {
    fn new(negative: bool, abs: Vec<u8>, power: u32) -> BigNum {
        let mut res = BigNum {negative, abs, power};
        res.clean();
        res
    }
    
    pub fn zero() -> BigNum {BigNum {negative: false, abs: vec![0], power: 0}}
    pub fn one() -> BigNum {BigNum {negative: false, abs: vec![1], power: 0}}



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

        // convert string of digits (ex: 12345) to vec of digits from least to most significant ([5, 4, 3, 2, 1])
        let abs = string
            .chars()
            .rev()
            .map(|c| c.to_digit(10).and_then(|d| Some(d as u8)))
            .collect::<Option<Vec<u8>>>();

        match abs {
            None => Err("Invalid format".to_string()),
            Some(mut a) => {
                ub_clean(&mut a);
                Ok(BigNum::new(negative.unwrap_or(IMPLICIT_SIGN), a, power as u32))
            }
        }
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








    /// Reduce the power as much possible by removing useless decimal zeroes (0.10 => 0.1)
    fn clean(&mut self) {
        if self.abs.is_empty() {return}

        // decimal zeroes (12.120 => 12.12)
        let check = |x: &mut BigNum| x.abs.first().is_some() && x.abs.first().unwrap() == &0 && x.power > 0;
        while check(self) {
            self.power -= 1;
            self.abs.remove(0);
        }
    }

    
    /// Increase the power of the BigNum to the required value, add zeroes to match
    fn with_power(&mut self, n: u32) {
        if self.power >= n {return;}

        while self.power != n {
            self.power += 1;
            self.abs.insert(0, 0);
        }
    }




    /// Return the opposite of this BigNum
    pub fn opposite(&self) -> BigNum {
        BigNum { negative: !self.negative, abs: self.abs.clone(), power: self.power }
    }



    /// Return true if n1 == n2
    /// Will not work if both BigNums are not cleaned
    /// => BigNums MUST be cleaned after each operation
    fn are_equal(n1: &BigNum, n2: &BigNum) -> bool {
        n1.negative == n2.negative && n1.abs == n2.abs && n1.power == n2.power
    }


    /// Return true if n1 < n2
    fn is_lower(n1: &BigNum, n2: &BigNum) -> bool {
        // easy cmp of signs
        if n1.negative && !n2.negative {return true}
        else if !n1.negative && n2.negative {return false}

        // if both are negative, calculations may vary
        let neg = n1.negative && n2.negative;

        // easy cmp with the number of whole digits
        if n1.abs.len() - n1.power as usize != n2.abs.len() - n2.power as usize {
            if neg {return (n1.abs.len() - n1.power as usize) > (n2.abs.len() - n2.power as usize)}
            else {return (n1.abs.len() - n1.power as usize) < (n2.abs.len() - n2.power as usize)}
        }

        // Same amount of digits before the '.', so we can compare each digit one by one
        let min_len = std::cmp::min(n1.abs.len(), n2.abs.len());
        let len_n1 = n1.abs.len();
        let len_n2 = n2.abs.len();

        for i in 0..min_len {
            let d1 = &n1.abs[len_n1 - i - 1];
            let d2 = &n2.abs[len_n2 - i - 1];

            if neg {
                if d1 < d2 {return false}
                if d1 > d2 {return true}
            }
            else {
                if d1 < d2 {return true}
                if d1 > d2 {return false}
            }
        }

        // at this point we reach the end of at least one BigNum
        if neg {
            len_n2 == min_len && len_n1 != min_len
        }
        else {
            len_n1 == min_len && len_n2 != min_len
        }
    }




    /// Return true if n1 > n2
    fn is_greater(n1: &BigNum, n2: &BigNum) -> bool {
        !BigNum::are_equal(n1, n2) && !BigNum::is_lower(n1, n2)
    }



    /// Return the BigNum multiplied by 10^power
    pub fn bn_tenpow_mul(n: &BigNum, power: usize) -> BigNum {
        // result values
        let mut final_power = n.power;
        let mut abs = n.abs.clone();


        let mut power = power;
        while power > 0 {
            if final_power > 0 {final_power -= 1;}
            else {abs.insert(0, 0);}
            power -= 1;
        }

        let mut res = BigNum {negative: n.negative, abs: abs, power: final_power};
        res.clean();
        res
    }



    /// Return the BigNum divided by 10^power
    pub fn bn_tenpow_div(n: &BigNum, power: usize) -> BigNum {
        // very simple function as we only need to increase
        // the n.power by power
        let mut res = BigNum {negative: n.negative, abs: n.abs.clone(), power: n.power + power as u32};
        res.clean();
        res
    }




    /// Return the multiplication of 2 BigNums
    pub fn bn_mul(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;
        let abs = core::ub_mul(n1.abs.clone(), n2.abs.clone());
        let pow = n1.power + n2.power;

        let mut res = BigNum { negative: sign, abs: abs, power: pow };
        
        res.clean();
        res
    }




    /// Return the euclidian quotient and remainder of num / denom
    pub fn euclidian(num: &BigNum, denom: &BigNum) -> (BigNum, BigNum) {
        let mut remainder = num.clone();
        let mut quotient = BigNum::zero();

        while &remainder >= &denom {
            remainder = &remainder - denom;
            quotient = quotient + BigNum::one();
        }

        (quotient, remainder)
    }






    /// Return the sum of 2 BigNums of the same sign.
    fn inner_add(n1: &BigNum, n2: &BigNum) -> BigNum {
        if n1.negative != n2.negative {panic!("inner_add can only add BigNums of the same sign")}

        let mut n1 = n1.clone();
        let mut n2 = n2.clone();
        BigNum::same_power(&mut n1, &mut n2);

        // Create the new value
        let sum = core::ub_add(n1.abs, n2.abs);
        let mut res = BigNum {
            negative: n1.negative, // n2.negative would work too (as n1.negative == n2.negative)
            abs: sum,   
            power: n1.power        // same
        };

        res.clean();
        res
    }






    /// Return the diff of 2 positive BigNums.
    /// panics if n1 < n2
    fn inner_sub(n1: &BigNum, n2: &BigNum) -> BigNum {
        if n1.negative || n2.negative {panic!("inner_sub can only substract positive BigNums")}
        if n1 < n2 {panic!("inner_sub requires n1 > n2")}

        let mut n1 = n1.clone();
        let mut n2 = n2.clone();

        BigNum::same_power(&mut n1, &mut n2);

        let mut res = BigNum::new(false, core::ub_sub(n1.abs, n2.abs), n1.power);

        res.clean();
        res
    }



    


    /// Add two BigNums
    pub fn bn_add(n1: &BigNum, n2: &BigNum) -> BigNum {
        // Transform the addition in order to use inner_add (addition of same sign)
        // or inner_sub (substraction of positive BigNums)
        match (n1.negative, n2.negative) {
            (false, false) => { // x + y
                BigNum::inner_add(n1, n2)
            },
            (true, true) => { // -x + -y
                BigNum::inner_add(n1, n2)
            },
            (true, false) => { // -x + y <=> y - x
                n2 - &n1.opposite()
            },
            (false, true) => { // x + -y <=> x - y
                n1 - &n2.opposite()
            },
        }
    }






    /// Substract two BigNums
    pub fn bn_sub(n1: &BigNum, n2: &BigNum) -> BigNum {
        match (n1.negative, n2.negative) {
            (false, false) => {
                if n1 < n2 {BigNum::inner_sub(n2, n1).opposite()} // require n1 > n2 :    (x-y) <=> -(y-x)
                else {BigNum::inner_sub(n1, n2)}
            },
            (true, true) => {  // -x - -y <=> y - x
                &n2.opposite() - &n1.opposite()
            },
            (true, false) => { // -x - y <=> -x + -y
                n1 + &n2.opposite()
            },
            (false, true) => { // x - -y <=> x + y
                n1 + n2
            },
        }
    }








    /// Divide one BigNum by another
    pub fn bn_div(n1: &BigNum, n2: &BigNum) -> BigNum {
        let sign = n1.negative != n2.negative;
        let pow = n1.power as i64 - n2.power as i64; // pow can be negative. If so it will be modified after the division


        let mut n1 = n1.clone();
        let n2 = n2.clone();



        // increase the power of n1 so that n1.power - n2.power >= FLOAT_PRECISION
        let delta = FLOAT_PRECISION - pow;
        if delta > 0 {n1.with_power(n1.power + delta as u32);}


        let quotient = if n2.abs.len() == 1 {
            core::ub_shortdiv(n1.abs, n2.abs[0]).0
        } else {
            core::ub_div(n1.abs, n2.abs).0
        };
        
        assert!(n1.power - n2.power > 0, "resulting power is negative");

        // return the cleaned result
        let mut res = BigNum { negative: sign, abs: quotient, power: n1.power - n2.power as u32};
        res.clean();

        res
    }


}




impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.negative {write!(f, "-")?};
        
        let nb_digits = self.abs.len(); 
        let mut dot_pos = nb_digits as isize - self.power as isize;

        // special case if |self| < 1
        if dot_pos <= 0 {
            write!(f, "0.")?;
            while dot_pos < 0 {
                write!(f, "0")?;
                dot_pos += 1;
            }
        }
        for i in 0..nb_digits {
            if i == dot_pos as usize && i > 0 {write!(f, ".")?};
            write!(f, "{}", self.abs[nb_digits - i - 1])?;
        };

        Ok(())
    }
}




impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        BigNum::are_equal(self, other)
    }
}


impl PartialOrd for BigNum {
    fn lt(&self, other: &Self) -> bool {
        BigNum::is_lower(self, other)
    }

    fn le(&self, other: &Self) -> bool {
        !BigNum::is_greater(self, other)
    }

    fn gt(&self, other: &Self) -> bool {
        BigNum::is_greater(self, other)
    }

    fn ge(&self, other: &Self) -> bool {
        !BigNum::is_lower(self, other)
    }


    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if BigNum::are_equal(self, other) {Some(std::cmp::Ordering::Equal)}
        else if BigNum::is_lower(self, other) {Some(std::cmp::Ordering::Less)}
        else {Some(std::cmp::Ordering::Greater)}
    }

}






impl Add for &BigNum {
    type Output = BigNum;

    fn add(self, rhs: Self) -> Self::Output {
        BigNum::bn_add(self, rhs)
    }
}
impl Add for BigNum {
    type Output = BigNum;

    fn add(self, rhs: Self) -> Self::Output {
        BigNum::bn_add(&self, &rhs)
    }
}


impl Sub for &BigNum {
    type Output = BigNum;

    fn sub(self, rhs: Self) -> Self::Output {
        BigNum::bn_sub(self, rhs)
    }
}
impl Sub for BigNum {
    type Output = BigNum;

    fn sub(self, rhs: Self) -> Self::Output {
        BigNum::bn_sub(&self, &rhs)
    }
}


impl Mul for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: Self) -> Self::Output {
        BigNum::bn_mul(self, rhs)
    }
}
impl Mul for BigNum {
    type Output = BigNum;

    fn mul(self, rhs: Self) -> Self::Output {
        BigNum::bn_mul(&self, &rhs)
    }
}