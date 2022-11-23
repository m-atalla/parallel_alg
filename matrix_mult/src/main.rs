use std::{ops::Mul, sync::mpsc, fmt::Display};
use util::{self, time_eval, Instant};
use threads::ThreadPool;
const M: usize = 10;
const N: usize = 10;


#[derive(Debug, Clone)]
struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<i32>>
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = String::from("\n");
        for row in &self.data {
            for cell in row {
                fmt.push_str(format!("{cell}\t").as_str());
            }
            fmt.push('\n');
        }
        write!(f, "{}", fmt)
    }
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

    // A single row matrix instance
    pub fn from_vec(vec: Vec<i32>) -> Matrix {
        Matrix {
            rows: 1,
            cols: vec.len(),
            data: vec![vec]
        }
    }

    pub fn collect(accord: Vec<Matrix>) -> Matrix {
        let mut rows = 0;
        let cols = accord.first().unwrap().cols;
        let mut data = vec![];

        accord.iter().all(move |m| m.cols == cols);

        for mtx in accord {
            for row in mtx {
                data.push(row);
                rows += 1;
            }
        }

        Matrix { 
            rows, 
            cols, 
            data
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
        assert!(self.cols == rhs.rows, "Expect dimensions to match");
        let mut new_mat = Matrix::new(self.rows, rhs.cols, 0);

        for i in 0..self.rows {
            for j in 0..self.cols {
                for k in 0..rhs.rows {
                    new_mat.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        new_mat
    }
}

fn mat_mul_par(a: Matrix, b: Matrix) -> Matrix {
    let pool = ThreadPool::new(a.rows);

    let (tx, rx) = mpsc::channel();

    let rows = a.rows;

    for row in a {
        let thread_b = b.clone();
        let vec_mat = Matrix::from_vec(row);
        let thread_res = tx.clone();
        pool.execute(move || {
            let result = vec_mat * thread_b;
            thread_res.send(result).unwrap();
        });
    }

    let mut mat_accord = Vec::with_capacity(rows);
    for _ in 0..rows {
        let res = rx.recv().unwrap();
        mat_accord.push(res);
    }
    Matrix::collect(mat_accord)
}


fn main() {
    let a = Matrix::new(5, 5, 10);
    let b = Matrix::new(5, 5, 20);

    time_eval!("Seq mult", {
        let out = a.clone() * b.clone();
        println!("{out}");
    });

    time_eval!("Par mult", {
        let out = mat_mul_par(a, b);
        println!("{out}");
    });
}
