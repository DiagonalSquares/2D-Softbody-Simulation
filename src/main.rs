use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use piston::{mouse};
use piston_window::{RenderEvent};

mod render;
mod ui;
mod input;
mod simulation;

use simulation::{SoftBodyCollection, SoftBody};

fn main() {
    let window_size = [800.0, 600.0];
    let mut mouse_pos = [0.0, 0.0];
    let mut mouse_down = false;
    let mut held_point_index: Option<usize> = None;
    let mut softbody_index: Option<usize> = None;

    // Two-way communication
    let (to_sim_tx, to_sim_rx): (Sender<SoftBodyCollection>, Receiver<SoftBodyCollection>) = mpsc::channel();
    let (from_sim_tx, from_sim_rx): (Sender<SoftBodyCollection>, Receiver<SoftBodyCollection>) = mpsc::channel();

    println!("Starting Soft Body Simulation...");

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Soft Body Simulation", window_size)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Spawn simulation thread
    thread::spawn(move || {
        let mut softbodycollection = SoftBodyCollection::new();
        softbodycollection.add(SoftBody::new_square([100.0, 300.0], 200.0, 3));
        softbodycollection.add(SoftBody::new_square([0.0, 100.0], 150.0, 6));
        softbodycollection.add(SoftBody::new_square([0.0, 0.0], 100.0, 5));
        loop {
            // Try to receive an updated softbody from UI
            while let Ok(updated) = to_sim_rx.try_recv() {
                softbodycollection = updated;
            }

            softbodycollection.update(&window_size);

            from_sim_tx.send(softbodycollection.clone()).unwrap();
            thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
        }
    });

    while let Some(event) = window.next() {
        // Track mouse position
        if let Some(pos) = mouse::MouseCursorEvent::mouse_cursor_args(&event) {
            mouse_pos = pos;
        }

        // Mouse press: find the closest point in any softbody
        if let Some(piston::Button::Mouse(button)) = piston::PressEvent::press_args(&event) {
            if button == piston::MouseButton::Left {
                mouse_down = true;
                // Always use the latest state for picking
                if let Some(softbodies) = from_sim_rx.try_iter().last() {
                    let mut found = false;
                    for (sbi, sb) in softbodies.softbodies.iter().enumerate() {
                        if let Some(pi) = sb.points.iter().position(|p| {
                            let dx = p.position[0] - mouse_pos[0];
                            let dy = p.position[1] - mouse_pos[1];
                            (dx * dx + dy * dy).sqrt() < 10.0
                        }) {
                            softbody_index = Some(sbi);
                            held_point_index = Some(pi);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        softbody_index = None;
                        held_point_index = None;
                    }
                }
            }
        }

        // Mouse release: stop dragging
        if let Some(piston::Button::Mouse(button)) = piston::ReleaseEvent::release_args(&event) {
            if button == piston::MouseButton::Left {
                mouse_down = false;
                held_point_index = None;
                softbody_index = None;
            }
        }

        // On render: drag logic + send updated softbody to simulation thread
        if let Some(_args) = event.render_args() {
            if let Some(mut softbodies) = from_sim_rx.try_iter().last() {
                // Dragging logic
                if mouse_down {
                    if let (Some(sb_idx), Some(pt_idx)) = (softbody_index, held_point_index) {
                        if sb_idx < softbodies.softbodies.len() && pt_idx < softbodies.softbodies[sb_idx].points.len() {
                            softbodies.softbodies[sb_idx].points[pt_idx].position = mouse_pos;
                        }
                    }
                }
                to_sim_tx.send(softbodies.clone()).unwrap();

                // Draw
                window.draw_2d(&event, |c, g, _| {
                    piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                    render::render_all_softbodies(c, g, &softbodies.softbodies);
                });
            }
        }
    }
}