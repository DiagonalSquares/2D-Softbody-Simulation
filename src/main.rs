use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, sleep};
use piston::window;
use piston_window::{UpdateEvent, RenderEvent};

mod render;
mod ui;
mod input;
mod simulation;

use simulation::Point;

fn main() {
    let window_size = [800.0, 600.0];

    let (tx, rx): (Sender<Point>, Receiver<Point>) = mpsc::channel();

    println!("Starting Soft Body Simulation...");
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Soft Body Simulation", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();
/*
    thread::spawn(move || {
        let mut point1 = simulation::Point::new([100.0, 100.0], 1.0);
        thread::sleep(std::time::Duration::from_millis(2000));
        loop {
            
            tx.send(point1.clone()).unwrap();
            thread::sleep(std::time::Duration::from_millis(16));
        }
    });
*/
    let mut point1 = simulation::Point::new([100.0, 100.0], 1.0);
    while let Some(event) = window.next() {
        if let Some(_args) = event.update_args() {
            point1.apply_all();
            point1.update();
            point1.handle_edge_collision(window_size.clone());
            sleep(std::time::Duration::from_millis(16));
        }

        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |c, g, _d| {
                piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                render::render_point(c, g, point1.clone());
            });
        }
    }
}