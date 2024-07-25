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

#[tokio::test]
#[ignore]
async fn friends() -> Result<(), ApiError> {
	let client = common::api_auth();

	let friends_query = resonite::query::Friends::default();
	let friends = client.query(friends_query).await?;

	// Resonite bot will always be at least one friend of yours
	assert!(!friends.is_empty());

	Ok(())
}

#[tokio::test]
#[ignore]
async fn get_messages() -> Result<(), ApiError> {
	let client = common::api_auth();

	let messages_query = resonite::query::Messages::default();
	let messages = client.query(messages_query).await?;

	//println!("Messages: {:?}", messages);

	// Test user should have at least a single message
	assert!(!messages.is_empty());

	Ok(())
}
