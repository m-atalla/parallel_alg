pub mod sequential;
pub mod parallel;
pub mod threads;


pub trait Constructed {
    fn new(x: f64, y: f64) -> Self;
}

#[macro_export]
macro_rules! print_clusters {
    ($clusters: expr) => {
        println!("\nFinal clusters:");
        for cluster in $clusters {
            println!("\t{:?}", cluster);
        }
    };
}

pub const MAX_ITER: usize = 10000;
