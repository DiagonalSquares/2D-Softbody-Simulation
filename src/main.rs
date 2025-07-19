use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use piston::{mouse, window};
use piston_window::{UpdateEvent, RenderEvent};

mod render;
mod ui;
mod input;
mod simulation;

use simulation::SoftBody;

fn main() {
    let window_size = [800.0, 600.0];
    let mut mouse_pos = [0.0, 0.0];
    let mut mouse_down = false;
    let mut held_point_index: Option<usize> = None;
    let mut current_softbody: Option<SoftBody> = None;

    // Two-way communication
    let (to_sim_tx, to_sim_rx): (Sender<SoftBody>, Receiver<SoftBody>) = mpsc::channel();
    let (from_sim_tx, from_sim_rx): (Sender<SoftBody>, Receiver<SoftBody>) = mpsc::channel();

    println!("Starting Soft Body Simulation...");

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Soft Body Simulation", window_size)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Spawn simulation thread
    thread::spawn(move || {
        let mut softbody = SoftBody::new_square([100.0, 50.0], 200.0, 6);
        loop {
            // Try to receive an updated softbody from UI
            while let Ok(updated) = to_sim_rx.try_recv() {
                softbody = updated;
            }

            softbody.update(&window_size);

            from_sim_tx.send(softbody.clone()).unwrap();
            thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
        }
    });

    while let Some(event) = window.next() {
        // Track mouse position
        if let Some(pos) = mouse::MouseCursorEvent::mouse_cursor_args(&event) {
            mouse_pos = pos;
        }

        // Mouse press: identify point to drag
        if let Some(piston::Button::Mouse(button)) = piston::PressEvent::press_args(&event) {
            if button == piston::MouseButton::Left {
                mouse_down = true;

                if let Some(sb) = &current_softbody {
                    held_point_index = sb.points.iter().position(|p| {
                        let dx = p.position[0] - mouse_pos[0];
                        let dy = p.position[1] - mouse_pos[1];
                        (dx * dx + dy * dy).sqrt() < 10.0
                    });
                }
            }
        }

        // Mouse release: stop dragging
        if let Some(piston::Button::Mouse(button)) = piston::ReleaseEvent::release_args(&event) {
            if button == piston::MouseButton::Left {
                mouse_down = false;
                held_point_index = None;
            }
        }

        // On render: drag logic + send updated softbody to simulation thread
        if let Some(_args) = event.render_args() {
            // Get latest softbody from simulation
            if let Some(mut sb) = from_sim_rx.try_iter().last() {
                // Dragging logic
                if mouse_down {
                    if let Some(index) = held_point_index {
                        if index < sb.points.len() {
                            sb.points[index].position = mouse_pos;
                        }
                    }
                }

                // Save locally and send to sim thread
                current_softbody = Some(sb.clone());
                to_sim_tx.send(sb.clone()).unwrap();

                // Draw
                window.draw_2d(&event, |c, g, _| {
                    piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                    render::render_softbody(c, g, &sb);
                });
            }
        }
    }
}