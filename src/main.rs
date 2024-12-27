use focus;
use std::process;

fn main() {
  let config = focus::Config::default();
  if let Err(e) = focus::run(config) {
    println!("App error: {e}");
    process::exit(1);
  }
}
