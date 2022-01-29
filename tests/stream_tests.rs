#[macro_use]
extern crate log;

use solitude::{Session, SessionStyle};

use anyhow::Result;
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;

use env_logger::Target;

async fn init() {
	let _ = env_logger::builder()
		.is_test(true)
		.format_module_path(true)
		.target(Target::Stdout)
		.try_init();

	tokio::time::sleep(Duration::from_secs(10)).await;
}

#[tokio::test]
async fn can_create_stream_forwarding_session() -> Result<()> {
	init().await;

	let tcp_listener = TcpListener::bind("127.0.0.1:0").await?;

	let mut session = Session::new("can_create_stream_forwarding_session", SessionStyle::Stream).await?;
	session.forward("127.0.0.1", tcp_listener.local_addr()?.port()).await?;

	Ok(())
}

#[tokio::test]
async fn client_stream_can_send_to_listening_stream() -> Result<()> {
	init().await;

	let test_name = "client_stream_can_send_to_listening_stream";

	let tcp_listener = TcpListener::bind("127.0.0.1:0").await?;
	let port = tcp_listener.local_addr()?.port();

	tokio::task::spawn(async move {
		debug!("awaiting connections");
		for (stream, _address) in tcp_listener.accept().await {
			debug!("received connection");

			let mut buffer = String::new();
			let mut reader = BufReader::new(stream);
			reader.read_line(&mut buffer).await.unwrap();
			debug!("Received message: {}", buffer);
		}
	});

	let mut session = Session::new(test_name, SessionStyle::Stream).await?;
	session.forward("127.0.0.1", port).await?;

	let client_stream_session_name = format!("{}_client", test_name);

	let mut client_stream = Session::new(client_stream_session_name, SessionStyle::Stream).await?;
	let mut tcp_stream = client_stream.connect_stream(session.public_key).await?;

	tcp_stream.write_all("Hello World!".as_bytes()).await?;

	Ok(())
}
