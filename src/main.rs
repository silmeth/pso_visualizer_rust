extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

fn main() {
    let _window: PistonWindow = WindowSettings::new("Test", [300; 2])
        .build()
        .unwrap();
}
