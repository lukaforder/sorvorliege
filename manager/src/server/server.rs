use std::{sync::Arc, fmt::Write};

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{communicator::{CommunicatorType, BoxedCommunicator, self}, state::ID};

use super::{Message, MessageType};


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

  pub fn get_logs(&self, page: Option<usize>) -> (usize, &[Message]) {
    let page = page.unwrap_or(self.messages.len() / PAGE_SIZE);
    let from = page * PAGE_SIZE.clamp(0, self.messages.len());
    let to = (from + PAGE_SIZE).clamp(0, self.messages.len());
    (page, &self.messages[from..to])
  }

  pub fn update(&mut self, user: String, name: Option<String>, comm_type: Option<CommunicatorType>) {
    let mut msg = "changed ".to_string();
    if let Some(name) = name{
      msg.write_fmt(format_args!("name to '{}' and ", &name)).unwrap();
      self.info.name = name;
    }
    if let Some(comm_type) = comm_type {
      self.info.comm_type = comm_type;
      msg.write_fmt(format_args!("communicator type to '{:?}' and ", comm_type)).unwrap();
    }
    if msg.len() > 8 {
      self.messages.push(Message::new(
        user,
        msg[..msg.len() - 5].to_string(),
        MessageType::Out
      ));
    }
  }

  pub async fn connect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
  pub async fn disconnect(&mut self) -> communicator::Result<()> {
    Ok(())
  }
}