use piston_window::*;

use crate::simulation;

pub fn render_softbody(c: Context, g: &mut G2d, softbody: &simulation::SoftBody) {
    for point in &softbody.points {
        ellipse([0.0, 1.0, 0.0, 1.0], rectangle::centered_square(point.position[0], point.position[1], 5.0), c.transform, g);
    }

    for (i, spring) in softbody.springs.iter().enumerate() {
        line([0.0, 0.0, 1.0, 1.0], 1.0, 
            [softbody.points[spring.point1].position[0], 
             softbody.points[spring.point1].position[1],
             softbody.points[spring.point2].position[0],
             softbody.points[spring.point2].position[1]],
            c.transform, g);
    }
}

pub fn render_point(c: Context, g: &mut G2d, point: &simulation::Point) {
    ellipse([1.0, 0.0, 0.0, 1.0], rectangle::centered_square(point.position[0], point.position[1], 5.0), c.transform, g);
}

pub fn render_spring(c: Context, g: &mut G2d, softbody: &simulation::SoftBody, spring: usize) {
    line([0.0, 0.0, 1.0, 1.0], 1.0, 
        [softbody.points[softbody.springs[spring].point1].position[0], 
            softbody.points[softbody.springs[spring].point1].position[1],
            softbody.points[softbody.springs[spring].point2].position[0],
            softbody.points[softbody.springs[spring].point2].position[1]],
        c.transform, g)
}