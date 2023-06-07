use std::ops;

use crate::field;

// overflow-safe 127 bit rational type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rat {
    pub num: i64,
    pub den: i64
}

impl Rat {
    pub fn new(val: i64) -> Rat {
        Rat {
            num: val,
            den: 1
        }
    }

    pub fn try_int(&self) -> Option<i64> {
        if self.den == 1 {
            Some(self.num)
        } else {
            None
        }
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
}

impl From<i64> for Rat {
    fn from(val: i64) -> Self {
        Self {
            num: val,
            den: 1
        }
    }
}

impl From<Rat> for f64 {
    fn from(val: Rat) -> f64 {
        val.num as f64 / val.den as f64
    }
}

impl TryFrom<Rat> for i64 {
    type Error = ();

    fn try_from(val: Rat) -> Result<i64, Self::Error> {
        if val.den == 1 {
            Ok(val.num)
        } else {
            Err(())
        }
    }
}

impl field::Zero for Rat {
    fn zero() -> Self {
        Self {
            num: 0,
            den: 1
        }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }
}

impl field::One for Rat {
    fn one() -> Self {
        Self {
            num: 1,
            den: 1
        }
    }
}

impl ops::Add<Rat> for Rat {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self {
        loop {
            let den_gcd = gcd(self.den, rhs.den);

            let lhs_num = match (rhs.den / den_gcd).checked_mul(self.num) {
                Some(v) => v,
                None => {
                    self.num >>= 1;
                    self.den >>= 1;

                    continue;    
                }
            };
    
            let rhs_num = match (self.den / den_gcd).checked_mul(rhs.num) {
                Some(v) => v,
                None => {
                    rhs.num >>= 1;
                    rhs.den >>= 1;

                    continue;
                }
            };
    
            let num = match lhs_num.checked_add(rhs_num) {
                Some(v) => v,
                None => {
                    if self.num > rhs.num {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1;
                    }

                    continue;
                }
            };
    
            let den = match (self.den / den_gcd).checked_mul(rhs.den) {
                Some(v) => v,
                None => {
                    if self.den > rhs.den {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1; 
                    }    

                    continue;
                }
            };

            let new_gcd = gcd(num, den).abs();
    
            if den > 0 {
                return Self { num: num / new_gcd, den: den / new_gcd }    
            } else {
                return Self { num: - num / new_gcd, den: - den / new_gcd }
            }
        }
    }
}

impl ops::Sub<Rat> for Rat {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        loop {
            let den_gcd = gcd(self.den, rhs.den);

            let lhs_num = match (rhs.den / den_gcd).checked_mul(self.num) {
                Some(v) => v,
                None => {
                    self.num >>= 1;
                    self.den >>= 1;

                    continue;    
                }
            };
    
            let rhs_num = match (self.den / den_gcd).checked_mul(rhs.num) {
                Some(v) => v,
                None => {
                    rhs.num >>= 1;
                    rhs.den >>= 1;

                    continue;
                }
            };
    
            let num = match lhs_num.checked_sub(rhs_num) {
                Some(v) => v,
                None => {
                    if self.num > rhs.num {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1;
                    }

                    continue;
                }
            };
    
            let den = match (self.den / den_gcd).checked_mul(rhs.den) {
                Some(v) => v,
                None => {
                    if self.den > rhs.den {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1; 
                    }    

                    continue;
                }
            };

            let new_gcd = gcd(num, den).abs();
    
            if den > 0 {
                return Self { num: num / new_gcd, den: den / new_gcd }    
            } else {
                return Self { num: - num / new_gcd, den: - den / new_gcd }
            }
        }
    }
}

impl ops::Mul<Rat> for Rat {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        loop {
            let lhs_gcd = gcd(self.num, rhs.den);
            let rhs_gcd = gcd(rhs.num, self.den);
            
            let num = (self.num / lhs_gcd).checked_mul(rhs.num / rhs_gcd);
            let den = (self.den / rhs_gcd).checked_mul(rhs.den / lhs_gcd);
    
            if let (Some(num), Some(den)) = (num, den) {
                return Self { num, den }
            } else if self.num > rhs.num {
                self.num >>= 1;
                self.den >>= 1;
            } else {
                rhs.num >>= 1;
                rhs.den >>= 1;
            }    
        }
    }
}

impl ops::Mul<i64> for Rat {
    type Output = Self;

    fn mul(mut self, rhs: i64) -> Self {
        if self.den % rhs == 0 {
            self.den /= rhs;
        } else {
            self.num *= rhs;
        }

        self
    }
}

impl ops::Div<Rat> for Rat {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self {
        loop {
            let num_gcd = gcd(self.num, rhs.num);
            let den_gcd = gcd(rhs.den, self.den);
            
            let num = (self.num / num_gcd).checked_mul(rhs.den / den_gcd);
            let den = (self.den / den_gcd).checked_mul(rhs.num / num_gcd);
    
            if let (Some(num), Some(den)) = (num, den) {
                return Self { num, den }
            } else if self.num > rhs.den {
                self.num >>= 1;
                self.den >>= 1;
            } else {
                rhs.num >>= 1;
                rhs.den >>= 1;
            }    
        }
    }
}

// Euclid's algorithm
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut shift = 0;

    while (a | b) & 0x1 == 0 {
        a >>= 1;
        b >>= 1;
        shift += 1;
    }

    let mut t: i64;

    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }

    a << shift
}

#[cfg(test)]
mod tests {
    use super::Rat;

    #[test]
    fn arith() {
        let a = Rat::new(1);
        let b = Rat::new(2);
        assert_eq!((a + b).num, 3);
        assert_eq!((a + b).den, 1);

        assert_eq!((a - b).num, -1);
        assert_eq!((a + b).den, 1);

        assert_eq!(((a + b) * (a - b) + b).num, -1);
    }

    #[test]
    fn overflow() {
        let a = Rat { num: (i64::MAX >> 1) + 1, den: i64::MAX };
        let b = Rat { num: (i64::MAX >> 1) + 3, den: i64::MAX };
        let c = a + b;
        assert_eq!(1., c.into());
        assert_eq!(f64::from(a) + f64::from(b), f64::from(c));
    }
}