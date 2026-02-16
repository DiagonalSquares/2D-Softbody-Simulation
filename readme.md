# 2D Softbody Simulation

A simple, real-time, 2-dimensional physics program written in Rust utilizing a mass-spring model approach to simulating softbodies

## Overview

This project aims to create a fast and efficient simulation of softbodies, accurately modeling deformities and bounciness typically seen associated with these squishy shapes

## Key Features

- Real-time softbody deformation
- Adjustable settings (inside code)
- User interactions like point dragging
- Softbody spawning
- multithreaded for a more smooth experience

## Installation

```bash
# 1. Clone the repository
git clone https://github.com/DiagonalSquares/2D-Softbody-Simulation.git

# 2. Navigate into the folder
cd 2D-Softbody-Simulation

# 3. Install dependencies
cargo build

# 4. Run the simulation
cargo run main.rs
