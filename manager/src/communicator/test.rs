use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use super::Communicator;
use super::Result;

#[derive(Serialize)]
pub struct Test {
  id: Uuid,
}

impl Test {
  pub fn new() -> Self {
    Self {
      id: uuid::Uuid::new_v4(),
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
  fn id(&self) -> Uuid {
    self.id.clone()
  }
}