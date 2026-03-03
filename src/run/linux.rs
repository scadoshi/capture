use crate::capture_state::CaptureState;
use anyhow::anyhow;
use evdev::{Device, EventSummary, EventType, InputEvent, KeyCode, RelativeAxisCode};
use nix::poll::{PollFd, PollFlags, PollTimeout};
use std::{os::fd::AsFd, rc::Rc, sync::Mutex};

trait Identify {
    fn is_probably_keyboard(&self) -> bool;
    fn is_probably_mouse(&self) -> bool;
}

impl Identify for Device {
    fn is_probably_keyboard(&self) -> bool {
        self.supported_events().contains(EventType::REPEAT)
            && self.supported_keys().is_some_and(|keys| {
                keys.contains(KeyCode::KEY_A)
                    && keys.contains(KeyCode::KEY_ENTER)
                    && keys.contains(KeyCode::KEY_SPACE)
            })
    }
    fn is_probably_mouse(&self) -> bool {
        self.supported_relative_axes().is_some_and(|axes| {
            axes.contains(RelativeAxisCode::REL_X) && axes.contains(RelativeAxisCode::REL_Y)
        })
    }
}

trait IsSecret {
    fn is_secret(&self) -> bool;
}

impl IsSecret for InputEvent {
    fn is_secret(&self) -> bool {
        matches!(
            self.destructure(),
            EventSummary::Key(_, KeyCode::KEY_ESC, 1)
        )
    }
}

#[cfg(target_os = "linux")]
pub fn run() -> anyhow::Result<()> {
    let state = Rc::new(Mutex::new(CaptureState::new()?));
    println!("CaptureState initialized");

    let mut devices = evdev::enumerate()
        .map(|(_, d)| d)
        .filter(|d| d.is_probably_mouse() || d.is_probably_keyboard())
        .collect::<Vec<Device>>();
    println!("Mouse and keyboard devices identified");

    for device in devices.iter_mut() {
        device.grab()?;
    }
    println!("Devices grabbed");

    let mut should_break = false;
    loop {
        let mut poll_fds: Vec<PollFd> = devices
            .iter()
            .map(|d| PollFd::new(d.as_fd(), PollFlags::POLLIN))
            .collect();
        nix::poll::poll(&mut poll_fds, PollTimeout::NONE)?;

        let ready: Vec<usize> = poll_fds
            .iter()
            .enumerate()
            .filter(|(_, pfd)| {
                pfd.revents()
                    .is_some_and(|flags| flags.contains(PollFlags::POLLIN))
            })
            .map(|(i, _)| i)
            .collect();
        drop(poll_fds);

        'device_loop: for i in ready {
            for event in devices[i].fetch_events()? {
                if event.is_secret() {
                    should_break = true;
                    println!("Secret key pressed");
                    println!("Exiting process");
                    break 'device_loop;
                } else if let Err(e) = state
                    .lock()
                    .map_err(|e| anyhow!("{}", e))
                    .and_then(|mut s| s.maybe_shoot_and_save().map_err(|e| anyhow!("{}", e)))
                {
                    eprintln!("Failed to shoot: {:?}", e);
                }
            }
        }

        if should_break {
            for device in devices.iter_mut() {
                device.ungrab()?;
            }
            break;
        }
    }

    Ok(())
}
