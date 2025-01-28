#[cfg(not(all(feature = "http_client", feature = "nanoserde_bin")))]
fn main() {
	println!("http_client and nanoserde_bin features required");
	std::process::exit(2);
}

#[cfg(all(feature = "http_client", feature = "nanoserde_bin"))]
const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-runner/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);

#[cfg(all(feature = "http_client", feature = "nanoserde_bin"))]
fn main() {
	use std::hash::{DefaultHasher, Hash, Hasher};

	// We really don't need to care about multithreading for this simple tool
	use nanoserde::{DeBin, SerBin};
	use resonite::{api_client::ApiClient, query};

	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.expect("Creating tokio runtime to work");

	let user_session = {
		let bytes = std::fs::read("local/user-session.bin")
			.expect("reading auth from `local/user-session.bin` to work");
		resonite::model::UserSession::deserialize_bin(&bytes).expect("parsing auth")
	};

	let client = resonite::api_client::AuthenticatedResonite::new(
		USER_AGENT.to_owned(),
		user_session.clone(),
	)
	.unwrap();

	println!("Ensuring connection is OK");
	rt.block_on(client.query(query::HealthCheck)).unwrap();

	let mut prev_hash: Option<u64> = None;
	let mut sleep_s = 1;
	loop {
		std::thread::sleep(std::time::Duration::from_secs(sleep_s));
		if sleep_s == 1 {
			sleep_s = 15;
		}

		let filename = "local/sessions/".to_owned()
			+ &time::OffsetDateTime::now_utc()
				.format(&time::format_description::well_known::Rfc3339)
				.unwrap();

		println!("Querying all sessions to {filename}");

		let d = match rt.block_on(client.query(query::Sessions)) {
			Ok(d) => d,
			Err(e) => {
				eprintln!("Failed querying sessions; {e}");
				sleep_s *= 2;
				continue;
			}
		};

		// Never going to happen due to updated timestamps, but was worth a shot...
		let mut hash = DefaultHasher::new();
		d.hash(&mut hash);
		let hash = hash.finish();
		if let Some(prev_hash) = prev_hash {
			if hash == prev_hash {
				print!("Sessions not updated, skipping writing");
				continue;
			}
		}
		prev_hash = Some(hash);

		match serde_json::to_vec(&d) {
			Ok(d) => match std::fs::write(filename.clone() + ".json", d) {
				Ok(_) => {}
				Err(e) => {
					eprintln!("Failed writing sessions json; {e}");
					sleep_s *= 2;
					continue;
				}
			},
			Err(e) => {
				eprintln!("Failed serializing sessions to json; {e}");
				sleep_s *= 2;
				continue;
			}
		};

		let mut buf = Vec::new();
		d.ser_bin(&mut buf);
		match std::fs::write(filename + ".bin", &buf) {
			Ok(_) => {}
			Err(e) => {
				eprintln!("Failed writing sessions bin; {e}");
				sleep_s *= 2;
				continue;
			}
		}

		sleep_s = 15;
	}
}
