mod camera;
mod capture_state;
mod run;

fn main() {
    if let Err(e) = run::run() {
        eprintln!("{e}");
    }
}
