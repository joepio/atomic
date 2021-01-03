/// Returns the current timestamp
pub fn now() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("You're a time traveler")
    .as_millis() as u64
}
