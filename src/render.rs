use piston_window::*;

use crate::{simulation};

pub fn render_all_softbodies(c: Context, g: &mut G2d, softbodies: &[simulation::SoftBody]) {
    for softbody in softbodies {
        render_softbody(c, g, softbody);
    }
}

pub fn render_softbody(c: Context, g: &mut G2d, softbody: &simulation::SoftBody) {
    for i in 0..softbody.springs.len() {
        render_spring(c, g, softbody, i);
    }

    for point in &softbody.points {
        render_point(c, g, point);
    }
}

pub fn render_point(c: Context, g: &mut G2d, point: &simulation::Point) {
    ellipse([1.0, 0.0, 0.0, 1.0], rectangle::centered_square(point.position[0], point.position[1], 5.0), c.transform, g);
}

pub fn render_spring(c: Context, g: &mut G2d, softbody: &simulation::SoftBody, spring: usize) {
    let pos1 = softbody.points[softbody.springs[spring].point1].position;
    let pos2 = softbody.points[softbody.springs[spring].point2].position;

    let dx = pos2[0] - pos1[0];
    let dy = pos2[1] - pos1[1];

    let distance_sq = dx * dx + dy * dy;
    let distance = distance_sq.sqrt();

    let color = (softbody.springs[spring].rest_length / distance) as f32;
    line([1.0 - color, color - 1.0, color, 1.0], 1.0, 
        [pos1[0], pos1[1], pos2[0], pos2[1]],
        c.transform, g)
}