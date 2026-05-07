pub mod environment;
pub mod grid_map;

pub use environment::{Environment, Configuration};
pub use grid_map::{GridMap, GridCell, MovementType};

#[cfg(test)]
mod test;