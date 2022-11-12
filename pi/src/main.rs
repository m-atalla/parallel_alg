use std::ops::Range;
use std::env;
use std::sync::mpsc;
use rand::{self, distributions::Uniform, prelude::Distribution};
use threads::ThreadPool;

const MAX: f64 = 0.5;
const MIN: f64 = -0.5;
const R: f64 = MAX;

pub fn is_in_circle((x, y): &(f64, f64)) -> bool {
    x.powi(2) + y.powi(2) < R.powi(2)
}


fn estimate_pi_parallel(n_points: usize) {
    let pool = ThreadPool::new(5);

    let chunk_size: usize = n_points / pool.size(); 

    let mut chunk_ranges = Vec::with_capacity(pool.size());

    let mut current = 0;
    
    let (tx, rx) = mpsc::channel();

    for _ in 0..pool.size() {
        chunk_ranges.push(current..current+chunk_size);
        current += chunk_size;
    }
    
    println!("Chunk ranges len: {}", chunk_ranges.len());
    for chunk_range in chunk_ranges {
        let thread_result = tx.clone();
        pool.execute(move || {
            let in_count = count_points_in_circle(chunk_range);
            thread_result.send(in_count).unwrap();
        });
    }

    

    let mut in_count = 0usize; 

    for _ in 0..pool.size() {
        in_count += rx.recv().unwrap();
    }

    // Accumelate thread results
    let pi = pi_estinate(in_count, n_points);

    println!("PI = {pi}");
}

fn count_points_in_circle(range: Range<usize>) -> usize {
    let uniform_range = Uniform::from(MIN..MAX);

    let mut rng = rand::thread_rng();

    range.map(|_| {
        (uniform_range.sample(&mut rng), uniform_range.sample(&mut rng))
    }).filter(|coords| {
        is_in_circle(coords)
    }).count() 
}

#[inline(always)]
fn pi_estinate(in_count: usize, n_points: usize) -> f64 {
    4.0f64 * (in_count as f64 / n_points as f64)
}
fn estimate_pi_seq(n_points: usize) {
    let in_count = count_points_in_circle(0..n_points);
    let pi: f64 = pi_estinate(in_count, n_points);
    println!("pi = {pi}");
}

fn main() {
    let mut args = env::args();
    
    if let Some(arg) = args.nth(1) {

        match arg.as_str() {
            "-p" => {
                estimate_pi_parallel(1_000_000);
            },
            "-s" => {
                estimate_pi_seq(1_000_000);
            },
            unkown => {
                println!("Unkown argument: {unkown}");
            }
        }
    }
}
