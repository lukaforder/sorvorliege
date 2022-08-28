use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use strum_macros::EnumVariantNames;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("connection to server lost")]
    ConnectionLost,
    #[error("failed to connect to server")]
    ConnectionFailed,
    #[error("unknown communicator error")]
    Unknown,
  }
  
pub type Result<T> = std::result::Result<T, Error>;
pub type BoxedCommunicator = Box<dyn Communicator + Send + Sync>;

#[async_trait]
pub trait Communicator {
  async fn send_cmd(&mut self, cmd: String) -> Result<()>;
  async fn connect(&mut self) -> Result<()>;
  async fn disconnect(&mut self) -> Result<()>;

  fn name(&self) -> &'static str;
  fn id(&self) -> String;
}

/* TODO: find a better alternative to hardcoding CommunicatorType
  Ideally the enum could be generated at compile time by looking at all communicator structs.
  Might be doable with procedural macros but I haven't touched those at all.
 */

 
use super::Test;

#[derive(Debug)]
#[derive(Serialize, Deserialize, JsonSchema)]
#[derive(Clone, Copy)]
#[derive(EnumVariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum CommunicatorType {
  None,
  #[strum(serialize="Test")]
  Test
}

pub fn generate_communicator(comm_type: &CommunicatorType) -> Option<BoxedCommunicator> {
  match comm_type {
    CommunicatorType::None => {
      None
    },
    CommunicatorType::Test => {
     Some(Box::new(Test::new()))
    },
  }
}