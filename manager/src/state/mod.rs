use std::sync::{atomic::AtomicU32, Arc};

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

use crate::server::{Server, ServerInfo};

// #[non_exhaustive]
// #[derive(Debug)]
// #[derive(Serialize, Deserialize, JsonSchema)]
// pub enum Resource {
//   Server,
//   User,
// }

// #[derive(Debug)]
// #[derive(Serialize, Deserialize, JsonSchema)]
// pub struct ID {
//   resource: Resource,
//   id: String,
// }

pub type ID = String;

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
  pub id: ID,
  pub username: String,
  pub name: String,
}

pub struct State {
  pub users: Arc<Mutex<Vec<User>>>,
  pub servers: Arc<Mutex<Vec<Server>>>,
  pub counter: AtomicU32,
}

impl State {
  pub fn new() -> Self {
    Self {
      users: Arc::new(Mutex::new(Vec::new())),
      servers: Arc::new(Mutex::new(Vec::new())),
      counter: AtomicU32::new(0),
    }
  }
}