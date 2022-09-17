use std::sync::{Arc, atomic::Ordering};

use log::{error, debug};
use manager_derive::command;
use uuid::{uuid, Uuid};

use crate::{state::State, messaging::CommandTX, send_command, commands::ServerCommands, server::{ServerInfo, CommunicatorStatus, Server}, communicator::CommunicatorType};

use super::ClientCommands;

#[command(tx)]
pub async fn handle_command(cmd: ClientCommands, state: Arc<State>) -> Vec<tungstenite::Message> {
  match cmd {
    ClientCommands::CreateServer => {
      let server = Server::new();
      send_command!(tx, &ServerCommands::ServerInfo(vec![server.info().clone()]));
      state.servers.lock().await.push(server);
    },
    ClientCommands::UpdateServer { id, name, communicator_type } => {
      let mut servers = state.servers.lock().await;
      if let Some(server) = servers.iter_mut().find(|s| s.id() == &id) {
        // FIXME: user association
        server.update(String::new(), name, communicator_type);
        send_command!(tx, &ServerCommands::ServerInfo(vec![server.info()]));
      }
    },
    ClientCommands::SetConnected { id, connected } => {
      let mut servers = state.servers.lock().await;
      debug!("Setting server {} to {}", id, connected);
      if let Some(server) = servers.iter_mut().find(|s| s.id() == &id) {
        debug!("found");
        let result = match connected {
          true => server.connect().await,
          false => server.disconnect().await,
        };
        if result.is_err() {
          error!("Failed to set connected: {:?}", result);
        } else {
          send_command!(tx, &ServerCommands::ServerInfo(vec![server.info()]));
        }
      }
    },
    ClientCommands::GetLogs { id, page } => {
      let mut servers = state.servers.lock().await;
      if let Some(server) = servers.iter_mut().find(|s| s.id() == &id) {
        let (page, messages) = server.get_logs(page);
        send_command!(tx, &ServerCommands::ServerLogs { page, messages: messages.to_vec() });
      }
    },
  };
}