#![cfg(feature = "signalr_client")]

use resonite::api_client::ApiError;
use tokio_stream::StreamExt;
mod common;

#[tokio::test]
#[ignore]
async fn listen_signalr() -> Result<(), ApiError> {
	let api_client = common::api_signalr().await;

	let listener_ref = api_client.listen();
	let mut listener_lock = listener_ref.lock().await;

	// Listen for a few messages, resonite spams session updates lots so should be
	// quick
	for _ in 0..3 {
		let next = listener_lock
			.next()
			.await
			.expect("WS listener to have next item")
			.expect("next WS item to not be err");
		dbg!(&next);
	}

	Ok(())
}
