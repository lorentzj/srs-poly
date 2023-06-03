extern crate srs_solver;
use srs_solver::system;

#[test]
fn gb() {
    let sys = system! {
        x^2*y + 1,
        2*x + y*z - 1,
        x - y^2*z^2 + 1
    };

    assert_eq!(
        "[4x - 5, 25y + 16, 32z - 75]",
        format!("{:?}", sys.gb())
    );
}
