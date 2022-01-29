use tokio::io::AsyncWriteExt;

#[macro_use]
extern crate log;

use solitude::{Session, SessionStyle};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::builder()
		.filter_level(log::LevelFilter::Info)
		.parse_env("RUST_LOG")
		.init();

	let arguments: Vec<String> = std::env::args().collect();

	if arguments.len() < 2 {
		panic!("must supply I2P hostname, i.e. eva example.i2p");
	}

	let server_name = arguments[1].to_owned();

	info!("Creating a SAM v3 session");
	let mut session = Session::new("stream_client_example", SessionStyle::Stream).await?;
	let destination = session.look_up(server_name).await?;

	info!("Connecting to server");
	let mut tcp_stream = session.connect_stream(destination).await?;
	tcp_stream.write_all("Hello World!".as_bytes()).await?;
	tcp_stream.flush().await?;
	info!("Sent message!");

	Ok(())
}
