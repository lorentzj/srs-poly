use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops;

use crate::poly::mono::*;
use crate::poly::*;

impl<T: Field> ops::Add<Poly<T>> for Poly<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.terms.is_empty() {
            return rhs;
        }

        if rhs.terms.is_empty() {
            return self;
        }

        let mut new_terms = VecDeque::from(vec![]);

        let mut lhs_term_iter = self.terms.into_iter().peekable();
        let mut rhs_term_iter = rhs.terms.into_iter().peekable();

        loop {
            if let Some(lhs_term) = lhs_term_iter.peek() {
                if let Some(rhs_term) = rhs_term_iter.peek() {
                    match grevlex(lhs_term, rhs_term) {
                        Ordering::Equal => {
                            let new_val = lhs_term.val.clone() + rhs_term.val.clone();
                            if !new_val.is_zero() {
                                new_terms.push_back(Mono {
                                    val: new_val,
                                    vars: lhs_term.vars.clone(),
                                });
                            }
                            lhs_term_iter.next();
                            rhs_term_iter.next();
                        }

                        Ordering::Greater => {
                            new_terms.push_back(rhs_term.clone());
                            rhs_term_iter.next();
                        }
                        Ordering::Less => {
                            new_terms.push_back(lhs_term.clone());
                            lhs_term_iter.next();
                        }
                    }
                } else {
                    new_terms.push_back(lhs_term.clone());
                    lhs_term_iter.next();
                }
            } else if let Some(rhs_term) = rhs_term_iter.next() {
                new_terms.push_back(rhs_term);
            } else {
                break;
            }
        }

        Self { terms: new_terms }
    }
}

impl<T: Field> ops::Sub<Poly<T>> for Poly<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.terms.is_empty() {
            return Self::constant(-1) * rhs;
        }

        if rhs.terms.is_empty() {
            return self;
        }

        let mut new_terms = VecDeque::from(vec![]);

        let mut lhs_term_iter = self.terms.into_iter().peekable();
        let mut rhs_term_iter = rhs.terms.into_iter().peekable();

        loop {
            if let Some(lhs_term) = lhs_term_iter.peek() {
                if let Some(rhs_term) = rhs_term_iter.peek() {
                    match grevlex(lhs_term, rhs_term) {
                        Ordering::Equal => {
                            let new_val = lhs_term.val.clone() - rhs_term.val.clone();

                            if !new_val.is_zero() {
                                new_terms.push_back(Mono {
                                    val: new_val,
                                    vars: lhs_term.vars.clone(),
                                });
                            }
                            lhs_term_iter.next();
                            rhs_term_iter.next();
                        }

                        Ordering::Greater => {
                            let mut rhs_term = rhs_term_iter.next().unwrap();
                            rhs_term.val = T::zero() - rhs_term.val;
                            new_terms.push_back(rhs_term);
                        }
                        Ordering::Less => {
                            new_terms.push_back(lhs_term.clone());
                            lhs_term_iter.next();
                        }
                    }
                } else {
                    new_terms.push_back(lhs_term.clone());
                    lhs_term_iter.next();
                }
            } else if let Some(mut rhs_term) = rhs_term_iter.next() {
                rhs_term.val = T::zero() - rhs_term.val;
                new_terms.push_back(rhs_term);
            } else {
                break;
            }
        }

        Self { terms: new_terms }
    }
}

impl<T: Field> ops::Mul<Poly<T>> for Poly<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mul_ref(&rhs)
    }
}

impl<T: Field> Poly<T> {
    pub fn mul_ref(&self, other: &Poly<T>) -> Poly<T> {
        let mut new = Self::constant(0);

        for lhs_term in &self.terms {
            for rhs_term in &other.terms {
                let new_term = Poly {
                    terms: VecDeque::from(vec![monomial_mul(lhs_term, rhs_term)]),
                };

                new = new + new_term;
            }
        }

        new
    }

