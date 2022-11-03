pub mod sequential;
pub mod parallel;


pub trait Contained {
    fn new(x: f64, y: f64) -> Self;
}

pub const MAX_ITER: usize = 10000;
