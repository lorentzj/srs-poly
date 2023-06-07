use crate::field::Field;

pub struct Algebraic<T: Field> {
    pub poly: Vec<T>,
    pub interval: (T, T)
}

pub fn isolate_roots<T: Field>(_coefs: Vec<T>) -> Vec<Algebraic<T>> {
    vec![]
}

fn derivative<T: Field>(mut coefs: Vec<T>) -> Vec<T> {
    coefs.pop();
    let deg = coefs.len() - 1;

    for (i, coef) in coefs.iter_mut().enumerate() {
        *coef = coef.clone() * ((deg + 1 - i) as i64);
    }

    coefs
}

// Yun's algorithm
pub fn square_free<T: Field>(_coefs: Vec<T>) -> Vec<T> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::derivative;
    use crate::rational::Rat;

    #[test]
    fn univariate_derivative() {
        let p = vec![Rat::from(1), Rat::from(5), Rat::from(0), Rat::from(2), Rat::from(3)];
        let dp = derivative(p).into_iter().map(|x| i64::try_from(x).unwrap()).collect::<Vec<_>>();

        assert_eq!(dp, vec![4, 15, 0, 2]);
    }
}