pub use std::time::Instant;

#[macro_export]
/// Profiles an expression 
macro_rules! time_eval {
    ($name: expr, $expression:expr) => {
        print!($name);

        let now = Instant::now();

        $expression
            
        let elapsed = now.elapsed();

        print!("Done!, Elapsed: {:.2?}\n", elapsed);
    };
}



pub fn parse_usize_flag(flag_name: &str, default_value: usize, iter: &mut impl Iterator<Item = String>) -> usize {
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
