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

	let udp_socket = UdpSocket::bind("127.0.0.1:0").await?;
	udp_socket.connect("127.0.0.1:7655").await?;

	let port = udp_socket.local_addr()?.port();

	let mut session = Session::new("echo_server", solitude::SessionStyle::Datagram).await?;
	session.forward("127.0.0.1", port).await?;

	info!("Listening on i2p at {}", session.address()?);

	let mut buffer = [0u8; 2048];

	loop {
		info!("Waiting to receive");
		let frame = udp_socket.recv(&mut buffer).await?;
		let buffer = &mut buffer[..frame];

		debug!("Received datagram bytes: {:02x?}", buffer);

		let received_datagram = match DatagramMessage::from_bytes("echo_server", buffer) {
			Ok(received_datagram) => received_datagram,
			Err(_) => {
				debug!("Received a datagram but could not deserialize it");
				continue;
			}
		};

		info!("Received datagram: {}", std::str::from_utf8(&received_datagram.contents)?);

		let datagram_to_send = DatagramMessage::new("echo_server", &received_datagram.destination, received_datagram.contents);

		let datagram_to_send_bytes = datagram_to_send.serialize();

		udp_socket.send(&datagram_to_send_bytes).await?;
	}
}
