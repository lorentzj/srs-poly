use std::{ops, hash::Hash};
use std::fmt::Debug;
use crate::rational::Rat;

pub trait Field = 
    Clone
     + Debug
     + From<i64>
     + TryInto<i64>
     + Into<f64>
     + Into<Rat>
     + PartialEq
     + Eq
     + ops::Add<Output = Self>
     + ops::Sub<Output = Self>
     + ops::Mul<Output = Self>
     + ops::Mul<i64, Output = Self>
     + ops::Div<Output = Self>
     + Zero
     + One
     + Hash;

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}