use crate::{render, simulation, ui, input};

pub struct App {
    pub window_size: [f64; 2],
    pub mouse_pos: [f64; 2],
    pub mouse_down: bool,
    pub held_point_index: Option<usize>,
    pub softbody_index: Option<usize>,
    pub to_sim_tx: std::sync::mpsc::Sender<simulation::SoftBodyCollection>,
    pub to_sim_rx: std::sync::mpsc::Receiver<simulation::SoftBodyCollection>,
    pub from_sim_tx: std::sync::mpsc::Sender<simulation::SoftBodyCollection>,
    pub from_sim_rx: std::sync::mpsc::Receiver<simulation::SoftBodyCollection>,
}

impl App {
    pub fn new() -> Self {
        let window_size = [800.0, 600.0];
        let (to_sim_tx, to_sim_rx) = std::sync::mpsc::channel();
        let (from_sim_tx, from_sim_rx) = std::sync::mpsc::channel();

        App {
            window_size,
            mouse_pos: [0.0, 0.0],
            mouse_down: false,
            held_point_index: None,
            softbody_index: None,
            to_sim_tx,
            to_sim_rx,
            from_sim_tx,
            from_sim_rx,
        }
    }
}