    pub fn compound_divide(&self, divisors: &Vec<Poly<T>>) -> (Vec<Poly<T>>, Poly<T>) {
        if divisors.is_empty() {
            return (vec![], self.clone());
        }

        let mut dividend = self.clone();

        let mut rem = Poly::constant(0);
        let mut quotients: Vec<_> = std::iter::repeat(Poly::constant(0))
            .take(divisors.len())
            .collect();

        let mut curr_divisor = 0;

        while !dividend.is_zero() {
            let self_lt = dividend.terms[0].clone();
            if !divisors[curr_divisor].terms.is_empty() {
                let div_lt = &divisors[curr_divisor].terms[0];
                let self_over_div_lt = monomial_div(&self_lt, div_lt);

                if let Some(self_over_div_lt) = self_over_div_lt {
                    quotients[curr_divisor]
                        .terms
                        .push_back(self_over_div_lt.clone());

                    let self_over_div_lt = Poly {
                        terms: VecDeque::from(vec![self_over_div_lt]),
                    };

                    dividend = dividend - (self_over_div_lt.mul_ref(&divisors[curr_divisor]));
                    curr_divisor = 0;
                } else {
                    curr_divisor += 1;
                }
            } else {
                curr_divisor += 1;
            }

            if curr_divisor == divisors.len() {
                let self_lt = Poly {
                    terms: VecDeque::from(vec![self_lt.clone()]),
                };

                dividend.terms.pop_front();

                rem = rem + self_lt;
                curr_divisor = 0;
            }
        }

        (quotients, rem)
    }

    pub fn try_divide(&self, divisor: &Poly<T>) -> Option<Poly<T>> {
        let (quots, rem) = self.compound_divide(&vec![divisor.clone()]);

        if rem.is_zero() {
            Some(quots[0].clone())
        } else {
            None
        }
    }

    pub fn derivative(&self, by: usize) -> Poly<T> {
        let mut new_terms = VecDeque::new();
        for term in &self.terms {
            let mut new_term = Mono {
                val: term.val.clone(),
                vars: vec![],
            };
            let mut found = false;
            for (var, pow) in &term.vars {
                if *var == by {
                    found = true;
                    if *pow > 1 {
                        new_term.val = new_term.val * *pow as i64;
                        new_term.vars.push((*var, *pow - 1));
                    }
                } else {
                    new_term.vars.push((*var, *pow));
                }
            }

            if found {
                new_terms.push_back(new_term);
            }
        }

        Poly { terms: new_terms }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use std::rc::Rc;
    use crate::rational::Rat;
    use super::Poly;

    #[test]
    fn arith() {
        let var_dict = Rc::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        let a: Poly<Rat> = Poly::var(0, 2) * Poly::constant(3);
        let b = Poly::var(1, 1) * Poly::constant(4);
        let c = Poly::constant(2);

        assert_eq!("3a^2 + 4b + 2", (c + b + a).format(&var_dict));

        // (a + 1)(a + 1)
        let a: Poly<Rat> = (Poly::var(0, 1) + Poly::constant(1)) * (Poly::var(0, 1) + Poly::constant(1));
        // a^2 + 2a + 1
        let b = Poly::var(0, 2) + Poly::constant(2) * Poly::var(0, 1) + Poly::constant(1);

        assert!(a == b);
    }

    #[test]
    fn arith_fuzz() {
        let mut rng = SmallRng::seed_from_u64(1);

        fn create_random_poly(rng: &mut SmallRng, term_max: i32) -> Poly<Rat> {
            let mut p = Poly::constant(0);

            for _ in 0..rng.gen_range(0..term_max + 1) {
                let coef = rng.gen_range(-6..6);
                let xpow = rng.gen_range(0..4);
                let ypow = rng.gen_range(0..2);
                let zpow = rng.gen_range(0..3);

                p = p + Poly::constant(coef)
                    * Poly::var(0, xpow)
                    * Poly::var(1, ypow)
                    * Poly::var(2, zpow);
            }

            p
        }

        for _ in 0..1000 {
            let dividend = create_random_poly(&mut rng, 6);
            let n_divs = rng.gen_range(0..4);
            let divisors: Vec<_> = std::iter::repeat_with(|| create_random_poly(&mut rng, 4))
                .take(n_divs)
                .collect();

            let (quotients, rem) = dividend.clone().compound_divide(&divisors);

            let calculated_dividend = quotients
                .clone()
                .into_iter()
                .zip(divisors.clone())
                .fold(Poly::constant(0), |acc, (x, y)| acc + x * y)
                + rem.clone();

            assert_eq!(calculated_dividend, dividend);
        }
    }

    #[test]
    fn derivative() {
        let var_dict = vec!["x".to_string(), "y".to_string(), "z".to_string()];

        let p: Poly<Rat> = Poly::var(0, 2) * Poly::var(1, 2) * Poly::constant(3)
            + Poly::var(0, 1) * Poly::var(2, 1);

        assert_eq!(
            "6xy^2 + z",
            format!("{}", p.derivative(0).format(&var_dict))
        );
    }
}
