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
}
