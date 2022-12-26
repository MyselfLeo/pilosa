use crate::digit::Digit;

#[derive(Clone, PartialEq)]
pub struct BigUInt {
    pub digits: Vec<Digit> // from least to most significant
}


impl BigUInt {
    /// panics if the string is not a natural number
    pub fn from_string(v: &str) -> Result<BigUInt, ()> {
        let mut big_uint = BigUInt {digits: vec![]};
        for c in v.chars() {
            big_uint.digits.insert(0, Digit::from_char(c)?)
        };

        Ok(big_uint)
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



    // May not be useful for now
    /*
    /// Return true if n1 < n2
    pub fn is_lower(n1: &BigUInt, n2: &BigUInt) -> bool {
        if n1.digits.len() < n2.digits.len() {return true}
        if n1.digits.len() > n2.digits.len() {return false}

        for (d1, d2) in std::iter::zip(&n1.digits, &n2.digits) {
            if d1 < d2 {return true}
            if d1 > d2 {return false}
        }

        false
    } */




    pub fn mul(n1: &BigUInt, n2: &BigUInt) -> BigUInt {
        let mut res = BigUInt {digits: vec![]};


        // apply the multiplication for each digit of n1
        // sum those results to get the final result
        for (i, d1) in n1.digits.iter().enumerate() {
            let mut local_res = BigUInt {digits: vec![]};
            let mut carry: u8 = 0;

            for d2 in &n2.digits {
                let mul = d1.as_u8() * d2.as_u8() + carry;
                
                local_res.digits.push(Digit::from_u8(mul % 10));
                carry = mul / 10;
            }

            // add the carry
            if carry != 0 {local_res.digits.push(Digit::from_u8(carry))}

            // insert i zeroes in local_res (multiplying it by 10^i)
            for _ in 0..i {local_res.digits.insert(0, Digit::from_u8(0))}

            res = BigUInt::sum(&res, &local_res);
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



impl std::fmt::Debug for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            write!(f, "{}", d.as_char())?;
        }
        Ok(())
    }
}