# Map Pathfinder

A 2D pathfinding visualization application built in Rust. It generates a grid overlay on top of a customizable background image and calculates the most efficient route between a starting point and a destination using the A-star pathfinding algorithm. The application supports dynamic resolution, automatically scaling to fit fullscreen environments, and routes around user-defined polygonal obstacles representing restricted zones.

## Prerequisites

To build and run this application, you will need Rust and Cargo installed on your system. The project relies on the Macroquad crate for graphics and the Pathfinding crate for algorithmic calculations.

## Installation and Execution

Navigate your terminal to the root directory of the project where the Cargo.toml file is located.

If you wish to use a custom background image, place a valid image file named world_map.png in the root directory and uncomment the texture loading lines within the source code.

Execute the following command to compile and launch the application:

> cargo run --release

### Usage and Controls

The application can be launched in fullscreen mode (`fullscreen` branch), mapping the internal grid to your monitor's resolution. The start node is rendered in green, and the goal node is rendered in orange. The calculated path appears as a continuous red line circumventing the light blue polygon boundaries.

In fullscreen mode, press the Escape key to close the application and return to your desktop safely.

### Technical Details

The visualization is powered by the Macroquad framework. Pathfinding logic is handled by the pathfinding crate using an implementation of the A-star algorithm.

The application evaluates adjacent nodes using an eight-way movement system. It applies a cost of 10 for orthogonal steps and 14 for diagonal steps to approximate physical distances efficiently. Collision detection against restricted zones relies on a mathematical ray-casting algorithm to determine if any given grid coordinate falls inside a bounded polygon. If a node is determined to be inside a polygon, the algorithm discards it and routes around the obstacle.