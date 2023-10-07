use std::{env::args, process};

use rustaska::{run, Config};

const USAGE: &str = "\
You need to pass four filepaths, in this order:
  <source path.lst>, <numbers path.csv>, <strings path.csv>, <errors path.log>";

fn main() {
    let cfg = match Config::build(args()) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("{USAGE}");
            process::exit(1)
        }
    };

    if let Err(e) = run(cfg) {
        eprintln!("{e}");
        process::exit(1);
    }
}
