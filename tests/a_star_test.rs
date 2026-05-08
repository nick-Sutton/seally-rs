use nalgebra::DMatrix;
use seally::algo::search::AStar;
use seally::space::{GridMap, GridCell, MovementType};
use seally::common::heuristics::Chebyshev;

#[test]
fn test_astar() {
    let m = DMatrix::<u8>::zeros(3, 3);
    let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);
    let source = GridCell { x: 0, y: 0 };
    let goal = GridCell { x: 2, y: 2 };
    
    let astar = AStar::new(Chebyshev);
    let path: Vec<GridCell> = astar.find_path(&grid_map, &source, &goal).unwrap();

    assert!(!path.is_empty());
    assert!(path.get(0).unwrap().x == 0);
    assert!(path.get(0).unwrap().y == 0);
    assert!(path.get(1).unwrap().x == 1);
    assert!(path.get(1).unwrap().y == 1);
    assert!(path.get(2).unwrap().x == 2);
    assert!(path.get(2).unwrap().y == 2);
}