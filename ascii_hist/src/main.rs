use std::fs;
use std::sync::mpsc;
use threads::ThreadPool;
use std::thread;

fn parallel_freq_count(slice: Vec<u8>) -> [usize; 128] {

    let mut result_accum = [0; 128];
    // gets the number of available cores on the current machine.
    let n_cores = thread::available_parallelism().unwrap().get();

    println!("Creating a thread with a totoal of: {n_cores} threads");

    let pool = ThreadPool::new(n_cores);

    // estimate the number of
    let chunk_size = slice.len() / n_cores;

    // partition data for each thread.
    let chunks = slice.chunks(chunk_size);

    let (tx, rx) = mpsc::channel();

    let n_parts = chunks.len();

    //
    for chunk in chunks {

        let chunk = chunk.to_owned();
        let t_result = tx.clone();

        pool.execute(move || {
            let result = ascii_frequency(&chunk);
            t_result.send(result).unwrap();
        });
    }

    for _ in 0..n_parts {
        let res = rx.recv().unwrap();

        for (idx, &n) in res.iter().enumerate() {
            result_accum[idx] += n;
        }
    }
    println!("chunk_size = {chunk_size}");
    println!("data len = {}", slice.len());

    result_accum
}

fn ascii_frequency(slice: &[u8]) -> [usize; 128] {
    let mut freq = [0; 128];
    for &n in slice {
        freq[n as usize] += 1;
    }

    freq
}

/// prints frequency array
fn print_freq(freq: &[usize; 128]) {
    for (i, &n) in freq.iter().enumerate() {
        if n > 0 {
            let ch = (i as u8) as char;
            match ch {
                '\n' => println!("'\\n': {}", n),
                ' ' => println!("'<SPACE>': {}", n),
                '\t' => println!("'<TAB>': {}", n),
                other_char => println!("'{}': {}", other_char, n)
            }
        }
    }
}

fn main() {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parallel_freq_count_is_correct() {
        let input = fs::read("./bird.txt")
            .expect("Unable to read file.");

        let seq_freq = ascii_frequency(&input);

        print_freq(&seq_freq);

        let par_frq = parallel_freq_count(input);

        print_freq(&par_frq);


        for (&a, b) in seq_freq.iter().zip(par_frq) {
            assert_eq!(a, b);
        }
    }
}
