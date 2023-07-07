use std::cmp::Ordering;

// thanks to Osvaldo Carvalho
// https://www.researchgate.net/publication/320864673_A_simple_recursive_algorithm_to_find_all_real_roots_of_a_polynomial
use crate::field::Field;

pub struct UPoly<T: Field>(Vec<T>);

#[derive(Debug, PartialEq, Eq)]
pub enum Root<T: Field> {
    Point(T),
    Interval(T, T),
}

impl<T: Field> Root<T> {
    pub fn approx(&self) -> T {
        match self {
            Root::Point(p) => p.clone(),
            Root::Interval(start, end) => (start.clone() + end.clone()) / T::from(2),
        }
    }
}

impl<T: Field> UPoly<T> {
    // Horner's method
    pub fn eval(&self, x: &T) -> T {
        self.0
            .iter()
            .fold(T::zero(), |acc, next| acc * x.clone() + next.clone())
    }

    pub fn derivative(&self) -> Self {
        let mut new = self.0.clone();
        new.pop();
        let deg = new.len() - 1;

        for (i, coef) in new.iter_mut().enumerate() {
            *coef = coef.clone() * ((deg + 1 - i) as i64);
        }

        Self(new)
    }

    pub fn real_root_intervals(&self, tolerance: T) -> Vec<Root<T>> {
        match self.0.len() {
            0 | 1 => vec![],
            2 => {
                if self.0[0] == T::zero() {
                    vec![]
                } else {
                    vec![Root::Point(self.0[1].clone() * -1 / self.0[0].clone())]
                }
            }
            _ => {
                let derivative_roots = self.derivative().real_root_intervals(tolerance.clone());

                if derivative_roots.is_empty() {
                    vec![]
                } else {
                    let mut new_roots = vec![];

                    let first_derivative_root = derivative_roots[0].approx();

                    match self.eval(&first_derivative_root).cmp(&T::zero()) {
                        Ordering::Less => {
                            if self.0[0] < T::zero() {
                                // value here is same sign as -inf; no root
                            } else {
                                // probe backwards until we have a finite interval
                                let mut lhs = T::from(-1);
                                while self.eval(&(first_derivative_root.clone() + lhs.clone()))
                                    < T::zero()
                                {
                                    lhs = lhs.clone() * T::from(2);
                                }
                                new_roots.push(self.refine_root_interval(
                                    first_derivative_root.clone() + lhs,
                                    first_derivative_root,
                                    tolerance,
                                ))
                            }
                        }
                        Ordering::Greater => {}
                        Ordering::Equal => {}
                    }

                    new_roots
                }
            }
        }
    }

    pub fn refine_root_interval(&self, mut start: T, mut end: T, tolerance: T) -> Root<T> {
        let start_sign = self.eval(&start) > T::zero();

        while end.clone() - start.clone() > tolerance {
            let mid = (start.clone() + end.clone()) / T::from(2);
            if (self.eval(&mid) > T::zero()) == start_sign {
                start = mid;
            } else {
                end = mid;
            }
        }

        Root::Interval(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::{Root, UPoly};
    use crate::rational::Rat;

    #[test]
    fn eval() {
        let p = UPoly(vec![
            Rat::from(2),
            Rat::from(3),
            Rat::from(-4),
            Rat::from(1),
        ]);
        assert_eq!(p.eval(&Rat::from(8)), Rat::from(1185));
    }

    #[test]
    fn derivative() {
        let p = UPoly(vec![
            Rat::from(1),
            Rat::from(5),
            Rat::from(0),
            Rat::from(2),
            Rat::from(3),
        ]);
        let dp = p
            .derivative()
            .0
            .into_iter()
            .map(|x| i64::try_from(x).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(dp, vec![4, 15, 0, 2]);
    }

    #[test]
    fn refine_root_interval() {
        let quadratic = UPoly(vec![Rat::from(1), Rat::from(0), Rat::from(-2)]);
        let tol = Rat::from(1) / Rat::from(10000);
        let refined = quadratic
            .refine_root_interval(Rat::from(0), Rat::from(2), tol)
            .approx();
        let approx_zero = (2. - f64::from(refined) * f64::from(refined)).abs();

        assert!(approx_zero < f64::from(tol));
    }

    #[test]
    fn lin_root() {
        let linear = UPoly(vec![Rat::from(3), Rat::from(-2)]);
        assert_eq!(
            linear.real_root_intervals(Rat::from(1) / Rat::from(10000)),
            vec![Root::Point(Rat::from(2) / Rat::from(3))]
        );
    }
}
