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

	for _i in 0..3 {
		let next = listener_lock
			.next()
			.await
			.expect("WS listener to have next item")
			.expect("To be able to parse next item");
		dbg!(&next);
	}

	Ok(())
}
