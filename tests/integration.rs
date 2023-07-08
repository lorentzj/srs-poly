extern crate srs_solver;
use srs_solver::{system, univariate};

#[test]
fn roots() {
    let tol = 0.00000001;
    let poly = univariate!(x ^ 4 - 3 * x ^ 3 - 21 * x ^ 2 + 43 * x + 60);
    let roots = poly.real_roots(tol);

    assert!((roots[0] + 4.).abs() < tol);
    assert!((roots[1] + 1.).abs() < tol);
    assert!((roots[2] - 3.).abs() < tol);
    assert!((roots[3] - 5.).abs() < tol);
}

#[test]
fn gb() {
    let sys = system! {
        x^2*y + 1,
        2*x + y*z - 1,
        x - y^2*z^2 + 1
    };

    assert_eq!("[4x - 5, 25y + 16, 32z - 75]", format!("{:?}", sys.gb()));
}
