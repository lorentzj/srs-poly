# srs_poly

Some utilities for integer polynomial systems.

```rust
let sys = srs_poly::system!(
    a^2 - b^2*c^2,
    a + b*c,
    2*a + 3*b^2 + c + 10    
);

println!("{:?}", sys.members);
println!("{:?}", sys.get(0).try_divide(&sys.get(1)));
println!(
    "{:?}",
    sys.get(0) * sys.constant(5) - sys.get(1) + sys.var("a", 2)
);
```

```
[-b^2c^2 + a^2, bc + a, 3b^2 + 2a + c + 10]
Some(-bc + a)
-5b^2c^2 + 6a^2 - bc - a
```