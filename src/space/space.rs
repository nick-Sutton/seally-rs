use std::fmt::Debug;
use std::hash::Hash;

pub trait Configuration: Clone + Hash + Eq + Debug {}

pub trait Metric {
    fn coords(&self) -> Vec<f64>;
}

pub trait Space {
    type Config: Configuration;
    
    fn get_cost(&self, source: &Self::Config, goal: &Self::Config) -> f64;
    fn is_occupied(&self, config: &Self::Config) -> bool;
    fn in_bounds(&self, config: &Self::Config) -> bool;
    fn get_neighbors(&self, config: &Self::Config) -> Vec<Self::Config>;
}