#![cfg(feature = "http_client")]

use racal::reqwest::{ApiClient, ApiError};

mod common;

#[tokio::test]
#[ignore]
async fn extend_session() -> Result<(), ApiError> {
	let client = common::api_auth();

	let extend_session = resonite::query::ExtendUserSession;
	client.query(extend_session).await?;

	Ok(())
}
