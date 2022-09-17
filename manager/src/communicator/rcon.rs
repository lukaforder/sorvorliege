use async_trait::async_trait;
use serde::Serialize;
use tokio::net::TcpStream;
use uuid::Uuid;

use rcon::{Connection};

use super::Communicator;
use super::Error;
use super::Result;

#[derive(Serialize)]
pub struct Rcon {
  id: Uuid,

  #[serde(skip)]
  conn: Option<Connection<TcpStream>>,
}

impl Rcon {
  pub fn new() -> Self {
    Self {
      id: uuid::Uuid::new_v4(),
      conn: None,
    }
  }
}

#[async_trait]
impl Communicator for Rcon {
  async fn send_cmd(&mut self, cmd: String) -> Result<()> {
    Ok(())
  }
  async fn connect(&mut self) -> Result<()> {
    let conn = Connection::builder()
      .connect("gs.alanp.me:27015", "test")
      .await.map_err(|_| Error::ConnectionFailed)?;
    self.conn = Some(conn);
    Ok(())
  }
  async fn disconnect(&mut self) -> Result<()> {
    Ok(())
  }
  fn name(&self) -> &'static str {
    "Rcon"
  }
  fn id(&self) -> Uuid {
    self.id.clone()
  }
}