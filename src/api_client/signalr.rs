use async_trait::async_trait;
use serde::Serialize;
use tokio::{
	sync::mpsc::UnboundedSender,
	task::{JoinHandle, JoinSet},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::ApiError;
use crate::query::Authentication;

type ListenMessageResult = Result<crate::signalr::Message, ApiError>;

/// A thread-safe mutex for a stream of receiving messages from the server
pub type ReceiverContainer = std::sync::Arc<
	tokio::sync::Mutex<UnboundedReceiverStream<ListenMessageResult>>,
>;

/// A `SignalR` (`WebSocket`) API client
pub struct ResoniteSignalRClient {
	receive: ReceiverContainer,
	handle: JoinSet<()>,
	internal_client: ezsockets::Client<InternalClientExt>,
}

struct InternalClientExt {
	received_sender: UnboundedSender<ListenMessageResult>,
	connected_sender: UnboundedSender<bool>,
}

impl InternalClientExt {
	/// Turns a WS receiving channel to an async streams
	fn send_ws_msg(&self, bytes: &[u8]) {
		let res: ListenMessageResult =
			serde_json::from_slice::<crate::signalr::Message>(bytes)
				.map_err(ApiError::from);
		match self.received_sender.send(res) {
			Ok(v) => v,
			Err(_e) => {
				// TODO: Error handling
			}
		};
	}
}

#[async_trait]
impl ezsockets::ClientExt for InternalClientExt {
	type Call = ();

	async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
		self.send_ws_msg(text.as_bytes());
		Ok(())
	}

	async fn on_binary(
		&mut self, bytes: Vec<u8>,
	) -> Result<(), ezsockets::Error> {
		self.send_ws_msg(&bytes);
		Ok(())
	}

	async fn on_call(
		&mut self, _params: Self::Call,
	) -> Result<(), ezsockets::Error> {
		Ok(())
	}

	async fn on_connect(&mut self) -> Result<(), ezsockets::Error> {
		dbg!("Connected!");
		self.connected_sender.send(true).ok();

		Ok(())
	}
}

impl ResoniteSignalRClient {
	/// Creates a new `SignalR` client
	///
	/// # Errors
	///
	/// If creating the client/connection fails
	pub async fn new(
		user_agent: &str, auth: &Authentication,
	) -> Result<Self, ApiError> {
		let mut ws_config = ezsockets::ClientConfig::new(crate::SIGNALR_HUB_URI);

		let (header_name, header_value) = auth.to_header();
		ws_config = ws_config.header(header_name, header_value);
		ws_config = ws_config.header("User-Agent", user_agent);

		let (received_sender, received_receiver) =
			tokio::sync::mpsc::unbounded_channel::<ListenMessageResult>();

		let (connected_sender, mut connected_receiver) =
			tokio::sync::mpsc::unbounded_channel::<bool>();

		let (internal_client, future) = ezsockets::connect(
			|_client| InternalClientExt { received_sender, connected_sender },
			ws_config,
		)
		.await;

		let mut handle = JoinSet::new();

		if let Err(_e) = internal_client.call(()) {
			// TODO: Error handling
		}

		handle.spawn(async move {
			// Resolves once connection is closed
			future.await.ok();
		});

		let client_clone = internal_client.clone();
		handle.spawn(async move {
			loop {
				dbg!("Awaiting connection");
				connected_receiver.recv().await;

				client_clone.binary(r#"{"protocol":"json","version":1}"#).ok();
				dbg!("Sent protocol");
			}
		});

		let ws_client = Self {
			internal_client,
			handle,
			receive: std::sync::Arc::new(tokio::sync::Mutex::new(
				UnboundedReceiverStream::from(received_receiver),
			)),
		};

		Ok(ws_client)
	}

	/// Sends a `SignalR` message to the Resonite API.
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub fn send<T: Serialize>(
		&self, requestable: &crate::signalr::Message,
	) -> Result<(), ApiError> {
		let data = serde_json::to_string(requestable)?;
		self
			.internal_client
			.binary(data)
			.map_err(|e| ApiError::Other(e.to_string()))?;

		Ok(())
	}

	#[must_use]
	/// Gets the events sent by the server
	pub fn listen(&self) -> ReceiverContainer { self.receive.clone() }
}

impl Drop for ResoniteSignalRClient {
	fn drop(&mut self) { self.handle.abort_all(); }
}
