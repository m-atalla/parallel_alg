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
