use evdev::Device;
use nix::poll::{PollFd, PollFlags, PollTimeout};
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};
use std::{fs::create_dir_all, os::fd::AsFd};

const OUT_DIR: &str = "captures";

fn run() -> anyhow::Result<()> {
    // init camera
    let mut camera = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution),
    )?;
    camera.open_stream()?;
    for _ in 0..30 {
        camera.frame()?;
    }

    // ensure file exists
    create_dir_all(OUT_DIR)?;

    let mut poll_fds: Vec<PollFd> = Vec::new();
    let devices: Vec<Device> = evdev::enumerate().map(|(_, device)| device).collect();
    for device in devices.iter() {
        poll_fds.push(PollFd::new(device.as_fd(), PollFlags::POLLIN));
    }

    loop {
        nix::poll::poll(&mut poll_fds, PollTimeout::NONE)?;
    }

    let image = camera.frame()?.decode_image::<RgbFormat>()?;
    image.save(format!("{}/capture1.png", OUT_DIR))?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}
