use std::collections::HashSet;
use std::hash::Hash;

use crate::field::Field;
use crate::poly::Poly;

pub fn reducta_set<T: Field + Hash>(p: &Poly<T>, var: usize) -> HashSet<Vec<Poly<T>>> {
    let coefs = p.coefs(var);

    if coefs.is_empty() {
        return HashSet::new();
    }

    let mut first_nonzero = coefs.len() - 1;

    while coefs[first_nonzero].is_zero() {
        first_nonzero -= 1;
    }

    (0..=first_nonzero)
        .map(|i| coefs[i..=first_nonzero].to_vec())
        .collect()
}

pub fn projection<T: Field + Hash>(ps: Vec<Poly<T>>, var_order: &[usize]) -> Vec<HashSet<Vec<Poly<T>>>> {
    let mut projs = vec![HashSet::new()];
    for poly in ps {
        projs[0].insert(poly.coefs(var_order[0]));
    }

    // for var in var_order.iter().skip(1) {
    //     let mut next = HashSet::new();

    //     for p in &projs[projs.len() - 1] {
    //         // for red in reducta_set(p, var) {

    //         // }
    //     }

    //     projs.push(next);
    // }

    projs
}

#[cfg(test)]
mod tests {
    use super::reducta_set;
    use crate::system;

    #[test]
    fn red() {
        let sys = system! {
            3*x^2*y*z + 2*x^2*z^2 - 4*x + 7*x*y + y^2 - z,
            3*y*z + 2*z^2,
            7*y - 4,
            y^2 - z,
        };

        let expected = vec![
            vec![sys.members[1].clone(), sys.members[2].clone(), sys.members[3].clone()],
            vec![sys.members[2].clone(), sys.members[3].clone()],
            vec![sys.members[3].clone()]
        ];

        let rs = reducta_set(&sys.members[0], 0);

        assert_eq!(rs.len(), expected.len());

        for eps in &expected {
            assert!(rs.contains(eps));
        }
    }
}
