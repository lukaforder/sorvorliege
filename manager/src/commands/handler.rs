use std::sync::{Arc, atomic::Ordering};

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
        server.update(name, communicator_type);
        send_command!(tx, &ServerCommands::ServerInfo(vec![server.info().clone()]));
      }
    },
    ClientCommands::Increment(amount) => {
      state.counter.fetch_add(amount, Ordering::SeqCst);
      send_command!(tx, &ServerCommands::Counter(state.counter.load(Ordering::SeqCst)));
    },
  };
}