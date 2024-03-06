use std::fs::OpenOptions;
use std::io::Write;

pub fn registry(tag: &str, username: &str, password: &str, description: &str) -> std::io::Result<()> {
  let mut file = OpenOptions::new()
  .create(true)
  .append(true)
  .open(".data.txt")?;

  let data = format!("Tag => {} | Username => {} | Password => {} | Description=> {}\n", tag.trim(), username.trim(), password.trim(), description.trim());
  file.write_all(data.as_bytes())?;

  Ok(())
}
