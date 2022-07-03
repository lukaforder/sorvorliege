use async_trait::async_trait;
use serde::Serialize;

use super::Communicator;
use super::Result;

#[derive(Serialize)]
pub struct Test {
}

impl Test {
  pub fn new() -> Self {
    Self {

    }
  }
}

#[async_trait]
impl Communicator for Test {
  async fn send_cmd(&mut self, cmd: String) -> Result<()> {
    Ok(())
  }
  async fn connect(&mut self) -> Result<()> {
    Ok(())
  }
  async fn disconnect(&mut self) -> Result<()> {
    Ok(())
  }
  fn name(&self) -> &'static str {
    "Test"
  }
}