use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::communicator::CommunicatorType;

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
}

#[non_exhaustive]
#[derive(Debug)]
#[derive(Serialize, JsonSchema)]
#[serde(tag="type", content="body")]
pub enum ServerCommands {
  Identity,
}