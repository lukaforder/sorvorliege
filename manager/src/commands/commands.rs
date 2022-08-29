use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{communicator::CommunicatorType, server::{ServerInfo, Message}};

#[non_exhaustive]
#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag="type", content="body")]
pub enum ClientCommands {
  /// Create a new server.
  CreateServer,
  /// Update server information.
  UpdateServer {
    id: Uuid,
    name: Option<String>,
    communicator_type: Option<CommunicatorType>,
  },
  /// Request server logs.
  GetLogs {
    id: Uuid,
    /// If not specified, will return the most recent page.
    page: Option<usize>,
  },
}

#[non_exhaustive]
#[derive(Debug)]
#[derive(Serialize, JsonSchema)]
#[serde(tag="type", content="body")]
pub enum ServerCommands {
  Identity,
  ServerInfo(Vec<ServerInfo>),
  ServerLogs{page: usize, messages: Vec<Message>},
}