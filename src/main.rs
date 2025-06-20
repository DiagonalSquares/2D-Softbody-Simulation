use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

mod render;
mod ui;
mod input;
mod simulation;

fn main() {
    render::render();
    ui::render();
}