#![cfg(feature = "http_client")]

use racal::reqwest::{ApiClient, ApiError};

mod common;

// #[tokio::test]
// #[ignore]
// async fn ping() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	client.query(resonite::query::Ping).await?;

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn online_user_count() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	assert!(client.query(resonite::query::OnlineUserCount).await? > 0);

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn online_instance_count() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	assert!(client.query(resonite::query::OnlineInstanceCount).await? > 0);

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn get_user() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	let user_id = resonite::id::User::try_from("U-Resonite").unwrap();
// 	let user_id_query = resonite::query::UserInfo::new(user_id);
// 	let user_from_id = client.query(user_id_query).await?;
// 	let user_name_query = resonite::query::UserInfo::new("Resonite");
// 	let user_from_username = client.query(user_name_query).await?;

// 	assert_eq!(user_from_id.id, user_from_username.id);
// 	assert_eq!(user_from_id.username, user_from_username.username);

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn get_user_status() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	let user_id = resonite::id::User::try_from("U-Resonite").unwrap();
// 	let user_status_query = resonite::query::UserStatus::new(user_id);
// 	let _user_status = client.query(user_status_query).await?;

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn search_users() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	let user_search_query = resonite::query::UserSearch::new("Resonite");
// 	let users = client.query(user_search_query).await?;

// 	assert!(!users.is_empty());

// 	let resonite_bot_user = users.iter().find(|user| user.username == "Resonite");

// 	assert!(resonite_bot_user.is_some());

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn sessions() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	let sessions = client.query(resonite::query::Sessions).await?;

// 	let public_session = sessions
// 		.iter()
// 		.find(|session| {
// 			session.access_level == resonite::model::SessionAccessLevel::Anyone
// 				&& session.is_valid
// 		})
// 		.expect("there should be at least one public session");

// 	// Test that getting a specific session works.
// 	let session = client
// 		.query(resonite::query::SessionInfo::new(public_session.id.clone()))
// 		.await?;

// 	// Some basic sanity checks, can't do full eq since some data might've changed
// 	assert!(session.id == public_session.id);
// 	assert!(session.host_id == public_session.host_id);
// 	assert!(session.compatibility_hash == public_session.compatibility_hash);

// 	Ok(())
// }

// #[tokio::test]
// #[ignore]
// async fn get_group() -> Result<(), ApiError> {
// 	let client = common::api_no_auth();

// 	let group_id = resonite::id::Group::try_from("G-Resonite").unwrap();
// 	let group_query = resonite::query::GroupInfo::new(group_id);
// 	let _group = client.query(group_query).await?;

// 	Ok(())
// }
