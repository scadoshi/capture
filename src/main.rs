mod camera;

use rdev::{Event, EventType, Key, grab};
use std::fs::create_dir_all;

use crate::camera::Camera;

const OUTPUT_DIR: &str = "captures";

fn run() -> anyhow::Result<()> {
    let mut camera = Camera::new()?;
    create_dir_all(OUTPUT_DIR)?;

    let callback = |event: Event| match event.event_type {
        EventType::KeyPress(Key::Escape) => {
            println!("Releasing");
            std::process::exit(0);
        }
        _ => {
            println!("Gotcha!");
            None
        }
    };

    camera
        .shoot()?
        .save(format!("{}/capture1.png", OUTPUT_DIR))?;

    if let Err(e) = grab(callback) {
        eprintln!("Grab failed: {:?}", e);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}
