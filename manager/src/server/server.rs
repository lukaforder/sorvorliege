use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{communicator::{CommunicatorType, BoxedCommunicator, self}, state::ID};

use super::Message;


// FIXME: make this configurable?
pub const PAGE_SIZE: usize = 50; // server page size, in messages

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum CommunicatorStatus {
  Disconnected,
  Connecting,
  Connected,
  Missing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ServerInfo {
  pub id: ID,
  pub name: String,
  pub comm_status: CommunicatorStatus,
  pub comm_type: CommunicatorType,
}

pub struct Server {
  id: ID,
  info: ServerInfo,
  communicator: Option<BoxedCommunicator>,
  messages: Vec<Message>,
}

impl Server {
  pub async fn connect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
  pub async fn disconnect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
}