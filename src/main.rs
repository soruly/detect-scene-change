use detect_scene_change::Config;
use std::env;
use std::process;

// use std::fs::File;
// use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(e) = detect_scene_change::run(config) {
    eprintln!("Application error: {}", e);

    process::exit(1);
  }
}
