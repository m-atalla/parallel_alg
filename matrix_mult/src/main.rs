use std::{ops::{Mul, AddAssign}, process::Output, result, vec};
const M: usize = 10;
const N: usize = 10;

struct Matrix {
    rows: usize,
    cols: usize,
    pub data: Vec<Vec<i32>>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, init_val: i32) -> Matrix {
        let mat = vec![vec![init_val; cols]; rows];
        Matrix { 
            rows, 
            cols,
            data: mat 
        }
    }
}

impl IntoIterator for Matrix {
    type Item = Vec<i32>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_mat = Matrix::new(self.rows, rhs.cols, 0);
        for i in 0..self.rows {
            for j in 0..self.cols {
                for k in 0..self.rows {
                    new_mat.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        new_mat
    }
}


fn main() {
    let a = Matrix::new(5, 5, 10);
    let b = Matrix::new(5, 5, 20);
    let out = a * b;

    for row in out {
        println!("{row:?}");
    }
}
