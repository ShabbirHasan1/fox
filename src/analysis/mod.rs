mod rings;
mod interpolation;
mod gradient_descent;
mod partitions;

#[derive(Clone, Copy)]
pub struct Trade {
    pub price: f64,
    pub timestamp: u64,
}

pub struct Ring<const N: usize> {
    ring: [Trade; N],
    sum: f64,
    average: f64,
    index: usize,
    full: bool,
    size: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Partition {
    sum: f64,
    average: f64,
    high: f64,
    low: f64,
    direction: f64,
    volume: usize,
}

pub struct Polynomial(Vec<f64>);
pub struct Interpolation;
pub struct StochasticGradientDescent;
