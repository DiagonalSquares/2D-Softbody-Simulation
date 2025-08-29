use std::sync::mpsc::{Sender};
use piston_window::*;

use crate::simulation::{self, SoftBodyCollection};

pub struct Button {
    position: [f64; 2],
    size: [f64; 2],
    color: [f32; 4],
    label: String,
}

impl Button {
    pub fn new(position: [f64; 2], size: [f64; 2], color: [f32; 4],label: String ) -> Self {
        Button {
            position,
            size,
            color,
            label,
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        rectangle(self.color, [self.position[0], self.position[1], self.size[0], self.size[1]], c.transform, g);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
            .draw(&self.label, glyphs, &c.draw_state, c.transform.trans(self.position[0], self.position[1] + 50.0), g)
            .unwrap();
    }

    pub fn click_range(&self, mouse_pos: [f64; 2]) -> bool {
        mouse_pos[0] >= self.position[0] &&
        mouse_pos[0] <= self.position[0] + self.size[0] &&
        mouse_pos[1] >= self.position[1] &&
        mouse_pos[1] <= self.position[1] + self.size[1]
    }

    pub fn handle_click_spawn(&self, mouse_pos: [f64; 2], mut softbodies: SoftBodyCollection, to_sim_tx: &Sender<simulation::SoftBodyCollection>) {
        if self.click_range(mouse_pos) {
            let new_softbody = simulation::SoftBody::new_square([200.0, 100.0], 100.0, 4);
            softbodies.add(new_softbody);
            to_sim_tx.send(softbodies).unwrap();
            println!("Spawned new softbody!");
        }
    }

    pub fn handle_click_pause(&self, mouse_pos: [f64; 2], pause: bool) -> bool {
        if self.click_range(mouse_pos) {
            return !pause;
        }
        pause
    }
}

struct AllButtons {
    buttons: Vec<Button>,
}