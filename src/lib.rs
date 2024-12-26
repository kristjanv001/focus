use crossterm::event;
use crossterm::terminal;
use std::error;

// pub struct Config {}
struct CleanUp;

impl Drop for CleanUp {
  fn drop(&mut self) {
    terminal::disable_raw_mode().expect("Could not disable raw mode");
  }
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
  let _cleanup = CleanUp;

  println!("focusing...");
  // io::stdout().flush()?;

  terminal::enable_raw_mode().map_err(|e| {
    eprintln!("Failed to enable raw mode: {}", e);
    e
  })?;

  loop {
    if let event::Event::Key(event) = event::read().unwrap() {
      if event.code == event::KeyCode::Char('q') {
        break;
      }
    }
  }

  terminal::disable_raw_mode().map_err(|e| {
    eprintln!("Failed to disable raw mode: {}", e);
    e
  })?;

  println!("Nice Work! You worked for x min.");

  Ok(())
}

#[cfg(test)]
mod tests {
  // use super::*;
  // #[test]
  // fn mytest() {}
}
