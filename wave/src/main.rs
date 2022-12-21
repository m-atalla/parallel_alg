//! ## Programming Model
//! Manual parallelization using thread pool and a multiple producer single consumer (mpsc) channel for collecting results.
//!
//! 
//! #### Thread operation:
//! - Calculate the given point using the provided formula in the problem statement..
//! - Send the result to the main thread through the mpsc channel
//! 
//! ## Partitioning
//! **Domain decomposition**: the data points are divided evenly for each thread.
//! 
//! ## Communication
//! Collective communication: scatter and gather operation done by the main thread. 
//! 
//! #### Main thread communication sequence:
//! - Send points to be calculated to the thread pool
//! - Collect the result
//! - Add the result to the current wave array
//! - Add the new wave to the "waves timeline" array for next waves calculations
//!
//! No need for inter-thread communication as there is no dependency between the separate data
//! partitions.
//! 
//! ## Synchronization
//! - **Lock / Semaphore**: internally the thread pool send a Mutex (mutual exclusion lock) of a point to a job (a function
//! pointer) the first thread to acquire the lock gets to execute the job
//! - **Synchronous communication operations**: through the mpsc channel discussed earlier to
//! scatter/reduce the data/results.
//! 
//! ## Data Dependencies
//! There are dependencies between waves as calculated a point in the current wave (t) requires the
//! equivalent point in the previous two waves (t - 1 and t - 2) and so we cannot parallelize
//! waves. However, we can parallelize a single wave points calculation.
//! 
//! ## Partitioning
//! **Equally partitioned** individual points are calculated by threads for a given wave.
//! 
//! ## Granularity
//! **Coarse grained** the communication part is small, as it is only sending the integer result of
//! the total points inside the circle through the channel. 
//! While the majority of the computation is done in the thread without any extra need for communication during the computation.
//! 
//! ## I/O
//! Not really a bottleneck in this problem as I/O is only used to display the final output.
//! 
//! ## Performance Analysis
//! Done using **Perf** Linux profiler.
//!
use std::{sync::{mpsc, Arc}, env};

use threads::ThreadPool;

const WAVE_C: f64 = 0.5;
const MAX_X: usize = 10;
const MAX_T: usize = 10;

fn wave_eq_seq() {
    let mut init: Vec<f64> = (0..MAX_X - 1).into_iter().map(|x| (x as f64).sin()).collect();
    init.push(0.0);

    let zero: Vec<f64> = init.iter().map(|x| (x * 0.0).abs()).collect();
    let mut time: Vec<Vec<f64>> = Vec::with_capacity(MAX_T);

    // preconditions
    time.push(zero);
    time.push(init);


    for t in 2..MAX_T {
        let mut wave = Vec::with_capacity(MAX_X);
        wave.push(0.0);

        for i in 1..(MAX_X - 1) {
            let point = (2.0 * time[t-1][i]) - time[t-2][i]
                + ( WAVE_C * (time[t-1][i-1]) - (2.0 * time[t-1][i]) + time[t-1][i+1]);
            wave.push(point);
        }

        wave.push(0.0);

        time.push(wave);
    }


    for t in time {
        println!("{t:?}");
    }
}

fn wave_eq_par() {
    let pool = ThreadPool::new(5);

    let mut init: Vec<f64> = (0..MAX_X - 1).into_iter().map(|x| (x as f64).sin()).collect();
    init.push(0.0);

    let zero: Vec<f64> = init.iter().map(|x| (x * 0.0).abs()).collect();
    let mut time: Vec<Vec<f64>> = Vec::with_capacity(MAX_T);

    // preconditions
    time.push(zero);
    time.push(init);

    let (tx, rx) = mpsc::sync_channel(0);

    for t in 2..MAX_T {

        let mut wave = Vec::with_capacity(MAX_X);

        let parent = Arc::new(time[t - 1].clone());
        let grandparent = Arc::new(time[t - 2].clone());
            
        // wave boundry
        wave.push(0.0);

        for i in 1..(MAX_X - 1) {

            let sender = tx.clone();
            let t_1 = Arc::clone(&parent);
            let t_2 = Arc::clone(&grandparent);

            pool.execute(move || {
                let point = (2.0 * t_1[i]) - t_2[i]
                    + ( WAVE_C * (t_1[i-1]) - (2.0 * t_1[i]) + t_1[i+1]);

                sender.send(point).unwrap();
            });
        }

        for _ in 1..(MAX_X - 1) {
            let received = rx.recv().unwrap();
            wave.push(received);
        }

        // wave boundry
        wave.push(0.0);

        time.push(wave);
    }

    for t in time {
        println!("{t:?}");
    }
}

fn main() {
    let mut args = env::args().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-s" => wave_eq_seq(),
            "-p" => wave_eq_par(),
            unknown => {
                eprintln!("Err: unknown flag `{unknown}`, exiting..");
                return;
            }
        }
    };
}
