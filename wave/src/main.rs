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
