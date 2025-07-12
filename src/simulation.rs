const GRAVITY: [f64; 2] = [0.0, 0.98];

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
    pub rest_length: f64,
    pub stiffness: f64,
    pub damping: f64,
}

impl Spring {
    pub fn new(point1: usize, point2: usize, rest_length: f64) -> Self {
        Spring {
            point1,
            point2,
            rest_length,
            stiffness: 0.5,
            damping: 0.10,
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
}