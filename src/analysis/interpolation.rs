use crate::analysis::{Ring, Interpolation};
use crate::dydx::Position;

use std::{time, thread, sync::Arc};
use tokio::runtime::{Builder, Runtime};
use rayon::ThreadPoolBuilder;
use primitive_types::U512;

pub trait DataType {}

pub struct PriceEntry(f64, f64);
pub struct Orderbook(Position);

impl DataType for PriceEntry {}
impl DataType for Orderbook {}

impl Interpolation {

    // TODO: Make this a linreg weighted on different data types
    fn predict_for_type<const N: usize, D: DataType>(data_type: D, predictor: f64, lagrange_bases: [(f64, f64); N]) -> f64 
    {
        Interpolation::interpolate(predictor, lagrange_bases)
    }

    // Working on the stack and working single-threaded is faster since:
    // 1. We are dealing with low degree polynomials since no primitive types hold the immense
    //    calculations for higher degree polynomials. Also, smaller degree polynomials can just be
    //    stored on the stack, which speeds up runtime by ~4x.
    fn interpolate<const N: usize>(predictor: f64, lagrange_bases: [(f64, f64); N]) -> f64 {
        let n = lagrange_bases.len();
        let mut prediction = 0f64;
        for i in 0..n {
            let mut term = lagrange_bases[i].1;
            for j in 0..n {
                if i != j { 
                    term *= (predictor - lagrange_bases[j].0)/(lagrange_bases[i].0 - lagrange_bases[j].0);
                }
            }
            prediction += term
        }
        prediction
    }

    // Not used
    async fn tokio_interpolate<const N: usize>(predictor: f64, lagrange_bases: [(f64, f64); N], threads: usize) -> f64 {
        let mut prediction = 0f64;
        for i in 0..N {
            let term = tokio::spawn(async move {
                let mut term = lagrange_bases[i].1;
                for j in 0..N {
                    if i != j {
                        term *= (predictor - lagrange_bases[j].0)/(lagrange_bases[i].0 - lagrange_bases[j].0);
                    }
                }
                term
            }).await.unwrap();
            prediction += term;
        }
        prediction
    }

    // Not used
    fn rayon_interpolate<const N: usize>(predictor: f64, lagrange_bases: [(f64, f64); N], threads: usize) -> f64 {
        let mut prediction = 0f64;
        let thread_pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        for i in 0..N {
            prediction += thread_pool.install(move || {
                let mut term = lagrange_bases[i].1;
                for j in 0..N {
                    if i != j {
                        term *= (predictor - lagrange_bases[j].0)/(lagrange_bases[i].0 - lagrange_bases[j].0);
                    }
                }
                term
            });
        }
        prediction
    }

    // Not used
    fn mt_interpolate(predictor: f64, lagrange_bases: &Arc<Vec<(f64, f64)>>, threads: usize) -> f64 {
        let n = lagrange_bases.len();
        let mut prediction = 0f64;
        let step = n/threads;
        for t in 0..threads {
            let ic = Arc::clone(lagrange_bases);
            let partial_sum = thread::spawn(move || {
                let mut sum = 0f64;
                for i in (t * step)..((t+1) * step) {
                    let mut term = ic[i].1;
                    for j in 0..n {
                        if i != j {
                            term *= (predictor - ic[j].0)/(ic[i].0 - ic[j].0); 
                        }
                    }
                    sum += term;
                }
                sum
            });
            prediction += partial_sum.join().unwrap();
        }
        prediction
    }
}

#[cfg(test)]
mod interpolation_tests {

    use super::*;
    use std::time;
    use rand::Rng;
    
    #[test]
    fn interpolation() {
        let points: [(f64, f64); 4] = [(0.0, 2.0), (1.0, 3.0), (2.0, 12.0), (5.0, 147.0)];
        let predict = 3.0;
        let result = Interpolation::interpolate(predict, points.clone());
        let result_mt = Interpolation::mt_interpolate(predict, &Arc::new(points.to_vec()), 2);
        assert_eq!(result, 35f64);
        assert_eq!(result, result_mt);
    }

    #[test]
    fn price_prediction() {
        let prices: [(f64, f64); 10] = 
        [
            (15.0, 80.0), (17.0, 82.0), (20.0, 79.0), (25.0, 81.0), (26.0, 80.0), 
            (30.0, 76.0), (33.0, 74.0), (37.0, 78.0), (42.0, 84.0), (45.0, 80.0)
        ];
        let predict = 45.1;
        let result = Interpolation::predict_for_type(PriceEntry(0.0, 0.0), predict, prices);
        assert!(result - 80.0 < 5.0);
    }

    #[test]
    fn multithreaded_vs_single_interpolation() {
        let mut rng = rand::thread_rng();

        const DEGREE: usize = 500;

        let mut points: [(f64, f64); DEGREE] = [(0f64, 0f64); DEGREE];
        let mut c = 0f64;
        for (x, y) in points.iter_mut() {
            *x = c;
            *y = (rng.gen::<u16>() % 5000) as f64;
            c += 1f64;
        }
        let predict = (DEGREE) as f64 - 0.9999999;
        let single_thread = points.clone();
        let points = Arc::new(points.to_vec());

        let time_before = time::Instant::now();
        let st_result = Interpolation::interpolate(predict, single_thread.clone());
        let time_after = time::Instant::now();
        println!("Interpolation of degree {} took {:?} (Singlethreaded)", DEGREE, time_after - time_before);

        let pc = Arc::clone(&points);
        let time_before = time::Instant::now();
        let mt_result = Interpolation::mt_interpolate(predict, &pc, 2);
        let time_after = time::Instant::now();
        println!("Interpolation of degree {} took {:?} (Multithreaded with {} threads)", DEGREE, time_after - time_before, 2);
        println!("st result: {}, mt result: {}", st_result, mt_result);
    }
}
