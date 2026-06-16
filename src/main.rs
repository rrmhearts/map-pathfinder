use macroquad::prelude::*;
use pathfinding::prelude::astar;

const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 30;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

// Ray-casting algorithm to determine if a point is inside a polygon
fn is_point_in_polygon(x: f32, y: f32, polygon: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        if (polygon[i].1 > y) != (polygon[j].1 > y)
            && x < (polygon[j].0 - polygon[i].0) * (y - polygon[i].1) / (polygon[j].1 - polygon[i].1) + polygon[i].0
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn is_in_any_polygon(pos: &Pos, polygons: &[Vec<(f32, f32)>]) -> bool {
    // We now check collisions purely using grid coordinates!
    let px = pos.0 as f32;
    let py = pos.1 as f32;
    
    for poly in polygons {
        if is_point_in_polygon(px, py, poly) {
            return true;
        }
    }
    false
}

// Set fullscreen to true
fn window_conf() -> Conf {
    Conf {
        window_title: "A* Pathfinding Map".to_owned(),
        fullscreen: true, 
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Polygons are now defined in Grid Coordinates (X: 0 to 40, Y: 0 to 30)
    let polygons: Vec<Vec<(f32, f32)>> = vec![
        vec![(10.0, 5.0), (20.0, 7.5), (17.5, 15.0), (7.5, 12.5)],
        vec![(25.0, 15.0), (35.0, 10.0), (37.5, 22.5), (27.5, 25.0)],
    ];

    let start_node = Pos(2, 2);
    let goal_node = Pos(37, 27);

    // Uncomment this once you have your image in the same folder as Cargo.toml
    let background_tex = load_texture("world_map.png").await.ok();

    loop {
        // Exit if user presses ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Calculate dynamic cell sizes based on the current screen resolution
        let cell_w = screen_width() / GRID_WIDTH as f32;
        let cell_h = screen_height() / GRID_HEIGHT as f32;
        
        // 1. Draw the background image stretched to the whole screen
        if let Some(tex) = &background_tex {
            draw_texture_ex(
                tex,
                0.0,
                0.0,
                Color::new(1.0, 1.0, 1.0, 0.3), // Faded overlay
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }

        // 2. Draw the network/grid points dynamically
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let px = x as f32 * cell_w;
                let py = y as f32 * cell_h;
                draw_circle(px, py, 2.0, Color::new(0.5, 0.5, 0.5, 0.5));
            }
        }

        // 3. Draw the polygons (Light Blue) mapped to screen space
        for poly in &polygons {
            let mut scaled_poly = Vec::new();
            for p in poly {
                scaled_poly.push(vec2(p.0 * cell_w, p.1 * cell_h));
            }

            for i in 0..scaled_poly.len() {
                let p1 = scaled_poly[i];
                let p2 = scaled_poly[(i + 1) % scaled_poly.len()];
                
                draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, SKYBLUE);
                draw_circle(p1.x, p1.y, 4.0, BLUE); 
            }
            
            for i in 1..scaled_poly.len()-1 {
                draw_triangle(
                    scaled_poly[0], 
                    scaled_poly[i], 
                    scaled_poly[i+1], 
                    Color::new(0.68, 0.85, 0.9, 0.3) 
                );
            }
        }

        // 4. Calculate A* Path
        let result = astar(
            &start_node,
            |p| {
                let mut successors = Vec::new();
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        
                        let nx = p.0 + dx;
                        let ny = p.1 + dy;
                        let next_pos = Pos(nx, ny);

                        if nx >= 0 && nx < GRID_WIDTH && ny >= 0 && ny < GRID_HEIGHT {
                            if !is_in_any_polygon(&next_pos, &polygons) {
                                let cost = if dx == 0 || dy == 0 { 10 } else { 14 };
                                successors.push((next_pos, cost));
                            }
                        }
                    }
                }
                successors
            },
            |p| ((p.0 - goal_node.0).abs().max((p.1 - goal_node.1).abs())) * 10,
            |p| *p == goal_node,
        );

        // 5. Draw the Path
        if let Some((path, _cost)) = result {
            for i in 0..path.len() - 1 {
                let p1 = &path[i];
                let p2 = &path[i + 1];
                
                let x1 = p1.0 as f32 * cell_w;
                let y1 = p1.1 as f32 * cell_h;
                let x2 = p2.0 as f32 * cell_w;
                let y2 = p2.1 as f32 * cell_h;

                draw_line(x1, y1, x2, y2, 4.0, RED);
            }
        }

        // 6. Draw Start and End Points
        draw_circle(start_node.0 as f32 * cell_w, start_node.1 as f32 * cell_h, 6.0, GREEN);
        draw_circle(goal_node.0 as f32 * cell_w, goal_node.1 as f32 * cell_h, 6.0, ORANGE);

        next_frame().await
    }
}