use std::collections::HashSet;
use piston_window::*;
use std::sync::mpsc::{Receiver, Sender};
use crate::simulation::{self, SoftBodyCollection};

pub struct Input_Handler {
    pub mouse_pos: [f64; 2],
    pub mouse_down: bool,
    pub held_point_index: Option<usize>,
    pub softbody_index: Option<usize>,
}

impl Input_Handler {
    pub fn new() -> Self {
        Input_Handler {
            mouse_pos: [0.0, 0.0],
            mouse_down: false,
            held_point_index: None,
            softbody_index: None,
        }
    }

    pub fn handle_mouse_move(&mut self, pos: [f64; 2]) {
        self.mouse_pos = pos;
    }

    pub fn handle_mouse_down(&mut self, softbodies: SoftBodyCollection) {
        self.mouse_down = true;
        let mut found = false;
        for (sbi, sb) in softbodies.softbodies.iter().enumerate() {
            if let Some(pi) = sb.points.iter().position(|p| {
                let dx = p.position[0] - self.mouse_pos[0];
                let dy = p.position[1] - self.mouse_pos[1];
                (dx * dx + dy * dy).sqrt() < 10.0
            }) {
                self.softbody_index = Some(sbi);
                self.held_point_index = Some(pi);
                found = true;
                 break;
            }
        }
        if !found {
            self.softbody_index = None;
            self.held_point_index = None;
        }
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_down = false;
        self.held_point_index = None;
        self.softbody_index = None;
    }
}