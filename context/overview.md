# Capture

Security tool that catches people accessing your computer while you're away.

## How It Works

- Grabs all input devices (keyboard, mouse) — they stop reaching the desktop
- Snaps a webcam photo on input, saves timestamped to `captures/`
- Typing a secret key ungrabs everything and exits

## Current State

- Camera module working (nokhwa) with warmup frames
- Photo capture and save working
- Grab via `rdev` `unstable_grab` feature wired up
- TODO: secret key detection, debounce, selective device grabbing

## Cross-Platform

- Compiled binary is always native to one platform
- Rust `#[cfg(target_os = "...")]` for conditional compilation
- CI pipeline needed for both Linux and macOS builds
- Shared code: camera capture, photo saving, secret key matching, debounce, CLI
- Platform-specific: grab/ungrab and raw input device access

## Crates

- `rdev` (with `unstable_grab` feature) — cross-platform input grab and detection
- `nokhwa` — webcam capture
- `image` — saving photos
- `anyhow` — error handling

For macOS, `rdev` uses Accessibility API / CGEventTap under the hood.
For Linux, `rdev` uses evdev/uinput under the hood.

## Grab Architecture

- `rdev::grab` takes a `FnOnce` callback, called on every input event
- Return `None` to swallow the event (block from reaching desktop)
- Return `Some(event)` to let it pass through
- Callback needs interior mutability (`Arc<Mutex<_>>` or atomics) for mutable state
- `grab` is a blocking call — it IS the main loop
- Use `std::process::exit(0)` to break out (no clean stop API)

## Known Issues

- `rdev` grab on Linux grabs ALL evdev devices including Bluetooth/network controllers
- When grab fails or crashes, devices may not release cleanly (kills Bluetooth, network)
- May need to fall back to `evdev` directly on Linux for selective device grabbing
- `uinput` kernel module must be loaded (`sudo modprobe uinput`)
- `/dev/uinput` needs `input` group permissions (`sudo chown root:input`, `chmod 660`)
- udev rule for persistence: `KERNEL=="uinput", GROUP="input", MODE="0660"` in `/etc/udev/rules.d/99-uinput.rules`
- `rdev` listen (not grab) does NOT work on Wayland — but grab does (uses evdev directly)

## Permissions

- Linux: user must be in `input` group for `/dev/input/*` and `/dev/uinput`
- macOS: app needs Accessibility API access (System Preferences > Security & Privacy > Privacy > Accessibility)
