use nalgebra::dmatrix;
use seally::{
            algo::search::AStar, 
            common::heuristics::Chebyshev, 
            space::{GridCell, GridMap, MovementType}};

fn main() {
    let m = dmatrix![0, 0, 1;
                     0, 1, 0;
                     0, 0, 0];
    let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);
    let source = GridCell { x: 0, y: 0 };
    let goal = GridCell { x: 2, y: 2 };
    
    let astar = AStar::new(Chebyshev);
    let path: Option<Vec<GridCell>> = astar.find_path(&grid_map, &source, &goal);

    match path {
        Some(_) => println!("Path {:#?}", path),
        None => println!("No path exists from {:#?} to {:#?}", source, goal),
    }
}