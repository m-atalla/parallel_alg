//! 
//! ## Programming Model
//! Manual parallelization using thread pool and 
//! a multiple producer single consumer (mpsc) channel for collecting results.
//! 
//! #### Thread operation:
//! Can be summarized in the following [!figure](./par_solution.png)
//! 
//! ## Partitioning
//! **Domain decomposition**: each task is a row of the grid to be calculated.
//! 
//! ## Communication
//! Collective communication: scatter and gather operation done by the main thread. 
//! 
//! #### Main thread communication sequence:
//! - Make an **atomic** reference counter 
//! (Arc: atomic reference counters are used to safely share pointers between threads) 
//! to the previous grid
//! - Send a row of the grid to the thread pool to be executed
//! - Collect the results and update the new grid
//! - Add the new grid to the "grid timeline" array for reuse in the next iteration
//! 
//! No need for inter-thread communication as there is no dependency between the separate data
//! partitions. The previous grid is duplicated and so no need for communication as we broke the dependencies with adjacent cells.
//! 
//! ## Synchronization
//! - **Lock / Semaphore**: internally the thread pool send a Mutex (mutual exclusion lock) of a point to a job (a function
//! pointer) the first thread to acquire the lock gets to execute the job
//! - **Synchronous communication operations**: through the mpsc channel discussed earlier to
//! scatter/reduce the data/results.
//! 
//! ## Data Dependencies
//! - The problem implies data dependencies with adjacent cells. However, we can break this
//! dependency if we keep a copy of the previous state. So the solution here duplicates the
//! space to avoid communication and synchronization overhead.
//! 
//! - There is also a hard dependency that we cannot get around, that each "step" of the heat time is dependent on the previous step result.
//! 
//! ## Partitioning
//! **Equally partitioned** individual rows are calculated by threads for a given grid.
//! 
//! ## Granularity
//! **Coarse grained** the communication part is small, as it is only sending the result of the new grid _row_ to the main thread.
//! While the majority of the computation is done in the thread without any extra need for communication during the computation.
//! 
//! ## I/O
//! Not really a bottleneck in this problem as I/O is only used to display the final output.
//! 
//! ## Performance Analysis
//! Done using **Perf** Linux profiler.
//!
//!

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
