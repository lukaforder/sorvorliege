use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::communicator::CommunicatorType;

#[non_exhaustive]
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(tag="type", content="body")]
pub enum Incoming {
  CreateServer,
  UpdateServer {
    id: Uuid,
    name: Option<String>,
    communicator_type: Option<CommunicatorType>,
  },
}

#[non_exhaustive]
#[derive(Debug)]
#[derive(Serialize)]
#[serde(tag="type", content="body")]
pub enum Outgoing {

}