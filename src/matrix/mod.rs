use std::ops::{Add, Mul};
use std::cmp::Ordering;

pub mod determinant;

#[derive(Clone, Debug)]
pub struct Matrix<T: Add + Mul + Clone + PartialEq + Default> {
    pub n_rows: usize,
    pub n_cols: usize,
    vals: Vec<(usize, Vec<(usize, T)>)>
}

impl<T: Add + Mul + Clone + PartialEq + Default> Matrix<T> {
    pub fn from_dense(v: Vec<Vec<T>>) -> Matrix<T> {
        let n_rows =  v.len();
        let n_cols  = v.first().and_then(|row| Some(row.len())).unwrap_or(0);

        let mut sparse_v = vec![];
        let default = T::default();
        for (i, row) in v.into_iter().enumerate() {
            let mut sparse_row = vec![];
            for (j, item) in row.into_iter().enumerate() {
                if item != default {
                    sparse_row.push((j, item));
                }
            }

            if !sparse_row.is_empty() {
                sparse_v.push((i, sparse_row));
            }
        }

        Matrix {
            n_rows,
            n_cols,
            vals: sparse_v
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let mut row_i_max = self.vals.len();
        let mut row_i_min = 0;
        let mut row_i     = (row_i_max + row_i_min) / 2;

        if row_i_max == row_i {
            return T::default();
        }

        loop {
            match self.vals[row_i].0.cmp(&row) {
                Ordering::Equal => {
                    let mut col_i_max = self.vals[row_i].1.len();
                    let mut col_i_min = 0;
                    let mut col_i     = (col_i_max + col_i_min) / 2;
            
                    if col_i_max == col_i {
                        return T::default();
                    }

                    loop {
                        match self.vals[row_i].1[col_i].0.cmp(&col) {
                            Ordering::Equal => return self.vals[row_i].1[col_i].1.clone(),
                            Ordering::Greater => {
                                if col_i_max == col_i {
                                    return T::default();
                                }
                                col_i_max = col_i;
                                col_i     = (col_i_max + col_i_min) / 2;
                            },
                            Ordering::Less => {
                                if col_i_min == col_i {
                                    return T::default();
                                }
                                col_i_min = col_i;
                                col_i     = (col_i_max + col_i_min) / 2;
                            }
                        }
                    }
                },
                Ordering::Greater => {
                    if row_i_max == row_i {
                        return T::default();
                    }
                    row_i_max = row_i;
                    row_i = (row_i_max + row_i_min) / 2;
                },
                Ordering::Less => {
                    if row_i_min == row_i {
                        return T::default();
                    }
                    row_i_min = row_i;
                    row_i = (row_i_max + row_i_min) / 2;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn get_and_set() {
        let items = vec![
            vec![1, 2, 0, 3, 0, 0, 4],
            vec![0, 0, 0, 0, 2, 2, 2],
            vec![0, 1, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![4, 1, 2, 0, 1, 0, 1]
        ];

        let mat = Matrix::from_dense(items);

        assert_eq!(1, mat.get(0, 0));
        assert_eq!(2, mat.get(0, 1));
        assert_eq!(0, mat.get(0, 2));
        assert_eq!(3, mat.get(0, 3));
        assert_eq!(0, mat.get(0, 4));
        assert_eq!(0, mat.get(0, 5));
        assert_eq!(4, mat.get(0, 6));

        assert_eq!(0, mat.get(1, 0));
        assert_eq!(0, mat.get(1, 1));
        assert_eq!(0, mat.get(1, 2));
        assert_eq!(0, mat.get(1, 3));
        assert_eq!(2, mat.get(1, 4));
        assert_eq!(2, mat.get(1, 5));
        assert_eq!(2, mat.get(1, 6));

        assert_eq!(0, mat.get(2, 0));
        assert_eq!(1, mat.get(2, 1));
        assert_eq!(0, mat.get(2, 2));
        assert_eq!(0, mat.get(2, 3));
        assert_eq!(1, mat.get(2, 4));
        assert_eq!(0, mat.get(2, 5));
        assert_eq!(1, mat.get(2, 6));

        assert_eq!(0, mat.get(3, 0));
        assert_eq!(0, mat.get(3, 1));
        assert_eq!(0, mat.get(3, 2));
        assert_eq!(0, mat.get(3, 3));
        assert_eq!(0, mat.get(3, 4));
        assert_eq!(0, mat.get(3, 5));
        assert_eq!(0, mat.get(3, 6));

        assert_eq!(4, mat.get(4, 0));
        assert_eq!(1, mat.get(4, 1));
        assert_eq!(2, mat.get(4, 2));
        assert_eq!(0, mat.get(4, 3));
        assert_eq!(1, mat.get(4, 4));
        assert_eq!(0, mat.get(4, 5));
        assert_eq!(1, mat.get(4, 6));
    }
}