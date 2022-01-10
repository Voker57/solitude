use solitude::{DatagramMessage, Session, SessionStyle};

use std::net::UdpSocket;

use anyhow::Result;

fn init() {
	let _ = env_logger::builder().is_test(true).format_module_path(true).try_init();
}

#[test]
fn can_create_datagram_session() -> Result<()> {
	init();

	let test_name = "can_create_datagram_session".to_owned();

	let mut session = Session::new(test_name, SessionStyle::Datagram)?;
	session.forward("127.0.0.1".to_owned(), 0)?;

	session.close()?;
	Ok(())
}

#[test]
fn can_create_raw_session() -> Result<()> {
	init();

	let test_name = "can_create_raw_session".to_owned();

	let mut session = Session::new(test_name, SessionStyle::Raw)?;
	session.forward("127.0.0.1".to_owned(), 0)?;

	Ok(())
}

#[test]
fn can_send_raw_datagram_to_service() -> Result<()> {
	can_send_datagram_or_raw_to_service(SessionStyle::Raw)?;
	Ok(())
}

#[test]
fn can_send_datagram_to_service() -> Result<()> {
	can_send_datagram_or_raw_to_service(SessionStyle::Datagram)?;
	Ok(())
}

fn can_send_datagram_or_raw_to_service(session_style: SessionStyle) -> Result<()> {
	init();

	let test_name = "can_send_data_to_service".to_string();

	let (udp_socket, second_udp_socket) = create_two_udp_sockets()?;

	let mut session = Session::new(test_name.clone(), session_style)?;
	session.forward(String::from("127.0.0.1"), udp_socket.local_addr()?.port())?;

	let mut second_session = Session::new(test_name.clone() + "_second", session_style)?;
	second_session.forward(String::from("127.0.0.1"), second_udp_socket.local_addr()?.port())?;

	let destination = second_session.look_up("ME".to_string())?;

	let datagram_message = DatagramMessage::new(&test_name, &destination, [0x05, 0x15].to_vec());
	let datagram_message_bytes = datagram_message.serialize();

	// Attempt to receive the datagram on another thread
	let handle = std::thread::spawn(move || {
		let mut buffer = [0u8; 2048];
		second_udp_socket.recv(&mut buffer)
	});

	udp_socket.send(&datagram_message_bytes)?;

	// Ensure the datagram was received
	handle.join().unwrap().unwrap();

	session.close()?;
	second_session.close()?;

	Ok(())
}

#[test]
fn can_create_datagram_message() -> Result<()> {
	init();

	let contents: [u8; 32] = rand::random();
	let _datagram_message = DatagramMessage::new("test", "test_destination", contents.to_vec());

	Ok(())
}

#[test]
fn can_serialize_datagram_message() -> Result<()> {
	init();

	let contents: [u8; 32] = rand::random();
	let datagram_message = DatagramMessage::new("test", "test_destination", contents.to_vec());
	let _datagram_message_bytes = datagram_message.serialize();

	Ok(())
}

