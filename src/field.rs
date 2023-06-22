use std::{hash::Hash, ops, fmt::Debug};

pub trait Field = Clone
    + Hash
    + Debug
    + ToString
    + Eq
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Mul<i64, Output = Self>
    + ops::Div<Output = Self>
    + Zero
    + One;

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}
