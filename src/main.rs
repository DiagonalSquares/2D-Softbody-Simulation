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
        let mut softbody = simulation::SoftBody::new();
        softbody.points.push(simulation::Point::new([100.0, 100.0], 1.0));
        softbody.points.push(simulation::Point::new([300.0, 150.0], 1.0));
        softbody.springs.push(simulation::Spring::new(0, 1, 100.0));
        thread::sleep(std::time::Duration::from_millis(2000));
        loop {
            softbody.apply_spring_force(0);
            softbody.points[0].apply_all();
            softbody.points[1].apply_all();
            softbody.points[0].update();
            softbody.points[1].update();
            softbody.points[0].handle_edge_collision(&window_size);
            softbody.points[1].handle_edge_collision(&window_size);
            tx.send(softbody.clone()).unwrap();
            thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    while let Some(event) = window.next() {
        if let Some(_args) = event.render_args() {
            if let Some(softbody) = rx.try_iter().last() {
                window.draw_2d(&event, |c, g, _d| {
                    piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                    render::render_point(c, g, &softbody.points[0]);
                    render::render_point(c, g, &softbody.points[1]);
                });
            }
        }
    }
}