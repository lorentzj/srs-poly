# srs_poly

Some utilities for integer polynomial systems.

```rust
use srs_poly::system;

#[test]
fn gb() {
    let sys = system! {
        3*x + y^2 + 2*z^3,
        x - y + 3*z + 5,
        2*x - 2*y + 3
    };

    // groebner basis
    assert_eq!(
        "[108y^2 + 324y - 829, 2x - 2y + 3, 6z + 7]",
        format!("{:?}", sys.gb().members)
    );
}
```