#[cfg(not(feature = "http_client"))]
fn main() {
	println!("http_client feature required");
	std::process::exit(2);
}

#[cfg(feature = "http_client")]
use std::{fs::File, io, process::exit};

#[cfg(feature = "http_client")]
const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-AuthHelper/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);

#[cfg(feature = "http_client")]
fn main() {
	// We really don't need to care about multithreading for this simple tool

	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.expect("Creating tokio runtime to work");

	let user_session_file = File::create("local/user-session.json")
		.expect("Creating local/user-session.json file to work");

	println!(
		"This is a very simple helper tool to generate a `user-session.json` file for running the integration tests."
	);
	println!(
		"Be aware that it does not censor what you type (including the password)."
	);

	let stdin = io::stdin();

	let username = {
		let input = &mut String::new();
		println!("Username?");
		input.clear();
		stdin.read_line(input).expect("Reading input to work");
		input.trim().to_owned()
	};

	if username.is_empty() {
		println!("Username cannot be empty!");
		exit(1);
	}

	let password = {
		let input = &mut String::new();
		println!("Password?");
		input.clear();
		stdin.read_line(input).expect("Reading input to work");
		let input = input.trim_end_matches('\n').to_owned();

		if input.is_empty() {
			println!("Username cannot be empty!");
			exit(1);
		}

		input
	};

	let unique_machine_identifier = {
		let input = &mut String::new();
		#[cfg(not(feature = "rand_util"))]
		println!("UID?");
		#[cfg(feature = "rand_util")]
		println!("UID? (enter empty to generate)");
		input.clear();
		stdin.read_line(input).expect("Reading input to work");
		let mut input = input.trim().to_owned();

		if input.is_empty() {
			#[cfg(not(feature = "rand_util"))]
			{
				println!("UID cannot be empty!");
				exit(1);
			}
			#[cfg(feature = "rand_util")]
			{
				input = resonite::util::random_ascii_string(32);
			}
		}
		input
	};

	let secret_machine_id = {
		let input = &mut String::new();
		#[cfg(not(feature = "rand_util"))]
		println!("Secret machine ID?");
		#[cfg(feature = "rand_util")]
		println!("Secret machine ID? (enter empty to generate)");
		input.clear();
		stdin.read_line(input).expect("Reading input to work");
		let mut input = input.trim().to_owned();

		if input.is_empty() {
			#[cfg(not(feature = "rand_util"))]
			{
				println!("UID cannot be empty!");
				exit(1);
			}
			#[cfg(feature = "rand_util")]
			{
				input = resonite::util::random_ascii_string(32);
			}
		}
		input
	};

	let second_factor = {
		let input = &mut String::new();
		println!("TOTP? (enter empty if none)");
		input.clear();
		stdin.read_line(input).expect("Reading input to work");
		let input = input.trim().to_owned();

		if input.is_empty() { None } else { Some(input) }
	};

	let auth_state = resonite::query::Authenticating {
		second_factor,
		unique_machine_identifier: unique_machine_identifier.clone(),
	};

	let queryable = resonite::query::UserSession {
		remember_me: true,
		secret_machine_id,
		authentication: resonite::query::UserSessionAuthentication::Password(
			resonite::query::UserSessionPasswordAuthentication {
				password,
				recovery_code: None,
			},
		),
		identifier: resonite::query::LoginCredentialsIdentifier::Username(username),
	};

	// Execute the future, blocking the current thread until completion
	let user_session = rt
		.block_on(request_session(auth_state, queryable))
		.expect("login should succeed");
	drop(rt);

	println!("Login successful");
	serde_json::to_writer_pretty(user_session_file, &user_session.user_session)
		.expect("Writing user session to work");

	println!(
		"Dumping config files not implemented, received: {:?}",
		user_session.config_files
	);
}

#[cfg(feature = "http_client")]
async fn request_session(
	auth_state: resonite::query::Authenticating,
	queryable: resonite::query::UserSession,
) -> Result<resonite::model::UserSessionResult, racal::reqwest::ApiError> {
	use resonite::api_client::ApiClient;
	let client =
		resonite::api_client::UnauthenticatedResonite::new(USER_AGENT.to_owned())
			.expect("Creating API client to work");

	let client =
		resonite::api_client::AuthenticatingResonite::from((client, auth_state));
	client.query(queryable).await
}
