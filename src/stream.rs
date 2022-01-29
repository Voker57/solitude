use crate::*;
use tokio::io::AsyncBufReadExt;

// TODO: add FROM_PORT and TO_PORT
pub struct StreamInfo {
	pub destination: String,
}

impl StreamInfo {
	pub async fn from_bufread<T: tokio::io::AsyncBufRead + std::marker::Unpin>(stream: &mut T) -> Result<Self> {
		debug!("deserializing stream info");

		let mut header = String::new();
		stream.read_line(&mut header).await?;

		let expression = regex::Regex::new("^[^ ]+")?;

		let destination = expression
			.captures(&header)
			.context("Could not regex header")?
			.get(0)
			.context("Could not find destination in header")?
			.as_str()
			.to_owned();

		Ok(Self { destination })
	}
}
