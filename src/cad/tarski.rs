use crate::poly::Poly;
use crate::rational::Rat;

#[derive(Debug, Clone)]
pub enum Cmp {
    Gt,
    Eq,
    Lt,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub value: Poly<Rat>,
    pub cmp_zero: Cmp
}

#[derive(Debug, Clone)]
pub enum T {
    And(Box<T>, Box<T>),
    Or(Box<T>, Box<T>),
    Not(Box<T>),
    C(Constraint),
}

#[derive(Debug, Clone)]
pub struct Tarski {
    pub var_dict: Vec<String>,
    pub exists: Vec<usize>,
    pub forall: Vec<usize>,
    pub data: T,
}
