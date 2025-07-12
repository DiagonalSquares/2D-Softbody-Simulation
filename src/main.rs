use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, sleep};
use piston::window;
use piston_window::{UpdateEvent, RenderEvent};

mod render;
mod ui;
mod input;
mod simulation;

use simulation::SoftBody;

fn main() {
    let window_size = [800.0, 600.0];

    let (tx, rx): (Sender<SoftBody>, Receiver<SoftBody>) = mpsc::channel();

    println!("Starting Soft Body Simulation...");
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Soft Body Simulation", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    thread::spawn(move || {
        let mut softbody = simulation::SoftBody::new_triangle([400.0, 300.0], [200.0, 100.0], [600.0, 300.0]);
        
        thread::sleep(std::time::Duration::from_millis(2000));
        loop {
            softbody.update(&window_size);
            tx.send(softbody.clone()).unwrap();
            thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    while let Some(event) = window.next() {
        if let Some(_args) = event.render_args() {
            if let Some(softbody) = rx.try_iter().last() {
                window.draw_2d(&event, |c, g, _d| {
                    piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                    render::render_softbody(c, g, &softbody);
                });
            }
        }
    }
}