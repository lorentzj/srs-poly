pub mod macros;
pub mod mono;
pub mod poly_arithmetic;
pub mod system;

use std::collections::VecDeque;
use std::fmt::Write;

use crate::poly::mono::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly {
    pub terms: VecDeque<Mono>,
}

impl Poly {
    pub fn var(var: usize, pow: u64) -> Self {
        if pow == 0 {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![],
                }]),
            }
        } else {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![(var, pow)],
                }]),
            }
        }
    }

    pub fn constant(val: i64) -> Self {
        Self {
            terms: if val == 0 {
                VecDeque::new()
            } else {
                VecDeque::from(vec![Mono {
                    num: val,
                    den: 1,
                    vars: vec![],
                }])
            },
        }
    }

    pub fn get_constant_val(&self) -> Option<(i64, i64)> {
        if self.terms.is_empty() {
            Some((0, 1))
        } else if self.terms.len() == 1 {
            if self.terms[0].vars.is_empty() {
                Some((self.terms[0].num, self.terms[0].den))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn lt(&self) -> Poly {
        match self.terms.front() {
            Some(m) => Poly {
                terms: VecDeque::from(vec![m.clone()]),
            },
            None => Poly {
                terms: VecDeque::from(vec![]),
            },
        }
    }

    pub fn lt_mono(&self) -> Mono {
        match self.terms.front() {
            Some(m) => m.clone(),
            None => Mono {
                num: 0,
                den: 1,
                vars: vec![],
            },
        }
    }

    pub fn s_poly(p: Poly, q: Poly) -> Poly {
        let p_lt = p.lt();
        let q_lt = q.lt();

        let lcm_lmp_lmq = Poly {
            terms: VecDeque::from(vec![monomial_lcm(p_lt.lt_mono(), q_lt.lt_mono())]),
        };

        if let (Some(coef_p), Some(coef_q)) =
            (lcm_lmp_lmq.try_divide(&p_lt), lcm_lmp_lmq.try_divide(&q_lt))
        {
            coef_p * p - coef_q * q
        } else {
            unreachable!()
        }
    }

    pub fn norm(&self) -> Poly {
        let mut new = self.clone();

        let mut all_terms_den = 1;
        let mut all_terms_gcd = 1;

        if let Some(t) = new.terms.front() {
            all_terms_gcd = t.num;
        }

        for term in &new.terms {
            all_terms_den *= term.den / gcd(all_terms_den, term.den);
        }

        for term in &mut new.terms {
            let term_gcd = gcd(term.num * all_terms_den, term.den);
            term.num = term.num * all_terms_den / term_gcd;
            term.den = 1;

            all_terms_gcd = gcd(term.num, all_terms_gcd);
        }

        if let Some(t) = new.terms.front_mut() {
            if t.num < 0 {
                all_terms_gcd = -all_terms_gcd.abs();
            } else {
                all_terms_gcd = all_terms_gcd.abs();
            }
        }

        for term in &mut new.terms {
            term.num /= all_terms_gcd;
        }

        new
    }

    pub fn deg(&self, var: usize) -> usize {
        self.terms
            .iter()
            .map(|term| term.deg(var))
            .fold(0, |acc, v| acc.max(v))
    }

    pub fn coefs(&self, var: usize) -> Vec<Poly> {
        let deg = self.deg(var);
        let mut coefs: Vec<_> = std::iter::repeat(Poly::constant(0)).take(deg + 1).collect();

        for term in &self.terms {
            let (term_deg, term_coef) = term.coef(var);

            coefs[deg - term_deg] = coefs[deg - term_deg].clone()
                + Poly {
                    terms: VecDeque::from(vec![term_coef]),
                };
        }

        coefs
    }
}

impl Poly {
    pub fn format(&self, var_dict: &[String]) -> String {
        let mut s = String::new();
        if self.terms.is_empty() {
            write!(s, "0").unwrap();
        }

        for (i, Mono { num, den, vars }) in (self.terms).iter().enumerate() {
            let coef = (*num as f64) / (*den as f64);
            if coef != 1. || vars.is_empty() {
                if coef < 0. {
                    if coef == -1. && !vars.is_empty() {
                        if i == 0 {
                            write!(s, "-").unwrap();
                        } else {
                            write!(s, " - ").unwrap();
                        }
                    } else if i == 0 {
                        write!(s, "{coef}").unwrap();
                    } else {
                        write!(s, " - {}", -coef).unwrap();
                    }
                } else if i == 0 {
                    write!(s, "{coef}").unwrap();
                } else {
                    write!(s, " + {coef}").unwrap();
                }
            } else if i != 0 {
                write!(s, " + ").unwrap();
            }

            for (var, pow) in vars {
                if *pow == 1 {
                    write!(s, "{}", var_dict[*var]).unwrap();
                } else {
                    write!(s, "{}^{pow}", var_dict[*var]).unwrap();
                }
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Poly;

    #[test]
    fn coefs() {
        let var_dict = vec!["x".to_string(), "y".to_string(), "z".to_string()];

        let a = Poly::var(0, 4);
        let b = Poly::var(0, 2) * Poly::constant(3);
        let c = Poly::var(0, 2) * Poly::var(2, 3) * Poly::constant(5);
        let d = Poly::var(1, 1) * Poly::var(0, 1) * Poly::constant(4);
        let e = Poly::var(2, 1);
        let f = Poly::constant(2);

        let g = a + b + c + d + e + f;

        assert_eq!(
            "5x^2z^3 + x^4 + 3x^2 + 4xy + z + 2",
            format!("{}", g.format(&var_dict))
        );

        assert_eq!(
            "[\"1\", \"0\", \"5z^3 + 3\", \"4y\", \"z + 2\"]",
            format!(
                "{:?}",
                g.coefs(0)
                    .iter()
                    .map(|p| p.format(&var_dict))
                    .collect::<Vec<_>>()
            )
        );
    }
}
