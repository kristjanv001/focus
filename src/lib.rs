use crossterm::{self, ExecutableCommand};
use std::io::Write;
use std::{self, io::Error, time::Instant};

pub struct Config {
  start_msg: String,
  quit_key: char,
}
impl Default for Config {
  fn default() -> Self {
    Self {
      start_msg: "🍅 focusing...".to_string(),
      quit_key: 'q',
    }
  }
}

struct FocusTimer;
impl FocusTimer {
  fn calc_duration(start_time: Instant) -> f32 {
    let duration = start_time.elapsed();
    return duration.as_secs_f32();
  }
}

struct FocusDisplay;
impl FocusDisplay {
  fn convert_time_to_msg(seconds: f32) -> String {
    let mut time_message = String::new();

    let total_seconds = seconds.floor() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    // let seconds_left = total_seconds % 60;

    if hours > 0 {
      time_message.push_str(&format!("{} hour", hours));
      if hours > 1 {
        time_message.push_str("s");
      }
    }

    if minutes > 0 {
      if hours > 0 {
        time_message.push_str(" ");
      }
      time_message.push_str(&format!("{} minute", minutes));
      if minutes > 1 {
        time_message.push_str("s");
      }
    }

    if hours == 0 && minutes == 0 {
      time_message.push_str("few seconds");
    }

    time_message
  }

  fn display_result_message(seconds: f32) -> Result<(), Error> {
    let time_message = Self::convert_time_to_msg(seconds);
    writeln!(std::io::stdout(), "⌛️ {}\r", time_message)?;

    Ok(())
  }
}

struct CleanUp;
impl Drop for CleanUp {
  fn drop(&mut self) {
    crossterm::terminal::disable_raw_mode().expect("Could not disable raw mode");
    std::io::stdout()
      .execute(crossterm::cursor::Show)
      .expect("Could not show cursor");
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let _cleanup = CleanUp;

  println!("{}", config.start_msg);

  crossterm::terminal::enable_raw_mode().map_err(|e| {
    eprintln!("Failed to enable raw mode: {}", e);
    e
  })?;

  std::io::stdout()
    .execute(crossterm::cursor::Hide)
    .expect("Could not hide cursor");

  let start_time = Instant::now();

  loop {
    match crossterm::event::read() {
      Ok(crossterm::event::Event::Key(event)) => {
        if event.code == crossterm::event::KeyCode::Char(config.quit_key) {
          break;
        }
      }
      Ok(_) => continue,
      Err(e) => {
        eprintln!("Error reading event: {}", e);
        break;
      }
    }
  }

  let elapsed_time = FocusTimer::calc_duration(start_time);

  if elapsed_time >= 5.0 {
    // @todo store data
    FocusDisplay::display_result_message(elapsed_time)?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::{Duration, Instant};

  #[test]
  fn test_45min() {
    let sim_start = Instant::now() - Duration::new(45 * 60, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "45 minutes");
  }

  #[test]
  fn test_6seconds() {
    let sim_start = Instant::now() - Duration::new(6, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "few seconds");
  }

  #[test]
  fn test_59seconds() {
    let sim_start = Instant::now() - Duration::new(59, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "few seconds");
  }

  #[test]
  fn test_60seconds() {
    let sim_start = Instant::now() - Duration::new(60, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "1 minute");
  }

  #[test]
  fn test_1min20s() {
    let sim_start = Instant::now() - Duration::new(60 + 20, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "1 minute");
  }

  #[test]
  fn test_1h12min() {
    let sim_start = Instant::now() - Duration::new(1 * 60 * 60 + 12 * 60, 0);
    let sim_duration = FocusTimer::calc_duration(sim_start);
    let formatted_time = FocusDisplay::convert_time_to_msg(sim_duration);
    assert_eq!(formatted_time, "1 hour 12 minutes");
  }
}

// if 59s -> nothing
// if 1min59s -> 1min

// Duration::new(60, 0); // 1min
// Duration::new(4 * 60, 0); // 4min
// Duration::new(10 * 60, 0); // 10min
// Duration::new(1 * 60 * 60, 0); // 1h
// Duration::new(5 * 60 * 60, 0); // 5h
// Duration::new(1 * 60 * 60 + 12 * 60, 0); // 1h 12min
// Duration::new(2 * 60 * 60 + 30 * 60, 0); // 2h 30min
