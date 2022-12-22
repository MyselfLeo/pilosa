use core::panic;

pub struct Digit {
    value: u8 // strictly between 0 and 9 (obviously)
}

impl Digit {

    /// Sum the 2 digits and return it along with if it created a carry or not
    pub fn sum(n1: &Digit, n2: &Digit) -> (Digit, bool) {
        let sum = n1.as_u8() + n2.as_u8();
        let digit = Digit::from_u8(sum % 10);
        let carry = sum >= 10;

        (digit, carry)
    }





    /// panics if n is not a digit
    pub fn from_u8(n: u8) -> Digit {
        if n >= 10 {panic!("Given a non-digit number: '{n}'")}
        Digit {value: n}
    }

    /// panics if c is not a digit
    pub fn from_char(c: char) -> Digit {
        let v: u8 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Given a non-digit char")
        };

        Digit::from_u8(v)
    }


    pub fn as_u8(&self) -> u8 {
        self.value
    }

    pub fn as_char(&self) -> char {
        match self.value {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => unreachable!()
        }
    }
}