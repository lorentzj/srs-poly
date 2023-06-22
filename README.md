The constraint solver for the Sirius type checker. Implements cylindrical algebraic decomposition (CAD) for quantifier elimination and other algebraic geometry utilities.

WIP

Groebner basis example:

```rust
use srs_poly::system;

#[test]
fn gb() {
    let sys = system! {
        x^2*y + 1,
        2*x + y*z - 1,
        x - y^2*z^2 + 1
    };

    assert_eq!("[4x - 5, 25y + 16, 32z - 75]", format!("{:?}", sys.gb()));
}
```