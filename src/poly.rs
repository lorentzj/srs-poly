use serde::{Serialize, Serializer};
use std::fmt;
use std::{collections::VecDeque, rc::Rc};

use crate::mono::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Poly {
    pub terms: VecDeque<Mono>,
    pub var_dict: Rc<Vec<String>>,
}

impl Poly {
    pub fn var(var: usize, pow: u64, var_dict: &Rc<Vec<String>>) -> Self {
        if pow == 0 {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![],
                }]),
                var_dict: var_dict.clone(),
            }
        } else {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![(var, pow)],
                }]),
                var_dict: var_dict.clone(),
            }
        }
    }

    pub fn constant(val: i64, var_dict: &Rc<Vec<String>>) -> Self {
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
            var_dict: var_dict.clone(),
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

    pub fn lt(&self) -> Poly {
        match self.terms.front() {
            Some(m) => Poly {
                terms: VecDeque::from(vec![m.clone()]),
                var_dict: self.var_dict.clone(),
            },
            None => Poly {
                terms: VecDeque::from(vec![]),
                var_dict: self.var_dict.clone(),
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
            terms: VecDeque::from(vec![monomial_lcm(
                p_lt.lt_mono(),
                q_lt.lt_mono(),
                &p.var_dict,
            )]),
            var_dict: p.var_dict.clone(),
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
}

impl fmt::Debug for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            write!(f, "0")?
        }

        for (i, Mono { num, den, vars }) in (self.terms).iter().enumerate() {
            let coef = (*num as f64) / (*den as f64);
            if coef != 1. || vars.is_empty() {
                if coef < 0. {
                    if coef == -1. && !vars.is_empty() {
                        if i == 0 {
                            write!(f, "-")?;
                        } else {
                            write!(f, " - ")?;
                        }
                    } else if i == 0 {
                        write!(f, "{coef}")?;
                    } else {
                        write!(f, " - {}", -coef)?;
                    }
                } else if i == 0 {
                    write!(f, "{coef}")?;
                } else {
                    write!(f, " + {coef}")?;
                }
            } else if i != 0 {
                write!(f, " + ")?;
            }

            for (var, pow) in vars {
                if *pow == 1 {
                    write!(f, "{}", self.var_dict[*var])?;
                } else {
                    write!(f, "{}^{pow}", self.var_dict[*var])?;
                }
            }
        }

        Ok(())
    }
}

impl Serialize for Poly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("{self:?}"))
    }
}
