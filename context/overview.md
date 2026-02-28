# Capture

Security tool that catches people accessing your computer while you're away.

## How It Works

- Grabs all input devices (keyboard, mouse) — they stop reaching the desktop
- Polls all devices with `nix::poll` — blocks until any event occurs
- Snaps a webcam photo on input, saves timestamped to `captures/`
- Typing a secret key ungrabs everything and exits

## Current State

- Device enumeration and poll loop working
- Camera init wired up (nokhwa)
- TODO: grab/ungrab, secret key detection, photo triggering, debounce

## Cross-Platform

Planning to pivot crates to support both Linux and macOS.

## Crates

`evdev`, `nix` (poll), `nokhwa`, `anyhow`, `image`

## Permissions

Requires root or `input` group for `/dev/input/*`.
