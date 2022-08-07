use std::net::SocketAddr;

use futures::{pin_mut, StreamExt, TryStreamExt};
use futures_channel::mpsc::unbounded;
use log::info;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio_tungstenite::{accept_async, WebSocketStream};

use crate::commands;

fn encode_cmd(cmd: &commands::ServerCommands) -> Vec<u8> {
  Vec::from(
    base64::encode(
      urlencoding::encode(
        &serde_json::to_string(cmd).unwrap()
      ).as_bytes()
    )
  )
}

async fn process_message(msg: tungstenite::Message) -> tungstenite::Result<()> {

  Ok(())
}

pub async fn handle_connection(peer: SocketAddr, stream: TlsStream<TcpStream>) -> tungstenite::Result<()> {
  let ws_stream = accept_async(stream).await.expect("Failed to accept");
  info!("New connection from {}", &peer);

  let (ws_out, ws_in) = ws_stream.split();
  // channel to receive messages we want to send to the client
  let (tx, rx) = unbounded();

  // forward all messages from channel to the client
  let pipe_to_client = rx.map(Ok).forward(ws_out);
  pin_mut!(pipe_to_client);

  // process all incoming messages from the client
  let process_incoming = ws_in.try_for_each(process_message);

  // poll and wait for both simultaneously
  let (res1, res2) = futures::join!(pipe_to_client, process_incoming);
  res1.unwrap();
  res2.unwrap();
  // FIXME: clean this up

  info!("{} disconnected", &peer);
  Ok(())
}