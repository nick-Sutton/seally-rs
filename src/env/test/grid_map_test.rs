#[cfg(test)]
mod tests {
    use crate::env::{Environment, GridCell, GridMap, MovementType};
    use nalgebra::DMatrix;

    #[test]
    fn test_from_matrix() {

        // Construct 2x3 zero matrix
        let m = DMatrix::<u8>::zeros(2, 3);

        // Create a Gridmap from the Matrix
        let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);

        // Test that the map matches the matrix
        assert_eq!(m, grid_map.map);

        // Check that the dimensions of the map match the matrix
        assert_eq!(m.nrows(), grid_map.y_dim);
        assert_eq!(m.ncols(), grid_map.x_dim);
        
        // Test that the movement type was set correctly
        assert_eq!(MovementType::Diagonal, grid_map.movement_type)
    }

    #[test]
    fn test_is_occupied() {
        // Construct 2x3 matricies. One is full of zeros and one is full of ones
        let m_empty = DMatrix::<u8>::zeros(2, 3);
        let m_filled = DMatrix::<u8>::from_element(2, 3, 1);


        // Create a Gridmaps from the Matrix
        let grid_map_empty = GridMap::from_matrix(m_empty.clone(), MovementType::Diagonal);
        let grid_map_filled = GridMap::from_matrix(m_filled.clone(), MovementType::Diagonal);

        // Configuration to check for both maps
        let config = GridCell { x: 0, y: 0 };

        // The empty map should have no occupied gridcells
        assert!(!grid_map_empty.is_occupied(&config));

        // The filled gridmap should be occupied
        assert!(grid_map_filled.is_occupied(&config));

    }

    #[test]
    fn test_in_bounds() {
        // Construct 2x3 zero matrix
        let m = DMatrix::<u8>::zeros(2, 3);

        // Create a Gridmap from the Matrix
        let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);

        // Configuration just outside the bounds of the map
        let config_in = GridCell { x: 0, y: 0 };

        // Configuration just outside the bounds of the map
        let config_out = GridCell {x: 3, y: 2 };

        assert!(grid_map.in_bounds(&config_in));
        assert!(!grid_map.in_bounds(&config_out));
    }
}