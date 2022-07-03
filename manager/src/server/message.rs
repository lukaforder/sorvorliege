use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum MessageType {
  IN,
  OUT
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Message {
  pub timestamp: i64,
  pub user: Uuid,
  pub body: String,
  pub msg_type: MessageType,
}

impl Message {
  pub fn new(user: Uuid, body: String, msg_type: MessageType) -> Message {
    Message {
      timestamp: Utc::now().timestamp(),
      user,
      body,
      msg_type,
    }
  }
}