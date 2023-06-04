use std::collections::VecDeque;

use crate::poly::Poly;

// Bareiss algorithm
fn determinant(mut mat: Vec<Vec<Poly>>, size: usize) -> Poly {
    if size == 0 {
        Poly::constant(1)
    } else if size == 1 {
        mat.pop().unwrap().pop().unwrap()
    } else if size == 2 {
        mat[0][0].mul_ref(&mat[1][1]) - mat[0][1].mul_ref(&mat[1][0])
    } else if size == 3 {
        mat[0][0].mul_ref(&mat[1][1]).mul_ref(&mat[2][2]) + 
        mat[0][1].mul_ref(&mat[1][2]).mul_ref(&mat[2][0]) + 
        mat[0][2].mul_ref(&mat[1][0]).mul_ref(&mat[2][1]) - 
        mat[0][2].mul_ref(&mat[1][1]).mul_ref(&mat[2][0]) - 
        mat[0][0].mul_ref(&mat[1][2]).mul_ref(&mat[2][1]) - 
        mat[0][1].mul_ref(&mat[1][0]).mul_ref(&mat[2][2])
    } else {
        for i in 0..size {
            for j in 0..size {
                if i != j {
                    for k in (i + 1)..size {
                        mat[j][k] = mat[i][i].mul_ref(&mat[j][k]) - mat[j][i].mul_ref(&mat[i][k]);

                        if i != 0 {
                            mat[j][k] = mat[j][k].try_divide(&mat[i - 1][i - 1]).unwrap();
                        }
                    }
                }
            }
        }

        mat.pop().unwrap().pop().unwrap()
    }
}

// k'th order Sylvester matrix
// see https://link.springer.com/article/10.1007/s00200-004-0158-4
fn syl_k(a_coefs: &Vec<Poly>, b_coefs: &Vec<Poly>, k: usize) -> Vec<Vec<Poly>> {
    let mut rows = vec![];
    let a_deg = a_coefs.len() - 1;
    let b_deg = b_coefs.len() - 1;

    for i in 0..(b_deg - k) {
        let mut row = vec![];
        for j in 0..(a_deg + b_deg - k) {
            if i > j || j > i + a_deg {
                row.push(Poly::constant(0))
            } else {
                row.push(a_coefs[j - i].clone())
            }
        }
        rows.push(row);
    }

    for i in 0..(a_deg - k) {
        let mut row = vec![];
        for j in 0..(a_deg + b_deg - k) {
            if i > j || j > i + b_deg {
                row.push(Poly::constant(0))
            } else {
                row.push(b_coefs[j - i].clone())
            }
        }
        rows.push(row);
    }

    rows
}

// each subresultant is densely represented as poly coefs (univariate in var)
// deg(b) <= deg(a)
pub fn subresultants(a: &Poly, b: &Poly, var: usize) -> Vec<Vec<Poly>> {
    let mut srs = vec![a.coefs(var), b.coefs(var)];
    let (_n, m) = (srs[0].len() - 1, srs[1].len() - 1);
    
    for k in (0 .. m).rev() {
        let mut syl = syl_k(&srs[0], &srs[1],  k);

        let syl_m = syl.len();
        let syl_n = syl[0].len();

        let mut coefs = VecDeque::new();

        for _ in 0 .. syl_n + 1 - syl_m  {
            let mut syl_minor = vec![];
            for row in &mut syl {
                let mut syl_minor_row = vec![];
                syl_minor_row.extend_from_slice(&row[0..syl_m - 1]);
                syl_minor_row.push(row.pop().unwrap());

                syl_minor.push(syl_minor_row);
            }

            coefs.push_front(determinant(syl_minor, syl_m));
        }

        srs.push(Vec::from(coefs));
    }

    srs
}

#[cfg(test)]
mod tests {
    use crate::system;
    use super::{subresultants, determinant, syl_k};

    #[test]
    fn sylvester() {
        let a_coefs = system!{ 5, 4, 3, 2, 1 }.members;
        let b_coefs = system!{ 4, 3, 2, 1 }.members;

        let expected_deg0 = vec![
            "5, 4, 3, 2, 1, 0, 0",
            "0, 5, 4, 3, 2, 1, 0",
            "0, 0, 5, 4, 3, 2, 1",
            "4, 3, 2, 1, 0, 0, 0",
            "0, 4, 3, 2, 1, 0, 0",
            "0, 0, 4, 3, 2, 1, 0",
            "0, 0, 0, 4, 3, 2, 1"
        ];

        let expected_deg1 = vec![
            "5, 4, 3, 2, 1, 0",
            "0, 5, 4, 3, 2, 1",
            "4, 3, 2, 1, 0, 0",
            "0, 4, 3, 2, 1, 0",
            "0, 0, 4, 3, 2, 1",
        ];

        let mat = syl_k(&a_coefs, &b_coefs, 0);

        for i in 0..(a_coefs.len() + b_coefs.len() - 2) {
            let line = mat[i].iter().map(|p| p.format(&vec![])).collect::<Vec<_>>().join(", ");
            assert_eq!(expected_deg0[i], line);
        }

        let mat = syl_k(&a_coefs, &b_coefs, 1);

        for i in 0..(a_coefs.len() + b_coefs.len() - 4) {
            let line = mat[i].iter().map(|p| p.format(&vec![])).collect::<Vec<_>>().join(", ");
            assert_eq!(expected_deg1[i], line);
        }
    }

    #[test]
    fn const_det() {
        let mat = vec![
            system!{  1,  2,  3,  4 }.members,
            system!{  5,  6,  7, -8 }.members,
            system!{  0,  9,  0,  1 }.members, 
            system!{ -2, -5, 11,  1 }.members   
        ];

        assert_eq!(-3560, determinant(mat, 4).get_constant_val().unwrap().0);
    }

    #[test]
    fn srs() {
        let sys = system! {
            2*x^4 - 2*x^2*y + 3*x*y + 1,
            x^3 + 2*x^2*y - x*y^2 + 3*y
        };

        let expected = vec![
            "2, 0, -2y, 3y, 1",
            "1, 2y, -y^2, 3y",
            "10y^2 - 2y, -4y^3 - 3y, 12y^2 + 1",
            "-4y^6 + 24y^5 - 40y^4 + 12y^3 - y^2 + 2y, 12y^5 - 72y^4 - 48y^3 + 4y^2 - 3y",
            "40y^8 - 312y^7 + 568y^6 + 744y^5 + 398y^4 + 42y^3 + 42y^2 + 1",
        ];

        let subres = subresultants(&sys.members[0], &sys.members[1], 0);

        assert_eq!(subres.len(), expected.len());

        for (ps, eps) in subres.iter().zip(expected) {
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