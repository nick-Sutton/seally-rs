use macroquad::prelude::*;

#[macroquad::main("3D Grid")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        // Set up a basic 3D camera
        set_camera(&Camera3D {
            position: vec3(10.0, 10.0, 10.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // Draws a grid centered at (0, 0, 0)
        // slices: number of lines, spacing: distance between lines
        draw_grid(20, 1.0, BLACK, GRAY);

        set_default_camera();
        next_frame().await
    }
}