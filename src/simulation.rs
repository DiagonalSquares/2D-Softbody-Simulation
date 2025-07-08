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
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring {
    pub fn new(point1: usize, point2: usize, rest_length: f32) -> Self {
        Spring {
            point1,
            point2,
            rest_length,
            stiffness: 0.5,
            damping: 0.95,
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
        let (fx, fy) = {
        let a = self.points[self.springs[spring_index].point1].clone();
        let b = self.points[self.springs[spring_index].point2].clone();

        let dx = b.position[0] - a.position[0];
        let dy = b.position[1] - a.position[1];
        let distance = (dx * dx + dy * dy).sqrt();

        let stretch = distance - self.springs[spring_index].rest_length as f64;
        let force_mag = self.springs[spring_index].stiffness as f64 * stretch;

        (if distance != 0.0 { force_mag * dx / distance } else { 0.0 },
         if distance != 0.0 { force_mag * dy / distance } else { 0.0 })
        };

        println!("Spring Force: fx = {}, fy = {}", fx, fy);

        self.points[self.springs[spring_index].point1].force[0] += fx;
        self.points[self.springs[spring_index].point1].force[1] += fy;
        self.points[self.springs[spring_index].point2].force[0] -= fx;
        self.points[self.springs[spring_index].point2].force[1] -= fy;
    }
}