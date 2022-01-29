use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

#[macro_use]
extern crate log;

use solitude::{Session, SessionStyle, StreamInfo};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::builder()
		.filter_level(log::LevelFilter::Info)
		.parse_env("RUST_LOG")
		.init();

	info!("Creating tcp server");
	let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
	let port = tcp_listener.local_addr()?.port();

	info!("Creating SAMv3 session");
	let mut session = Session::new("stream_server_example", SessionStyle::Stream).await?;
	info!("Forwarding tcp server to i2p");
	session.forward("127.0.0.1", port).await?;

	info!("Listening on {}", session.address()?);

	for (mut stream, _address) in tcp_listener.accept().await {
		let mut reader = BufReader::new(&mut stream);
		let stream_info = StreamInfo::from_bufread(&mut reader).await?;

		let mut data = String::new();
		reader.read_line(&mut data).await?;

		info!("received: \"{}\" from {}", data, stream_info.destination);
	}

	Ok(())
}
