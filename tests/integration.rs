extern crate srs_poly;
use srs_poly::system;

#[test]
fn gb() {
    let sys = system! {
        3*x + y^2 + 2*z^3,
        x - y + 3*z + 5,
        2*x - 2*y + 3
    };

    println!("{:?}", sys.gb().members);
}
