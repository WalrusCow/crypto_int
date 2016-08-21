/// 512 bit unsigned integers.

use std::cmp;
use std::fmt;
use std::ops;

use super::arithmetic;

#[derive(Clone, Debug)]
pub struct U512 {
    // These are stored with the least significant 64 bits first.
    digits: Vec<u64>,
}

// TODO
// shr
// BitOr
// BitAnd
// BitXor
// Not
// *Assign
impl U512 {
    fn literal(digits: Vec<u64>) -> U512 {
        debug_assert_eq!(digits.len(), 8);
        U512 {
            digits: digits,
        }
    }

    pub fn from_u64(x: u64) -> U512 {
        U512::literal(vec![x, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn from_bytes_be(bytes: Vec<u8>) -> U512 {
        assert!(bytes.len() <= 64);
        let mut bytes = bytes;
        bytes.reverse();

        let mut digits: Vec<u64> = Vec::with_capacity(8);
        for chunk in bytes.chunks(8) {
            let mut x = 0u64;
            for (i, byte) in chunk.iter().enumerate() {
                x |= (*byte as u64) << i * 8;
            }
            digits.push(x);
        }

        while digits.len() < 8 {
            digits.push(0);
        }
        U512::literal(digits)
    }

    pub fn zero() -> U512 {
        U512::literal(vec![0, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn is_zero(&self) -> bool {
        self.digits[0] == 0 && self.digits[1] == 0 &&
            self.digits[2] == 0 && self.digits[3] == 0
    }
}

impl ops::Add for U512 {
    type Output = U512;
    fn add(mut self, rhs: U512) -> U512 {
        arithmetic::add(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::AddAssign for U512 {
    fn add_assign(&mut self, rhs: U512) {
        arithmetic::add(&mut self.digits, &rhs.digits);
    }
}

impl ops::Sub for U512 {
    type Output = U512;
    fn sub(mut self, rhs: U512) -> U512 {
        arithmetic::sub(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::SubAssign for U512 {
    fn sub_assign(&mut self, rhs: U512) {
        arithmetic::sub(&mut self.digits, &rhs.digits);
    }
}

impl ops::Mul for U512 {
    type Output = U512;
    fn mul(self, rhs: U512) -> U512 {
        let v = arithmetic::mul(&self.digits, &rhs.digits);
        U512::literal(v[..8].to_vec())
    }
}

impl ops::Rem for U512 {
    type Output = U512;
    fn rem(self, rhs: U512) -> U512 {
        let (_, rem) = arithmetic::div_rem(&self.digits, &rhs.digits);
        U512::literal(rem)
    }
}

impl ops::Div for U512 {
    type Output = U512;
    fn div(self, rhs: U512) -> U512 {
        let (quotient, _) = arithmetic::div_rem(&self.digits, &rhs.digits);
        U512::literal(quotient)
    }
}

impl ops::Shl<usize> for U512 {
    type Output = U512;
    fn shl(self, rhs: usize) -> U512 {
        U512::literal(arithmetic::shl(&self.digits, rhs))
    }
}

impl fmt::Display for U512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Think of a better way to print this...
        write!(f, "{:0>#018x}{:0>16x}{:0>16x}{:0>16x}",
               self.digits[3], self.digits[2],
               self.digits[1], self.digits[0])
    }
}

impl cmp::PartialEq for U512 {
    fn eq(&self, other: &U512) -> bool {
        self.digits == other.digits
    }

    fn ne(&self, other: &U512) -> bool {
        self.digits != other.digits
    }
}

impl cmp::Eq for U512 {}

impl cmp::Ord for U512 {
    fn cmp(&self, other: &U512) -> cmp::Ordering {
        arithmetic::cmp(&self.digits, &other.digits)
    }
}

impl cmp::PartialOrd for U512 {
    fn partial_cmp(&self, other: &U512) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let zero_1 = U512::zero();
        let zero_2 = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        let five_1 = U512::from_u64(5);
        let five_2 = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        ]);

        assert_eq!(zero_1, zero_2);
        assert_eq!(five_1, five_2);
    }

    #[test]
    fn addition() {
        let x = U512::from_u64(10);
        let y = U512::from_u64(12);
        assert_eq!(x + y, U512::from_u64(22));

        let x = U512::from_bytes_be(vec![
            0x01, 0xff, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
            0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        ]);
        let y = U512::from_bytes_be(vec![
            0x00, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0xab, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x01, 0x18,
        ]);
        let expected = U512::from_bytes_be(vec![
            0x02, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x02, 0x17,
        ]);
        let ans = x + y;
        assert_eq!(ans, expected);

        let x = U512::from_u64(187236152);
        let y = U512::from_u64(187236152);
        assert_eq!(x, y + U512::zero());
    }

    #[test]
    fn subtraction() {
        let x = U512::from_u64(10);
        let y = U512::from_u64(12);
        assert_eq!(y - x, U512::from_u64(2));

        let x = U512::from_bytes_be(vec![
            0x02, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x02, 0x17,
        ]);
        let y = U512::from_bytes_be(vec![
            0x00, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0xab, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x01, 0x18,
        ]);
        let expected = U512::from_bytes_be(vec![
            0x01, 0xff, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        ]);
        let ans = x - y;
        assert_eq!(ans, expected);

        let x = U512::from_u64(7192478999);
        let y = U512::from_u64(7192478999);
        assert_eq!(x, y - U512::zero());
    }

    #[test]
    fn multiplication() {
        let x = U512::from_u64(20);
        let y = U512::from_u64(16);
        assert_eq!(x * y, U512::from_u64(20 * 16));

        // Got these numbers from testing in Python
        let x = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x13, 0x2f, 0x40, 0xb7, 0x63,
            0x50, 0xe4, 0x7c, 0xcd, 0x9a, 0x5f, 0x4e, 0xa2,
        ]);
        let y = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x6f, 0x6c, 0x08, 0xeb, 0xf4,
            0x47, 0x5f, 0x5b, 0xdb, 0x28, 0xc7, 0x8d, 0x29,
        ]);
        let expected = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x59,
            0x95, 0xa9, 0xfa, 0x22, 0x7f, 0x94, 0x5c, 0xf4,
            0x80, 0x65, 0xd0, 0x3f, 0x78, 0x3c, 0xe1, 0xea,
            0xfd, 0xe0, 0xf9, 0xe9, 0xa7, 0x80, 0xd1, 0xf2,
        ]);
        assert_eq!(x * y, expected);

        let x = U512::from_u64(7192478999);
        let y = U512::from_u64(7192478999);
        assert_eq!(x, y * U512::from_u64(1));
    }

    #[test]
    fn remainder() {
        let x = U512::from_u64(13);
        let y = U512::from_u64(7);
        assert_eq!(y % x, U512::from_u64(7));

        for i in 0..60 {
            let x = U512::from_u64(13 + 7 * i);
            let y = U512::from_u64(7);
            assert_eq!(x % y, U512::from_u64(6));
        }

        let x = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xbc, 0x86, 0x00, 0x8f, 0xff, 0x85, 0x3f, 0x8e,
            0xc6, 0x0a, 0x0b, 0xb4, 0xd0, 0x36, 0x26, 0xfc,
            0x44, 0x7c, 0xf3, 0x2a, 0x45, 0x2c, 0xd0, 0x1c,
        ]);

