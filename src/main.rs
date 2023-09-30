use std::process;

use rustaska::program;

fn main() {
    if let Err(e) = program() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
