use crate::poly::poly::Poly;

#[derive(Debug, Clone)]
pub enum Cmp {
    Gt,
    Eq,
    Lt
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub value: Poly,
    pub cmp_zero: Cmp,
    pub provenance: Vec<usize>
}

#[derive(Debug, Clone)]
pub enum Tarski {
    And(Box<Tarski>, Box<Tarski>),
    Or(Box<Tarski>, Box<Tarski>),
    Not(Box<Tarski>),
    C(Constraint)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}