use crate::field::Field;
use crate::poly::Poly;

pub fn reducta_set<T: Field>(p: &Poly<T>, var: usize) -> Vec<Vec<Poly<T>>> {
    let coefs = p.coefs(var);

    if coefs.is_empty() {
        return vec![];
    }

    let mut first_nonzero = coefs.len() - 1;

    while coefs[first_nonzero].is_zero() {
        first_nonzero -= 1;
    }

    (0..=first_nonzero)
        .map(|i| coefs[i..=first_nonzero].to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::reducta_set;
    use crate::system;

    #[test]
    fn red() {
        let sys = system! {
            3*x^2*y*z + 2*x^2*z^2 - 4*x + 7*x*y + y^2 - z
        };

        let expected = vec!["3yz + 2z^2, 7y - 4, y^2 - z", "7y - 4, y^2 - z", "y^2 - z"];

        let rs = reducta_set(&sys.members[0], 0);

        assert_eq!(rs.len(), expected.len());

        for (ps, eps) in rs.iter().zip(expected) {
            assert_eq!(
                eps,
                ps.iter()
                    .map(|p| p.format(&sys.var_dict))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
}
