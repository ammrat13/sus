use super::VerifyResult;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn from_sudoers() -> VerifyResult {
  let file = File::open("sudoers.json")?;
  let reader = BufReader::new(file);
  let u = serde_json::from_reader(reader)?;
  
}