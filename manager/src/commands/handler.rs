use std::sync::{Arc, atomic::Ordering};

use manager_derive::command;

use crate::{state::State, messaging::CommandTX, send_command, commands::ServerCommands};

use super::ClientCommands;

#[command(tx)]
pub async fn handle_command(cmd: ClientCommands, state: Arc<State>) -> Vec<tungstenite::Message> {
  match cmd {
    ClientCommands::CreateServer => todo!(),
    ClientCommands::UpdateServer { id, name, communicator_type } => todo!(),
    ClientCommands::Increment(amount) => {
      state.counter.fetch_add(amount, Ordering::SeqCst);
      send_command!(tx, &ServerCommands::Counter(state.counter.load(Ordering::SeqCst)));
    },
  };
}