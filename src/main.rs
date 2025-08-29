use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Instant;
use piston::{mouse, Button};
use piston_window::{RenderEvent, TextureSettings};

mod render;
mod ui;
mod input;
mod simulation;
mod app;

use simulation::{SoftBodyCollection, SoftBody};

fn main() {
    let mut input_handler = input::Input_Handler::new();

    let window_size = [800.0, 600.0];

    let mut frame_count = 0;
    let mut last_fps_check = Instant::now();

    // Two-way communication
    let (to_sim_tx, to_sim_rx): (Sender<SoftBodyCollection>, Receiver<SoftBodyCollection>) = mpsc::channel();
    let (from_sim_tx, from_sim_rx): (Sender<SoftBodyCollection>, Receiver<SoftBodyCollection>) = mpsc::channel();
    let (to_sim_pause_tx, to_sim_pause_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (from_sim_pause_tx, from_sim_pause_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let mut pause = false;

    println!("Starting Soft Body Simulation...");

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Soft Body Simulation", window_size)
            .exit_on_esc(true)
            .build()
            .unwrap();
    
    let spawn_button = ui::Button::new(
        [50.0, 50.0],
        [100.0, 50.0],
        [0.2, 0.6, 0.8, 1.0],
        "Spawn".to_string()
    );

    let pause_button = ui::Button::new(
        [450.0, 50.0],
        [100.0, 50.0],
        [0.5, 0.5, 0.2, 1.0],
        "Pause".to_string()
    );

    let mut glyphs = piston_window::Glyphs::new(
        "src/resources/FiraSans-Regular.ttf",
        window.create_texture_context(),
        TextureSettings::new()
    ).unwrap();

    // Spawn simulation thread
    thread::spawn(move || {
        let mut softbodycollection = SoftBodyCollection::new();
        softbodycollection.add(SoftBody::new_square([100.0, 300.0], 200.0, 3));
        softbodycollection.add(SoftBody::new_square([0.0, 100.0], 150.0, 6));
        softbodycollection.add(SoftBody::new_square([0.0, 0.0], 100.0, 5));
        loop {
            if let Ok(p) = to_sim_pause_rx.try_recv() {
                pause = p;
            }
            // Try to receive an updated softbody from UI
            while let Ok(updated) = to_sim_rx.try_recv() {
                softbodycollection = updated;
            }

            if !pause {
                softbodycollection.update(&window_size);

                from_sim_tx.send(softbodycollection.clone()).unwrap();
                thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
            }

            from_sim_pause_tx.send(pause).unwrap();
        }
    });

    while let Some(event) = window.next() {
        // Track mouse position
        if let Some(pos) = mouse::MouseCursorEvent::mouse_cursor_args(&event) {
            input_handler.handle_mouse_move(pos);
        }

        // Mouse press: find the closest point in any softbody
        if let Some(Button::Mouse(button)) = piston::PressEvent::press_args(&event) {
            if button == piston::MouseButton::Left {
                if let Some(softbodies) = from_sim_rx.try_iter().last() {
                    input_handler.handle_mouse_down(softbodies.clone());

                    //spawn a softbody if the spawn button is clicked
                    spawn_button.handle_click_spawn(input_handler.mouse_pos, softbodies.clone(), &to_sim_tx);
                }
                pause = pause_button.handle_click_pause(input_handler.mouse_pos, pause);
                to_sim_pause_tx.send(pause).unwrap();
            }
        }

        // Mouse release: stop dragging
        if let Some(Button::Mouse(button)) = piston::ReleaseEvent::release_args(&event) {
            if button == piston::MouseButton::Left {
                input_handler.handle_mouse_up();
            }
        }

        // On render: drag logic + send updated softbody to simulation thread
        if let Some(_args) = event.render_args() {
            if let Some(p) = from_sim_pause_rx.try_iter().last() {
                pause = p;
            }

            if let Some(mut softbodies) = from_sim_rx.try_iter().last() {
                // Dragging logic
                if input_handler.mouse_down {
                    if let (Some(sb_idx), Some(pt_idx)) = (input_handler.softbody_index, input_handler.held_point_index) {
                        if sb_idx < softbodies.softbodies.len() && pt_idx < softbodies.softbodies[sb_idx].points.len() {
                            softbodies.softbodies[sb_idx].points[pt_idx].position = input_handler.mouse_pos;
                        }
                    }
                }

                // Send the updated softbodies to the simulation thread
                to_sim_tx.send(softbodies.clone()).unwrap();

                // Draw
                window.draw_2d(&event, |c, g, device| {
                    piston_window::clear([0.1, 0.1, 0.3, 1.0], g);
                    render::render_all_softbodies(c, g, &softbodies.softbodies);
                    spawn_button.render(c, g, &mut glyphs);
                    pause_button.render(c, g, &mut glyphs);

                    glyphs.factory.encoder.flush(device);
                });

                frame_count += 1;
                if last_fps_check.elapsed().as_secs() >= 1 {
                    let fps = frame_count as f64 / last_fps_check.elapsed().as_secs_f64();
                    println!("FPS: {:.2}", fps);
                    frame_count = 0;
                    last_fps_check = Instant::now();
                }
            }
        }
    }
}