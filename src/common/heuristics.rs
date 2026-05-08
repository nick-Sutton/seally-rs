use crate::space::Metric;

pub trait Heuristic<C> {
    fn estimate(&self, from: &C, to: &C) -> f64;
}

pub struct Euclidean;
pub struct Manhattan;
pub struct Chebyshev;

impl<C: Metric> Heuristic<C> for Euclidean {
    fn estimate(&self, from: &C, to: &C) -> f64 {
        let ac = from.coords();
        let bc = to.coords();
        ac.iter()
            .zip(bc.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl<C: Metric> Heuristic<C> for Manhattan {
    fn estimate(&self, from: &C, to: &C) -> f64 {
        let ac = from.coords();
        let bc = to.coords();
        ac.iter()
            .zip(bc.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

impl<C: Metric> Heuristic<C> for Chebyshev {
    fn estimate(&self, from: &C, to: &C) -> f64 {
        let ac = from.coords();
        let bc = to.coords();
        ac.iter()
            .zip(bc.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(f64::NEG_INFINITY, f64::max)
    }
}