use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops;
use std::rc::Rc;

use crate::mono::*;
use crate::poly::*;

impl ops::Add<Poly> for Poly {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        assert!(Rc::ptr_eq(&self.var_dict, &rhs.var_dict));

        let mut new_terms = VecDeque::from(vec![]);

        let mut lhs_term_iter = self.terms.into_iter().peekable();
        let mut rhs_term_iter = rhs.terms.into_iter().peekable();

        loop {
            if let Some(lhs_term) = lhs_term_iter.peek() {
                if let Some(rhs_term) = rhs_term_iter.peek() {
                    match grevlex(lhs_term, rhs_term, &rhs.var_dict) {
                        Ordering::Equal => {
                            let new_coef = lhs_term.coef + rhs_term.coef;
                            if new_coef != 0 {
                                new_terms.push_back(Mono {
                                    coef: new_coef,
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

        Self {
            terms: new_terms,
            var_dict: self.var_dict,
        }
    }
}

impl ops::Sub<Poly> for Poly {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::constant(-1, &self.var_dict) * rhs + self
    }
}

impl ops::Mul<Poly> for Poly {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        assert!(Rc::ptr_eq(&self.var_dict, &rhs.var_dict));

        let mut new = Self::constant(0, &self.var_dict);

        for lhs_term in self.terms {
            for rhs_term in &rhs.terms {
                let new_term = Poly {
                    terms: VecDeque::from(vec![monomial_mul(&lhs_term, rhs_term, &self.var_dict)]),
                    var_dict: self.var_dict.clone(),
                };

                new = new + new_term;
            }
        }

        new
    }
}

impl Poly {
    pub fn compound_divide(&self, divisors: Vec<Poly>) -> (Vec<Poly>, Poly) {
        if divisors.is_empty() {
            return (vec![], self.clone());
        }

        let mut dividend = self.clone();

        let mut rem = Poly::constant(0, &self.var_dict);
        let mut quotients: Vec<_> = std::iter::repeat(Poly::constant(0, &self.var_dict))
            .take(divisors.len())
            .collect();

        let mut curr_divisor = 0;

        while dividend.get_constant_val() != Some(0) {
            let self_lt = dividend.terms[0].clone();
            if !divisors[curr_divisor].terms.is_empty() {
                let div_lt = &divisors[curr_divisor].terms[0];
                let self_over_div_lt = monomial_div(&self_lt, div_lt, &self.var_dict);

                if let Some(self_over_div_lt) = self_over_div_lt {
                    quotients[curr_divisor]
                        .terms
                        .push_back(self_over_div_lt.clone());

                    let self_over_div_lt = Poly {
                        terms: VecDeque::from(vec![self_over_div_lt]),
                        var_dict: self.var_dict.clone(),
                    };

                    dividend = dividend - (self_over_div_lt * divisors[curr_divisor].clone());
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
                    var_dict: self.var_dict.clone(),
                };

                dividend.terms.pop_front();

                rem = rem + self_lt;
                curr_divisor = 0;
            }
        }

        (quotients, rem)
    }

    pub fn try_divide(&self, divisor: &Poly) -> Option<Poly> {
        let (quots, rem) = self.compound_divide(vec![divisor.clone()]);

        if let Some(0) = rem.get_constant_val() {
            Some(quots[0].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use std::rc::Rc;

    use super::Poly;

    #[test]
    fn arith() {
        let var_dict = Rc::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        let a = Poly::var(0, 2, &var_dict) * Poly::constant(3, &var_dict);
        let b = Poly::var(1, 1, &var_dict) * Poly::constant(4, &var_dict);
        let c = Poly::constant(2, &var_dict);

        assert_eq!("3a^2 + 4b + 2", format!("{:?}", c + b + a));

        // (a + 1)(a + 1)
        let a = (Poly::var(0, 1, &var_dict) + Poly::constant(1, &var_dict))
            * (Poly::var(0, 1, &var_dict) + Poly::constant(1, &var_dict));
        // a^2 + 2a + 1
        let b = Poly::var(0, 2, &var_dict)
            + Poly::constant(2, &var_dict) * Poly::var(0, 1, &var_dict)
            + Poly::constant(1, &var_dict);

        assert!(a == b);
    }

    #[test]
    fn arith_fuzz() {
        let mut rng = SmallRng::seed_from_u64(1);

        let var_dict = Rc::new(vec!["x".to_string(), "y".to_string(), "z".to_string()]);

        fn create_random_poly(
            rng: &mut SmallRng,
            term_max: i32,
            var_dict: &Rc<Vec<String>>,
        ) -> Poly {
            let mut p = Poly::constant(0, var_dict);

            for _ in 0..rng.gen_range(0..term_max) {
                let coef = rng.gen_range(-6..6);
                let xpow = rng.gen_range(0..3);
                let ypow = rng.gen_range(0..1);
                let zpow = rng.gen_range(0..2);

                p = p + Poly::constant(coef, var_dict)
                    * Poly::var(0, xpow, var_dict)
                    * Poly::var(1, ypow, var_dict)
                    * Poly::var(2, zpow, var_dict);
            }

            p
        }

        for _ in 0..1000 {
            let dividend = create_random_poly(&mut rng, 2, &var_dict);
            let n_divs = rng.gen_range(0..4);
            let divisors: Vec<_> =
                std::iter::repeat_with(|| create_random_poly(&mut rng, 1, &var_dict))
                    .take(n_divs)
                    .collect();

            let (quotients, rem) = dividend.clone().compound_divide(divisors.clone());

            let calculated_dividend = quotients
                .clone()
                .into_iter()
                .zip(divisors.clone())
                .fold(Poly::constant(0, &var_dict), |acc, (x, y)| acc + x * y)
                + rem.clone();

            assert_eq!(
                calculated_dividend, dividend,
                "tried dividing {:?} by {:?}; got {:?} and rem {:?}",
                dividend, divisors, quotients, rem
            );
        }
    }
}
