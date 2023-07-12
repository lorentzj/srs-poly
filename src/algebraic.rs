use crate::univariate::{UPoly, Root};
use crate::field::Field;

#[derive(Debug, Clone)]
pub struct Algebraic<T: Field> {
    pub p: UPoly<T>,
    pub val: Root<T>,
    pub n: usize
}

pub fn get_roots<T: Field>(p: UPoly<T>, tolerance: T) -> Vec<Algebraic<T>> {
    p.real_root_intervals(tolerance).into_iter().enumerate().map(|(i, root)| {
        Algebraic { p: p.clone(), val: root, n: i }
    }).collect()
}

impl<T: Field> Algebraic<T> {
    pub fn from_point(point: T) -> Self {
        Algebraic { p: 
            UPoly(vec![T::from(1), point.clone() * -1]),
            val: Root::Point(point),
            n: 0
        }
    }   
}