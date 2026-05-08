#[cfg(test)]
mod tests {
    use crate::space::{GridCell, GridMap, MovementType};
    use nalgebra::DMatrix;
    use crate::algo::search::astar;
    use crate::common::heuristic::chebyshev;

    #[test]
    fn test_a_star() {
        // Construct 9x9 zero matrix
        let m = DMatrix::<u8>::zeros(3, 3);

        // Create a Gridmap from the Matrix
        let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);

        // Set a start and goal configuration
        let source = GridCell { x: 0, y: 0 };
        let goal = GridCell { x: 2, y: 2 };

        let path: Vec<GridCell> = astar(&grid_map, &source, &goal, chebyshev).unwrap();

        assert!(!path.is_empty());
        assert!(path.get(0).unwrap().x == 0);
        assert!(path.get(0).unwrap().y == 0);

        assert!(path.get(1).unwrap().x == 1);
        assert!(path.get(1).unwrap().y == 1);

        assert!(path.get(2).unwrap().x == 2);
        assert!(path.get(2).unwrap().y == 2);        
    }
}