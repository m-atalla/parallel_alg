use std::{env, sync::{Arc, mpsc}};
use threads::{self, ThreadPool};
use util;

const C: f64 = 0.5;
const N_COLS: usize = 10;
const N_ROWS: usize = 10;

type Row = Vec<f64>;
type Matrix = Vec<Row>;

#[cfg(test)]
fn print_mat(mat: &Matrix) {
    for i in 0..N_ROWS {
        for j in 0..N_COLS {
            print!("{:.1} \t", mat[i][j]);
        }
        print!("\n");
    }
}

fn heat_spread_seq(iterations: usize) -> Vec<Matrix> {
    let mut time = init_time_vec(iterations);

    init_center_point(&mut time[0], 10.0);

    for i in 1..iterations {
        let old_mat = &time[i - 1];
        let mut new_mat = time[i].clone();


        for y in 1..(N_ROWS - 1) {
            for x in 1..(N_COLS - 1) {
                new_mat[y][x] = old_mat[y][x] + 
                    C * (old_mat[y + 1][x] + old_mat[y - 1][x] - 2. * old_mat[y][x]) +
                    C * (old_mat[y][x + 1] + old_mat[y][x - 1] - 2. * old_mat[y][x]);
            }
        }
        time[i] = new_mat;
    }
    time
}

fn heat_spread_par(iterations: usize) -> Vec<Matrix>{
    let pool = ThreadPool::new(10);

    let mut time: Vec<Matrix> = init_time_vec(iterations);

    let (tx, rx) = mpsc::channel();

    // Starting from index 1 since iteration 0
    // was already initialized in `init_time_vec`
    for i in 1..iterations {
        // 
        let old_mat_arc: Arc<_> = Arc::new(time[i - 1].clone());

        // Skipping boundaries: first and last rows
        for y in 1..(N_ROWS -1) {
            let mut new_row = time[i][y].clone();
            let old_mat = Arc::clone(&old_mat_arc);
            let sender = tx.clone();

            pool.execute(move || {
                for x in 1..(N_COLS - 1) {
                    new_row[x] = old_mat[y][x] + 
                        C * (old_mat[y + 1][x] + old_mat[y - 1][x] - 2. * old_mat[y][x]) +
                        C * (old_mat[y][x + 1] + old_mat[y][x - 1] - 2. * old_mat[y][x]);
                }

                sender.send((y, new_row)).unwrap();
            });
        }


        for _ in 1..(N_ROWS - 1){
            let (row_idx, row) = rx.recv().unwrap();
            time[i][row_idx] = row;
        }
    }
    time
}


fn init_time_vec(size: usize) -> Vec<Matrix> {
    let mut history = Vec::with_capacity(size);
    for _ in 0..size {
        history.push(vec![vec![0.0; N_COLS]; N_ROWS]);
    }

    init_center_point(&mut history[0], 10.0);

    history
}

#[inline(always)]
fn init_center_point(mat: &mut Matrix, initial_temp: f64) {
    mat[N_ROWS / 2][N_COLS / 2] = initial_temp;
}

enum Mode {
    SEQ,
    PAR
}

fn main() {
    // skip bin path
    let mut args = env::args().skip(1);

    let mut run_mode = Mode::SEQ;
    
    let mut iterations = 1000;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-s" | "-seq" => {
                run_mode = Mode::SEQ;
            },
            "-p" | "-par" | "--parallel" => {
                run_mode = Mode::PAR;
            },
            "-i" | "-iter" | "--iterations" => {
                iterations = util::parse_usize_flag(&arg, iterations, &mut args);
            }
            unkown =>  {
                println!("Skipping unknown argument: `{unkown}`");
            }
        }
    }


    match run_mode {
        Mode::SEQ => {
            heat_spread_seq(iterations);
        },
        Mode::PAR => {
            heat_spread_par(iterations);
        }
    }

}
