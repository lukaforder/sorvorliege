use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{communicator::{CommunicatorType, BoxedCommunicator, self}, state::ID};

use super::Message;


// FIXME: make this configurable?
pub const PAGE_SIZE: usize = 50; // server page size, in messages

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
#[derive(Clone)]
pub enum CommunicatorStatus {
  Disconnected,
  Connecting,
  Connected,
  Missing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
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
  pub fn new() -> Self {
    let id = Uuid::new_v4().to_string();
    let info = ServerInfo {
      id: id.clone(),
      name: "new server".to_string(),
      comm_status: CommunicatorStatus::Disconnected,
      comm_type: CommunicatorType::None,
    };
    Self {
      id,
      info,
      communicator: None,
      messages: Vec::new(),
    }
  }


  pub fn id(&self) -> &ID {
    &self.id
  }

  pub fn info(&self) -> &ServerInfo {
    &self.info
  }

  pub fn update(&mut self, name: Option<String>, comm_type: Option<CommunicatorType>) {
    if let Some(name) = name{
      self.info.name = name;
    }
    self.info.comm_type = comm_type.unwrap_or(self.info.comm_type);
  }

  pub async fn connect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
  pub async fn disconnect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
}