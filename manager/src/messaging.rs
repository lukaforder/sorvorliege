use std::{net::SocketAddr, sync::Arc};

use futures::{pin_mut, StreamExt, TryStreamExt};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use log::{info, error};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_rustls::server::TlsStream;
use tokio_tungstenite::{accept_async, WebSocketStream};

use crate::{commands::{self, ClientCommands, handler}, state::State};

pub type CommandTX = Mutex<UnboundedSender<tungstenite::Message>>;

#[macro_export]
macro_rules! send_command {
  ($tx:expr, $cmd:expr) => {
    $tx.push(tungstenite::Message::from(crate::messaging::encode_cmd($cmd)));
  };
}

pub fn encode_cmd(cmd: &commands::ServerCommands) -> Vec<u8> {
  Vec::from(
    base64::encode(
      &serde_json::to_string(cmd).unwrap()
    ).as_bytes()
  )
}

fn decode_cmd<S: AsRef<str>>(txt: S) -> Option<ClientCommands> {
  let txt = txt.as_ref();
  let txt = base64::decode(txt).unwrap();
  let txt = String::from_utf8(txt).unwrap();
  serde_json::from_str(&txt).ok()
} 

async fn process_message(msg: tungstenite::Message, state: Arc<State>, tx: Arc<CommandTX>) -> tungstenite::Result<()> {
  match msg {
    tungstenite::Message::Text(text) => {
      let res = match decode_cmd(&text) {
        Some(cmd) => handler::handle_command(cmd, state).await,
        _ => {error!("Could not decode message: {}", &text); Vec::new()},
      };
      {
        let tx = tx.lock().await; 
        for msg in res {
          tx.unbounded_send(msg);
        }
      }
    },
    _ => {},
}
  Ok(())
}

pub async fn handle_connection(peer: SocketAddr, stream: TlsStream<TcpStream>, state: Arc<State>) -> tungstenite::Result<()> {
  let ws_stream = accept_async(stream).await.expect("Failed to accept");
  info!("New connection from {}", &peer);

  let (ws_out, ws_in) = ws_stream.split();
  // channel to receive messages we want to send to the client
  let (tx, rx) = unbounded();

  // forward all messages from channel to the client
  let pipe_to_client = rx.map(Ok).forward(ws_out);
  pin_mut!(pipe_to_client);

  let tx = Arc::new(Mutex::new(tx));
  // process all incoming messages from the client
  let process_incoming = ws_in.try_for_each(|msg| process_message(msg, state.clone(), tx.clone()));

  // poll and wait for both simultaneously
  let (res1, res2) = futures::join!(pipe_to_client, process_incoming);
  res1.unwrap();
  res2.unwrap();
  // FIXME: clean this up

  info!("{} disconnected", &peer);
  Ok(())
}