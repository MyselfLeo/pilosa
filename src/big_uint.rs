use crate::digit::Digit;

pub struct BigUInt {
    digits: Vec<Digit> // from least to most significant
}


impl BigUInt {
    /// panics if the string is not a natural number
    pub fn from_string(v: &str) -> BigUInt {
        let mut big_uint = BigUInt {digits: vec![]};
        for c in v.chars() {
            big_uint.digits.insert(0, Digit::from_char(c))
        };

        big_uint
    }


    pub fn sum(n1: &BigUInt, n2: &BigUInt) -> BigUInt {
        let mut res = BigUInt {digits: vec![]};
        let mut carry = 0;

        let mut i = 0;
        loop {
            let mut sum;
            if i < n1.digits.len() && i < n2.digits.len() {
                let (tmp_sum, mut tmp_carry) = Digit::sum(&n1.digits[i], &n2.digits[i]);
                sum = tmp_sum.as_u8() + carry;
                if sum >= 10 {
                    sum = sum % 10;
                    tmp_carry = true;
                }
                carry = if tmp_carry {1} else {0};
            }
            else if i < n1.digits.len() {
                sum = &n1.digits[i].as_u8() + carry;
                carry = 0;
            }
            else if i < n2.digits.len() {
                sum = &n2.digits[i].as_u8() + carry;
                carry = 0;
            }
            else {break;}

            res.digits.push(Digit::from_u8(sum));
            i += 1;
        }

        if carry != 0 {
            res.digits.push(Digit::from_u8(carry))
        }

        res
    }
}





impl std::fmt::Display for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            write!(f, "{}", d.as_char())?;
        }
        Ok(())
    }
}