        let y = U512::from_bytes_be(vec![
            0x1b, 0x50, 0xdd, 0xa8, 0x70, 0x14, 0xa2, 0x7d,
            0x4b, 0xd4, 0xe8, 0xcb, 0x1d, 0xfa, 0xe7, 0xfc,
            0xbe, 0x5a, 0x68, 0x53, 0x24, 0x01, 0x92, 0xc1,
            0x55, 0x22, 0xbc, 0x55, 0x2e, 0xc5, 0xc8, 0x9a,
        ]);


        let expected = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x1b, 0x55, 0x59, 0x52, 0xf3, 0x27, 0x1f, 0xb2,
            0xb7, 0xaa, 0x52, 0x3a, 0x8d, 0x33, 0x0c, 0x7d,
            0x8d, 0xa1, 0xcd, 0xb1, 0x8f, 0x80, 0x29, 0x5e,
        ]);
        assert_eq!(y % x, expected);
    }

    #[test]
    fn division() {

        for i in 0..45 {
            for j in 0..15 {
                let x = U512::from_u64(i);
                let y = U512::from_u64(j + 1);
                assert_eq!(x / y, U512::from_u64(i / (j + 1)));
            }
        }


        let x = U512::from_bytes_be(vec![
            0x1b, 0xcc, 0x2c, 0x7b, 0x2c, 0x29, 0x41, 0x9d,
            0x16, 0xb8, 0x07, 0xcf, 0x3c, 0x41, 0x44, 0xba,
            0x5f, 0x4a, 0x89, 0xf6, 0xd0, 0x34, 0xdb, 0xc7,
            0x21, 0x0a, 0x23, 0x28, 0xac, 0x0e, 0x53, 0x04,
        ]);

        let y = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0xdc, 0xda,
            0x32, 0xcf, 0x64, 0x03, 0xd0, 0xea, 0xe4, 0x85,
            0x1a, 0x80, 0x29, 0x3c, 0xb2, 0x4f, 0x32, 0x3f,
        ]);


        let expected = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xf6, 0x8d, 0x74, 0xc3, 0xf6,
            0x77, 0x71, 0xf1, 0x3d, 0x16, 0x46, 0x9e, 0x17,
        ]);
        assert_eq!(x / y, expected);
    }

    #[test]
    fn shl() {
        let x = U512::from_u64(1);
        let y = U512::from_u64(64);
        assert_eq!(x << 6, y);

        let x = U512::from_bytes_be(vec![
            0x1b, 0xcc, 0x2c, 0x7b, 0x2c, 0x29, 0x41, 0x9d,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x21, 0x0a, 0x23, 0x28, 0xac, 0x0e, 0x53, 0x04,
        ]);

        let y = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b,
            0xcc, 0x2c, 0x7b, 0x2c, 0x29, 0x41, 0x9d, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21,
            0x0a, 0x23, 0x28, 0xac, 0x0e, 0x53, 0x04, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        assert_eq!(x << 72, y);
    }

}
