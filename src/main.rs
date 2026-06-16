use macroquad::prelude::*;
use pathfinding::prelude::astar;

// Grid configuration
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 30;
const CELL_SIZE: f32 = 20.0;

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
    let px = pos.0 as f32 * CELL_SIZE;
    let py = pos.1 as f32 * CELL_SIZE;
    
    for poly in polygons {
        if is_point_in_polygon(px, py, poly) {
            return true;
        }
    }
    false
}

fn window_conf() -> Conf {
    Conf {
        window_title: "A* Pathfinding Map".to_owned(),
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Define "No-Fly Zone" Polygons (Coordinates in pixels)
    let polygons: Vec<Vec<(f32, f32)>> = vec![
        vec![(200.0, 100.0), (400.0, 150.0), (350.0, 300.0), (150.0, 250.0)],
        vec![(500.0, 300.0), (700.0, 200.0), (750.0, 450.0), (550.0, 500.0)],
    ];

    let start_node = Pos(2, 2);
    let goal_node = Pos(37, 27);

    // Optional: Load your background image here
    let background_tex = load_texture("world_map.png").await.unwrap();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // Dark faded background
        
        // Optional: Draw the image
        draw_texture(&background_tex, 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 0.3));

        // 1. Draw the network/grid points (faded in the background)
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let px = x as f32 * CELL_SIZE;
                let py = y as f32 * CELL_SIZE;
                draw_circle(px, py, 2.0, Color::new(0.5, 0.5, 0.5, 0.5));
            }
        }

        // 2. Draw the polygons (Light Blue)
        for poly in &polygons {
            for i in 0..poly.len() {
                let p1 = poly[i];
                let p2 = poly[(i + 1) % poly.len()];
                
                // Draw edges
                draw_line(p1.0, p1.1, p2.0, p2.1, 3.0, SKYBLUE);
                // Highlight polygon vertices
                draw_circle(p1.0, p1.1, 4.0, BLUE); 
            }
            
            // To fill them, we draw semi-transparent triangles from the first vertex
            for i in 1..poly.len()-1 {
                draw_triangle(
                    vec2(poly[0].0, poly[0].1), 
                    vec2(poly[i].0, poly[i].1), 
                    vec2(poly[i+1].0, poly[i+1].1), 
                    Color::new(0.68, 0.85, 0.9, 0.3) // Faded light blue fill
                );
            }
        }

        // 3. Calculate A* Path
        let result = astar(
            &start_node,
            |p| {
                let mut successors = Vec::new();
                // 8-way movement (straight and diagonal)
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        
                        let nx = p.0 + dx;
                        let ny = p.1 + dy;
                        let next_pos = Pos(nx, ny);

                        // Check bounds and collision
                        if nx >= 0 && nx < GRID_WIDTH && ny >= 0 && ny < GRID_HEIGHT {
                            if !is_in_any_polygon(&next_pos, &polygons) {
                                // Cost: 10 for straight, 14 for diagonal (approximation of sqrt(2)*10)
                                let cost = if dx == 0 || dy == 0 { 10 } else { 14 };
                                successors.push((next_pos, cost));
                            }
                        }
                    }
                }
                successors
            },
            |p| {
                // Heuristic: Chebyshev distance * 10
                ((p.0 - goal_node.0).abs().max((p.1 - goal_node.1).abs())) * 10
            },
            |p| *p == goal_node,
        );

        // 4. Draw the Path (Red Lines)
        if let Some((path, _cost)) = result {
            for i in 0..path.len() - 1 {
                let p1 = &path[i];
                let p2 = &path[i + 1];
                
                let x1 = p1.0 as f32 * CELL_SIZE;
                let y1 = p1.1 as f32 * CELL_SIZE;
                let x2 = p2.0 as f32 * CELL_SIZE;
                let y2 = p2.1 as f32 * CELL_SIZE;

                draw_line(x1, y1, x2, y2, 4.0, RED);
            }
        }

        // 5. Draw Start and End Points
        draw_circle(start_node.0 as f32 * CELL_SIZE, start_node.1 as f32 * CELL_SIZE, 6.0, GREEN);
        draw_circle(goal_node.0 as f32 * CELL_SIZE, goal_node.1 as f32 * CELL_SIZE, 6.0, ORANGE);

        next_frame().await
    }
}