use std::time::Instant;
use humantime::format_duration;

pub fn start() -> Instant {
  Instant::now()
}

pub fn end(now: Instant) -> String {
  // let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
  let elapsed = now.elapsed();
  format_duration(elapsed).to_string()
}
