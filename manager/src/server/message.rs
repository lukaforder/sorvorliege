use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc};

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
#[derive(Clone)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
  In,
  Out
}

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
#[derive(Clone)]
pub struct Message {
  pub timestamp: i64,
  pub user: String,
  pub body: String,
  pub msg_type: MessageType,
}

impl Message {
  pub fn new(user: String, body: String, msg_type: MessageType) -> Message {
    Message {
      timestamp: Utc::now().timestamp(),
      user,
      body,
      msg_type,
    }
  }
}