use std::{fs, env::{self, Args}};
use kmeans::{Constructed, sequential, parallel};

fn read_points_csv<T: Constructed>(path: &str, container: &mut Vec<T>) {
    let contents = fs::read_to_string(path)
        .expect(&format!("A file to exist in the given path: {}", path));

    // parsing CSV line by line,
    // skipping the first line since it contains the header metadata.
    for line in contents.lines().skip(1) {
        let comps: Vec<&str> = line.split(",").collect();

        // skip rows that doesn't have exactly 2 entries
        if comps.len() != 2 {
            continue;
        }

        // if either of x or y fails to be parsed to f64 that line should also be skipped
        let x = match comps.get(0).unwrap().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };

        let y = match comps.get(1).unwrap().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };

        container.push(T::new(x, y));
    }
}


const DEFAULT_K: usize = 3;

const DEFAULT_MAX_ITER: usize = 10000;

const DEFAULT_N_THREADS: usize = 12;

enum ExecMode {
    SEQ,
    PAR
}

fn parse_usize_flag(flag_name: &str, default_value: usize, iter: &mut Args) -> usize {
    match iter.next() {
        Some(num_arg) => {
            num_arg.parse::<usize>().unwrap_or_else(|_| {
                eprintln!("Expected a numeric argument after `{flag_name}` flag, got: {}", num_arg);
                default_value
            })
        },
        None => {
            eprintln!("Missing argument after `-k` flag, using default {flag_name}={default_value}");
            default_value
        }
    }
}

fn main() {
    let mut args = env::args();
    
    let mut k = DEFAULT_K;
    
    let mut mode = ExecMode::PAR;

    let mut max_iter: usize = DEFAULT_MAX_ITER;

    let mut n_threads: usize = DEFAULT_N_THREADS;

    args.next().expect("bin");

    while let Some(arg) = args.next() { 
        match arg.as_str() {
            "-k" => {
                k = parse_usize_flag("-k", DEFAULT_K, &mut args)
            },
            "-i" => {
                max_iter = parse_usize_flag("-i", DEFAULT_MAX_ITER, &mut args)
            },
            "-t" => {
                n_threads = parse_usize_flag("-t", DEFAULT_N_THREADS, &mut args)
            }
            "-p" => {
                mode = ExecMode::PAR;
            }
            "-s" => {
                mode = ExecMode::SEQ;
            },
            unkown_arg => {
                eprintln!("Unkown argument provided: {unkown_arg}");
                return;
            }
        }
    }

    match mode {
        ExecMode::SEQ => {
            let mut points = Vec::new();
            read_points_csv("./xclara.csv", &mut points);
            sequential::kmeans(points, k, max_iter);
        },
        ExecMode::PAR => {
            let mut points = Vec::new();
            read_points_csv("./xclara.csv", &mut points);
            parallel::kmeans(points, k, max_iter, n_threads);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_can_perform_a_kmeans_iteration() {
        let points = [
            sequential::Point::new(2.0, 10.0),
            sequential::Point::new(2.0, 5.0),
            sequential::Point::new(8.0, 4.0),
            sequential::Point::new(5.0, 8.0),
            sequential::Point::new(7.0, 5.0),
            sequential::Point::new(6.0, 4.0),
            sequential::Point::new(1.0, 2.0),
            sequential::Point::new(4.0, 9.0),
        ];
        
        for point in &points {
            println!("{:?}", point);
        }

        let point_vec = Vec::from(points);

        sequential::kmeans(point_vec, 3, 1);
    }
}
