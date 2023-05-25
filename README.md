# srs_poly

Some utilities for integer polynomial systems.

```rust
let sys = srs_poly::system!(
    a^2 - b^2,
    a + b,
    2*a + 3*b^2 + c + 10
);

println!("{:?}", sys.members);
println!("{:?}", sys.members[0].try_divide(&sys.members[1]));
println!("{:?}", sys.members[0].clone() * sys.constant(5) - sys.members[1].clone() + sys.var("a", 2));
```

```
[a^2 - b^2, a + b, 2a + 3b^2 + c + 10]
Some(a - b)
6a^2 - 5b^2 - a - b
```