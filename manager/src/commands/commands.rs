use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{communicator::CommunicatorType, server::ServerInfo};

#[non_exhaustive]
#[derive(Debug)]
#[derive(Deserialize, JsonSchema)]
#[serde(tag="type", content="body")]
pub enum ClientCommands {
  CreateServer,
  UpdateServer {
    id: String,
    name: Option<String>,
    communicator_type: Option<CommunicatorType>,
  },
  /// Request to increment the server's counter by a certain amount.
  Increment(u32),
}

#[non_exhaustive]
#[derive(Debug)]
#[derive(Serialize, JsonSchema)]
#[serde(tag="type", content="body")]
pub enum ServerCommands {
  Identity,
  ServerInfo(ServerInfo),
  /// Tell the client the value of the test counter.
  Counter(u32),
}