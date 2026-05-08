use std::fmt::Debug;
use nalgebra::DMatrix;
use crate::space::{Configuration, Space, Metric};

#[derive(Debug, PartialEq)]
pub enum MovementType {
    Cardinal,
    Diagonal,
}

impl MovementType {
    fn neighbors(&self) -> &[(i32, i32)] {
        match self {
            MovementType::Cardinal => &[(0, -1), (-1, 0), (1, 0), (0, 1)],
            MovementType::Diagonal => &[(-1,-1), (0,-1), (1,-1),
                                        (-1,  0),        (1, 0),
                                        (-1,  1), (0,1), (1, 1),],
        }
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

impl Configuration for GridCell {}

impl Metric<2> for GridCell {
    fn coords(&self) -> [f64; 2] {
        [self.x as f64, self.y as f64]
    }
}

pub struct GridMap {
    pub map: DMatrix<u8>,
    pub x_dim: usize,
    pub y_dim: usize,
    pub movement_type: MovementType,
}

impl GridMap {
    // from_matrix is a constructor, not part of the Environment trait
    pub fn from_matrix(map: DMatrix<u8>, movement_type: MovementType) -> Self {
        let y_dim = map.nrows();
        let x_dim = map.ncols();
        Self { map, x_dim, y_dim, movement_type }
    }
}

impl Space for GridMap {
    type Config = GridCell;

    fn is_occupied(&self, cell: &GridCell) -> bool {
        self.map[(cell.y, cell.x)] > 0
    }

    fn get_cost(&self, source: &Self::Config, goal: &Self::Config) -> f64 {
        match self.movement_type {
            MovementType::Cardinal => {return 1.0_f64;},
            MovementType::Diagonal => {
                let dx = source.x.abs_diff(goal.x);
                let dy = source.y.abs_diff(goal.y);

                if dx + dy == 2 {
                    return 2.0_f64.sqrt()
                } else {
                    return 1.0_f64
                }
            }
        }
    }

    fn in_bounds(&self, cell: &GridCell) -> bool {
        cell.x < self.x_dim && cell.y < self.y_dim
    }

    fn get_neighbors(&self, cell: &GridCell) -> Vec<GridCell> {


        // !TODO Add Pattern matching for cardinal vs diagonal heres
        // !TODO figure out best way to do cost
        let offsets: &[(i32, i32)] = self.movement_type.neighbors();
        
        let x = cell.x as i32;
        let y = cell.y as i32;

        offsets.iter().filter_map(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0
                || nx >= self.x_dim as i32
                || ny >= self.y_dim as i32
                || self.map[(ny as usize, nx as usize)] > 0
            {
                return None;
            }

            Some(GridCell { x: nx as usize, y: ny as usize })
        })
        .collect()
    }
}