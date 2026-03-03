mod camera;
mod capture_state;
mod run;

#[cfg(target_os = "linux")]
use run::linux::run;

#[cfg(target_os = "macos")]
use run::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}
