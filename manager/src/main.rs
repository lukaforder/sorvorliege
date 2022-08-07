use log::*;

use rustls_pemfile::certs;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::{ServerConfig, PrivateKey, Certificate};
use std::fs::File;
use std::io::{self, BufReader};
use std::sync::Arc;

use tokio::net::{TcpListener};
use tokio_tungstenite::tungstenite::Error;
use anyhow::Result;

mod commands;
mod server;
mod messaging;
mod communicator;
mod schema;

extern crate strum;


// FIXME: not hardcode cert/key paths

fn load_certs(path: &str) -> Result<Vec<Certificate>> {
	let raw = certs(
		&mut BufReader::new(File::open(path)?)
	)?;

	Ok(raw.iter().map(|b| Certificate(b.clone())).collect())
}

fn load_key(path: &str) -> Result<PrivateKey> {
	Ok(PrivateKey(rustls_pemfile::pkcs8_private_keys(
		&mut BufReader::new(File::open(path)?)
	)?[0].clone()))
}

#[tokio::main]
async fn main() -> io::Result<()> {
	env_logger::init();

	// FIXME: move this to subcommand or seperate binary
	schema::generate_schemas()?;

	let certs = load_certs("ssl/sorverlord.dev+3.pem").unwrap();
	let key = load_key("ssl/sorverlord.dev+3-key.pem").unwrap();

	let config = ServerConfig::builder()
		.with_safe_defaults()
		.with_no_client_auth()
		.with_single_cert(certs, key)
		.expect("Failed to create server config");
	let acceptor = TlsAcceptor::from(Arc::new(config));

	let addr = "0.0.0.0:18249";
	let listener = TcpListener::bind(&addr).await.expect("could not bind to address");

	let acceptor = Arc::new(acceptor);

	info!("Listening on: {}", addr);

	while let Ok((stream, _)) = listener.accept().await {
		let acceptor = acceptor.clone();
		let peer = stream.peer_addr().expect("connected streams should have a peer address");
		info!("Peer address: {}", peer);
		if let Ok(stream) = acceptor.accept(stream).await {
			tokio::spawn(async move {
				if let Err(e) = messaging::handle_connection(peer, stream).await {
					match e {
						Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
						err => error!("Error processing connection: {:?}", err),
					}
				}
			});
		}
	}
	Ok(()) as io::Result<()>
}