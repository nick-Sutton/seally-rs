use macroquad::prelude::*;
use nalgebra::DMatrix;
use seally_rust::{algo::search::astar, common::heuristic::chebyshev, space::{GridCell, GridMap, MovementType}};

fn window_conf() -> Conf {
    let cell_size = 40;
    let grid_size = 3;
    Conf {
        window_title: "2D Grid".to_string(),
        window_width: (grid_size * cell_size) as i32,
        window_height: (grid_size * cell_size) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cell_size = 40.0;

    // Construct 9x9 zero matrix
    let m = DMatrix::<u8>::zeros(3, 3);

    // Create a Gridmap from the Matrix
    let grid_map = GridMap::from_matrix(m.clone(), MovementType::Diagonal);

    // Set a start and goal configuration
    let source = GridCell { x: 0, y: 0 };
    let goal = GridCell { x: 2, y: 2 };

    let path: Vec<GridCell> = astar(&grid_map, &source, &goal, chebyshev).unwrap();

    loop {
        clear_background(WHITE);
        // Draw path
        for cell in &path {
            draw_rectangle(
                cell.x as f32 * cell_size,
                cell.y as f32 * cell_size,
                cell_size,
                cell_size,
                BLUE,
            );
        }

        // Draw start and goal on top of path
        draw_rectangle(
            source.x as f32 * cell_size,
            source.y as f32 * cell_size,
            cell_size,
            cell_size,
            GREEN,
        );
        draw_rectangle(
            goal.x as f32 * cell_size,
            goal.y as f32 * cell_size,
            cell_size,
            cell_size,
            RED,
        );

        // Draw vertical lines
        for x in (0..=(screen_width() as i32)).step_by(cell_size as usize) {
            draw_line(x as f32, 0.0, x as f32, screen_height(), 1.0, LIGHTGRAY);
        }
        // Draw horizontal lines
        for y in (0..=(screen_height() as i32)).step_by(cell_size as usize) {
            draw_line(0.0, y as f32, screen_width(), y as f32, 1.0, LIGHTGRAY);
        }

        next_frame().await
    }
}