#[test]
fn can_deserialize_datagram_message() -> Result<()> {
	init();

	let example_received_datagram_bytes = [
		0x4a, 0x37, 0x61, 0x67, 0x75, 0x4b, 0x7e, 0x6a, 0x6c, 0x65, 0x75, 0x7e, 0x7a, 0x50, 0x7a, 0x64, 0x63, 0x64, 0x59, 0x36, 0x77, 0x47,
		0x47, 0x6c, 0x6d, 0x6c, 0x64, 0x6d, 0x53, 0x57, 0x47, 0x57, 0x30, 0x78, 0x4b, 0x7e, 0x65, 0x34, 0x62, 0x6f, 0x42, 0x31, 0x43, 0x7a,
		0x64, 0x54, 0x63, 0x38, 0x53, 0x6c, 0x37, 0x2d, 0x78, 0x6e, 0x79, 0x6a, 0x4f, 0x71, 0x79, 0x58, 0x78, 0x4f, 0x54, 0x68, 0x6a, 0x61,
		0x42, 0x43, 0x78, 0x72, 0x69, 0x4c, 0x48, 0x4c, 0x4d, 0x7e, 0x38, 0x55, 0x34, 0x46, 0x75, 0x6c, 0x49, 0x42, 0x78, 0x57, 0x61, 0x71,
		0x58, 0x2d, 0x57, 0x6d, 0x59, 0x54, 0x2d, 0x4e, 0x50, 0x57, 0x73, 0x7e, 0x7e, 0x7e, 0x32, 0x39, 0x44, 0x64, 0x76, 0x6b, 0x6e, 0x73,
		0x4c, 0x74, 0x7a, 0x78, 0x33, 0x57, 0x56, 0x71, 0x6b, 0x45, 0x66, 0x38, 0x55, 0x4e, 0x2d, 0x36, 0x45, 0x2d, 0x78, 0x4b, 0x7a, 0x78,
		0x4d, 0x41, 0x36, 0x50, 0x61, 0x44, 0x4a, 0x74, 0x78, 0x71, 0x44, 0x51, 0x77, 0x34, 0x48, 0x65, 0x44, 0x4e, 0x78, 0x30, 0x56, 0x45,
		0x75, 0x71, 0x54, 0x72, 0x63, 0x4a, 0x76, 0x37, 0x72, 0x74, 0x52, 0x31, 0x35, 0x79, 0x75, 0x4b, 0x34, 0x67, 0x6e, 0x47, 0x33, 0x77,
		0x6b, 0x7e, 0x58, 0x6e, 0x58, 0x77, 0x45, 0x75, 0x4f, 0x70, 0x32, 0x43, 0x64, 0x7e, 0x55, 0x32, 0x66, 0x35, 0x57, 0x72, 0x34, 0x6a,
		0x78, 0x79, 0x71, 0x76, 0x42, 0x39, 0x4e, 0x4c, 0x39, 0x38, 0x31, 0x61, 0x48, 0x47, 0x45, 0x6c, 0x57, 0x76, 0x63, 0x6e, 0x61, 0x78,
		0x38, 0x77, 0x6d, 0x5a, 0x42, 0x52, 0x6c, 0x33, 0x64, 0x41, 0x37, 0x4a, 0x35, 0x64, 0x51, 0x7a, 0x4d, 0x38, 0x77, 0x66, 0x66, 0x52,
		0x63, 0x7e, 0x69, 0x2d, 0x45, 0x70, 0x4b, 0x62, 0x43, 0x42, 0x33, 0x55, 0x77, 0x63, 0x61, 0x51, 0x30, 0x4f, 0x4a, 0x66, 0x45, 0x63,
		0x6a, 0x62, 0x56, 0x6e, 0x71, 0x47, 0x49, 0x5a, 0x4e, 0x4f, 0x55, 0x70, 0x74, 0x69, 0x69, 0x35, 0x6a, 0x6e, 0x6b, 0x58, 0x7a, 0x49,
		0x78, 0x72, 0x61, 0x42, 0x37, 0x56, 0x39, 0x32, 0x49, 0x2d, 0x34, 0x49, 0x67, 0x50, 0x30, 0x6a, 0x2d, 0x6d, 0x59, 0x56, 0x4d, 0x73,
		0x55, 0x6c, 0x4e, 0x71, 0x57, 0x61, 0x69, 0x56, 0x79, 0x7a, 0x66, 0x59, 0x57, 0x69, 0x37, 0x57, 0x4e, 0x49, 0x6a, 0x37, 0x6d, 0x52,
		0x44, 0x47, 0x6f, 0x34, 0x79, 0x62, 0x4e, 0x4c, 0x36, 0x43, 0x47, 0x7a, 0x32, 0x73, 0x76, 0x33, 0x74, 0x6d, 0x67, 0x35, 0x35, 0x62,
		0x56, 0x30, 0x30, 0x49, 0x2d, 0x61, 0x4d, 0x78, 0x43, 0x69, 0x4e, 0x50, 0x62, 0x62, 0x35, 0x66, 0x70, 0x72, 0x45, 0x47, 0x76, 0x45,
		0x6d, 0x32, 0x74, 0x47, 0x44, 0x54, 0x64, 0x41, 0x6c, 0x59, 0x42, 0x46, 0x70, 0x78, 0x42, 0x7a, 0x32, 0x48, 0x4c, 0x33, 0x37, 0x32,
		0x51, 0x45, 0x59, 0x74, 0x7a, 0x48, 0x33, 0x74, 0x54, 0x4d, 0x49, 0x4b, 0x52, 0x59, 0x62, 0x70, 0x56, 0x4b, 0x71, 0x6c, 0x50, 0x43,
		0x75, 0x59, 0x45, 0x6e, 0x51, 0x55, 0x6a, 0x46, 0x39, 0x43, 0x44, 0x49, 0x6b, 0x52, 0x7a, 0x4d, 0x4c, 0x58, 0x6b, 0x6e, 0x4c, 0x4f,
		0x7e, 0x71, 0x63, 0x76, 0x42, 0x2d, 0x32, 0x70, 0x6c, 0x50, 0x7e, 0x6e, 0x69, 0x73, 0x4c, 0x42, 0x6f, 0x45, 0x59, 0x30, 0x49, 0x6d,
		0x36, 0x6c, 0x5a, 0x52, 0x52, 0x37, 0x54, 0x36, 0x4f, 0x51, 0x35, 0x4f, 0x74, 0x70, 0x45, 0x62, 0x4d, 0x49, 0x79, 0x7e, 0x76, 0x65,
		0x48, 0x31, 0x57, 0x65, 0x74, 0x33, 0x34, 0x51, 0x76, 0x72, 0x35, 0x35, 0x71, 0x54, 0x34, 0x77, 0x42, 0x4f, 0x76, 0x79, 0x58, 0x4b,
		0x4e, 0x6e, 0x6d, 0x76, 0x67, 0x70, 0x41, 0x41, 0x41, 0x41, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
		0x21,
	];

	let destination_string = "J7aguK~jleu~zPzdcdY6wGGlmldmSWGW0xK~e4boB1CzdTc8Sl7-xnyjOqyXxOThjaBCxriLHLM~8U4FulIBxWaqX-WmYT-NPWs~~~29DdvknsLtzx3WVqkEf8UN-6E-xKzxMA6PaDJtxqDQw4HeDNx0VEuqTrcJv7rtR15yuK4gnG3wk~XnXwEuOp2Cd~U2f5Wr4jxyqvB9NL981aHGElWvcnax8wmZBRl3dA7J5dQzM8wffRc~i-EpKbCB3UwcaQ0OJfEcjbVnqGIZNOUptii5jnkXzIxraB7V92I-4IgP0j-mYVMsUlNqWaiVyzfYWi7WNIj7mRDGo4ybNL6CGz2sv3tmg55bV00I-aMxCiNPbb5fprEGvEm2tGDTdAlYBFpxBz2HL372QEYtzH3tTMIKRYbpVKqlPCuYEnQUjF9CDIkRzMLXknLO~qcvB-2plP~nisLBoEY0Im6lZRR7T6OQ5OtpEbMIy~veH1Wet34Qvr55qT4wBOvyXKNnmvgpAAAA";

	let datagram_message_after_deserialization = DatagramMessage::from_bytes("test", &example_received_datagram_bytes)?;

	assert_eq!(datagram_message_after_deserialization.contents, b"Hello World!");
	assert_eq!(datagram_message_after_deserialization.destination, destination_string);

	Ok(())
}

fn create_two_udp_sockets() -> Result<(UdpSocket, UdpSocket)> {
	let first_udp_socket = UdpSocket::bind("127.0.0.1:0")?;
	let second_udp_socket = UdpSocket::bind("127.0.0.1:0")?;

	first_udp_socket.connect("127.0.0.1:7655")?;
	second_udp_socket.connect("127.0.0.1:7655")?;

	Ok((first_udp_socket, second_udp_socket))
}
