use solitude::{Session, SessionStyle};

use std::time::Duration;

use anyhow::Result;

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
async fn service_can_be_resolved() -> Result<()> {
	init().await;

	let (session, mut second_session) = create_two_sessions("service_can_be_resolved", SessionStyle::Datagram, 0, 0).await?;

	let session_address = session.address()?;
	let name = second_session.look_up(session_address.clone()).await?;
	println!("resolved {} to {}", session_address, name);

	Ok(())
}

#[tokio::test]
async fn session_can_be_restored() -> Result<()> {
	init().await;

	let test_name = "session_can_be_restored";

	let (address, public_key, private_key) = {
		let session = Session::new(test_name, SessionStyle::Stream).await?;

		(session.address()?, session.public_key, session.private_key)
	};

	let session = Session::from(format!("{}_restore", test_name), SessionStyle::Stream, public_key, private_key).await?;

	assert!(address == session.address()?);

	Ok(())
}

async fn create_two_sessions(
	test_name: &str,
	session_style: SessionStyle,
	first_port: u16,
	second_port: u16,
) -> Result<(Session, Session)> {
	let mut first_session = Session::new(format!("{}_first", test_name), session_style).await?;
	first_session.forward("127.0.0.1", first_port).await?;

	let mut second_session = Session::new(format!("{}_second", test_name), session_style).await?;
	second_session.forward("127.0.0.1", second_port).await?;

	Ok((first_session, second_session))
}
