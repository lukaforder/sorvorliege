use std::{path::Path, io};

use log::info;
use schemars::{schema_for, schema::RootSchema};

fn write_schema(dir: &Path, name: &str, schema: &RootSchema) -> io::Result<()> {
  let output = serde_json::to_string_pretty(schema).unwrap();
  let path = dir.join(format!("{}.json", name));
  std::fs::write(path, output)
}

pub fn generate_schemas() -> std::io::Result<()> {
  let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("schemas");
  let e = std::fs::DirBuilder::new().create(&dir);
  if let Err(e) = e {
    if e.kind() != std::io::ErrorKind::AlreadyExists {
      return Err(e.into());
    }
  }

  
  let schema = schema_for!(super::state::User);
  write_schema(&dir, "user", &schema)?;

  let schema = schema_for!(super::commands::ClientCommands);
  write_schema(&dir, "client_commands", &schema)?;
  
  let schema = schema_for!(super::commands::ServerCommands);
  write_schema(&dir, "server_commands", &schema)?;

  info!("Generated schemas in: {}", dir.to_string_lossy());

  Ok(())
}