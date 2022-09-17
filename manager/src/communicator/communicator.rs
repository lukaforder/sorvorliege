use std::collections::BTreeMap;

use async_trait::async_trait;
use schemars::{JsonSchema, schema::{SchemaObject, InstanceType}};
use serde::{Serialize, Deserialize};
use serde_json::json;
use strum::VariantNames;
use strum_macros::EnumVariantNames;
use thiserror::Error;
use uuid::Uuid;


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
  fn id(&self) -> Uuid;
}

/* TODO: find a better alternative to hardcoding CommunicatorType
  Ideally the enum could be generated at compile time by looking at all communicator structs.
  Might be doable with procedural macros but I haven't touched those at all.
 */

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy)]
#[derive(EnumVariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum CommunicatorType {
  None,
  #[strum(serialize="Rcon")]
  Rcon,
}

impl JsonSchema for CommunicatorType {
  fn schema_name() -> String {
    "CommunicatorType".to_string()
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut extensions = BTreeMap::new();
    extensions.insert("tsEnumNames".to_string(), json!(CommunicatorType::VARIANTS));
    SchemaObject {
      instance_type: Some(InstanceType::String.into()),
      enum_values: Some(CommunicatorType::VARIANTS.to_vec().iter().map(|v| json!(v)).collect()),
      extensions,
      ..Default::default()
    }.into()
  }
}

pub fn generate_communicator(comm_type: &CommunicatorType) -> Option<BoxedCommunicator> {
  match comm_type {
    CommunicatorType::None => {
      None
    },
    CommunicatorType::Rcon => {
     Some(Box::new(super::Rcon::new()))
    },
  }
}