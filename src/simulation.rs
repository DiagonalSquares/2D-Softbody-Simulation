use piston::window;

const GRAVITY: [f64; 2] = [0.0, 0.098];

#[derive(Clone)]
pub struct Point {
    pub position: [f64; 2],
    force: [f64; 2],
    velocity: [f64; 2],
    mass: f64,
    friction: f64,
}

impl Point {
    pub fn new(position: [f64; 2], mass: f64) -> Self {
        Point {
            position,
            force: [0.0, 0.0],
            velocity: [0.0, 0.0],
            mass,
            friction: 0.99,
        }
    }

    pub fn apply_gravity(&mut self) {
        self.force[0] += GRAVITY[0];
        self.force[1] += GRAVITY[1];
    }

    pub fn apply_force(&mut self) {
        self.velocity[0] += self.force[0]/self.mass;
        self.velocity[1] += self.force[1]/self.mass;
    }

    pub fn apply_friction(&mut self) {
        self.velocity[0] *= self.friction;
        self.velocity[1] *= self.friction;
    }

    pub fn apply_all(&mut self) {
        self.apply_gravity();
        self.apply_force();
        self.apply_friction();
    }

    pub fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];

        self.force = [0.0, 0.0];
    }

    pub fn handle_edge_collision(&mut self, window_size: &[f64; 2]) {
        if self.position[0] < 0.0 {
            self.position[0] = 0.0;
            self.velocity[0] *= -1.0; 
        } else if self.position[0] > window_size[0] {
            self.position[0] = window_size[0];
            self.velocity[0] *= -1.0;
        }

        if self.position[1] < 0.0 {
            self.position[1] = 0.0;
            self.velocity[1] *= -1.0; 
        } else if self.position[1] > window_size[1] {
            self.position[1] = window_size[1];
            self.velocity[1] *= -1.0; 
        }
    }
}

#[derive(Clone)]
pub struct Spring {
    pub point1: usize,
    pub point2: usize,
    rest_length: f64,
    stiffness: f64,
    damping: f64,
}

impl Spring {
    pub fn new(point1: usize, point2: usize, rest_length: f64) -> Self {
        Spring {
            point1,
            point2,
            rest_length,
            stiffness: 0.5,
            damping: 0.3,
        }
    }
}

#[derive(Clone)]
pub struct SoftBody {
    pub points: Vec<Point>,
    pub springs: Vec<Spring>,
}

impl SoftBody {
    pub fn new() -> Self {
        SoftBody {
            points: Vec::new(),
            springs: Vec::new(),
        }
    }

    pub fn new_square(pos: [f64; 2], size: f64, faces: i32) -> Self {
        let mut soft_body = SoftBody::new();
        let mass = 1.0;
        let faces = faces+1;
        for i in 0..faces {
            for j in 0..faces {
                let x = pos[0] + (i as f64) * size / (faces - 1) as f64;
                let y = pos[1] + (j as f64) * size / (faces - 1) as f64;
                soft_body.points.push(Point::new([x, y], mass));
            }
        }

        for i in 0..faces {
            for j in 0..faces {
                let idx = (i * faces + j) as usize;
                if j < faces - 1 {
                    let right_idx = idx + 1;
                    soft_body.springs.push(Spring::new(idx, right_idx, size / (faces - 1) as f64));
                }
                if i < faces - 1 {
                    let down_idx = idx + faces as usize;
                    soft_body.springs.push(Spring::new(idx, down_idx, size / (faces - 1) as f64));
                }
            }
        }

        for i in 0..faces {
            for j in 0..faces {
                let idx = (i * faces + j) as usize;
                if i < faces - 1 && j < faces - 1 {
                    let down_right_idx = idx + faces as usize + 1;
                    soft_body.springs.push(Spring::new(idx, down_right_idx, (size / (faces - 1) as f64) * (2f64).sqrt()));
                }
                if i < faces - 1 && j > 0 {
                    let down_left_idx = idx + faces as usize - 1;
                    soft_body.springs.push(Spring::new(idx, down_left_idx, (size / (faces - 1) as f64) * (2f64).sqrt()));
                }
            }
        }

        soft_body
    }

    pub fn new_triangle(pos: [f64; 2]) -> Self {
        let mut soft_body = SoftBody::new();
        let idx1 = soft_body.points.len();
        soft_body.points.push(Point::new([pos[0], pos[1]-15.0], 1.0));
        let idx2 = soft_body.points.len();
        soft_body.points.push(Point::new([pos[0]+20.0, pos[1]], 1.0));
        let idx3 = soft_body.points.len();
        soft_body.points.push(Point::new([pos[0]-10.0, pos[1]], 1.0));

        soft_body.springs.push(Spring::new(idx1, idx2, 100.0));
        soft_body.springs.push(Spring::new(idx2, idx3, 100.0));
        soft_body.springs.push(Spring::new(idx3, idx1, 100.0));

        soft_body
    }

    pub fn get_points_len(&self) -> usize {
        self.points.len()
    }

    pub fn apply_spring_force(&mut self, spring_index: usize) {
        let spring = &self.springs[spring_index];
        let (i1, i2) = (spring.point1, spring.point2);

        let (p1, p2) = if i1 < i2 {
            let (left, right) = self.points.split_at_mut(i2);
            (&mut left[i1], &mut right[0])
        } else {
            let (left, right) = self.points.split_at_mut(i1);
            (&mut right[0], &mut left[i2])
        };

        let dx = p2.position[0] - p1.position[0];
        let dy = p2.position[1] - p1.position[1];
        let dist_sq = dx * dx + dy * dy;
        if dist_sq == 0.0 {
            return;
        }
        let distance = dist_sq.sqrt();

        let stretch   = distance - spring.rest_length;
        let force_mag = spring.stiffness * stretch;
        let fx = force_mag * dx / distance;
        let fy = force_mag * dy / distance;

        p1.force[0] +=  fx * spring.damping;
        p1.force[1] +=  fy * spring.damping;
        p2.force[0] -=  fx * spring.damping;
        p2.force[1] -=  fy * spring.damping;
    }

    pub fn point_collision(&mut self, point_index1: usize, point_index2: usize) {
        let (p1, p2) = if point_index1 < point_index2 {
            let (left, right) = self.points.split_at_mut(point_index2);
            (&mut left[point_index1], &mut right[0])
        } else {
            let (left, right) = self.points.split_at_mut(point_index1);
            (&mut right[0], &mut left[point_index2])
        };

        let dx = p2.position[0] - p1.position[0];
        let dy = p2.position[1] - p1.position[1];
        let dist_sq = dx * dx + dy * dy;
        if dist_sq == 0.0 {
            return;
        }
        let distance = dist_sq.sqrt();

        if distance < 20.0 {
            let overlap = 20.0 - distance;
            let force_mag = overlap * 0.5;
            p1.force[0] -= force_mag * dx / distance;
            p1.force[1] -= force_mag * dy / distance;
            p2.force[0] += force_mag * dx / distance;
            p2.force[1] += force_mag * dy / distance;
        }
    }

    pub fn update(&mut self, window_size: &[f64; 2]) {
        for i in 0..self.springs.len() {
            self.apply_spring_force(i);
        }

        for point in &mut self.points {
            point.apply_all();
            point.update();
        }

        for point in &mut self.points {
            point.handle_edge_collision(window_size);
        }

        for i in 0..self.points.len() {
            for j in (i + 1)..self.points.len() {
                self.point_collision(i, j);
            }
        }
    }
}