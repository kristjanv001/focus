use focus;
use std::process;

fn main() {
  if let Err(e) = focus::run() {
    println!("App error: {e}");
    process::exit(1);
  }
}
