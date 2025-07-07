use piston_window::*;

use crate::simulation;

pub fn render_all(c: Context, g: &mut G2d, point: &simulation::Point) {
    render_point(c, g, point);
}

pub fn render_point(c: Context, g: &mut G2d, point: &simulation::Point) {
    ellipse([1.0, 0.0, 0.0, 1.0], rectangle::centered_square(point.position[0], point.position[1], 5.0), c.transform, g);
}