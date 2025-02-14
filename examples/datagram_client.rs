#[macro_use]
extern crate log;

use solitude::{DatagramMessage, Session};

use tokio::net::UdpSocket;

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

	let udp_socket = UdpSocket::bind("127.0.0.1:0").await?;
	udp_socket.connect("127.0.0.1:7655").await?;
	let port = udp_socket.local_addr()?.port();

	let mut session = Session::new("echo_client", solitude::SessionStyle::Datagram).await?;
	session.forward("127.0.0.1", port).await?;

	let hostname = arguments[1].to_owned();

	let destination = session.look_up(hostname).await?;

	let datagram = DatagramMessage::new("echo_client", &destination, b"Hello World!".to_vec());
	info!("Sending datagram");
	debug!("datagram: {:x?}", datagram);

	let datagram_bytes = datagram.serialize();

	// Sends 10 datagrams over one second. Datagrams fail occasionally, this makes it likely that
	// at least on will go through
	for _ in 0..10 {
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;

		udp_socket.send(&datagram_bytes).await?;
		info!("Sent datagram");
	}

	Ok(())
}
