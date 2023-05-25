use crate::poly::Poly;
use std::rc::Rc;

pub struct System {
    pub var_dict: Rc<Vec<String>>,
    pub members: Vec<Poly>,
}

impl System {
    pub fn constant(&self, val: i64) -> Poly {
        Poly::constant(val, &self.var_dict)
    }

    pub fn var(&self, var: &str, pow: u64) -> Poly {
        match self.var_dict.iter().position(|v| v == var) {
            Some(i) => Poly::var(i, pow, &self.var_dict),
            None => panic!("variable {} not in system variable dict", var),
        }
    }
}
