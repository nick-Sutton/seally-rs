use crate::space::Metric;

pub fn chebyshev<C: Metric<N>, const N: usize>(a: &C, b: &C) -> f64 {
    let ac = a.coords();
    let bc = b.coords();
    ac.iter()
        .zip(bc.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(f64::NEG_INFINITY, f64::max)
}

pub fn euclidean<C: Metric<N>, const N: usize>(a: &C, b: &C) -> f64 {
    let ac = a.coords();
    let bc = b.coords();
    ac.iter()
        .zip(bc.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn manhattan<C: Metric<N>, const N: usize>(a: &C, b: &C) -> f64 {
    let ac = a.coords();
    let bc = b.coords();
    ac.iter()
        .zip(bc.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}