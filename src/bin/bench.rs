#[cfg(not(all(feature = "http_client", feature = "nanoserde_bin")))]
fn main() {
	println!("http_client and nanoserde_bin features required");
	std::process::exit(2);
}

#[cfg(all(feature = "http_client", feature = "nanoserde_bin"))]
const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-cli/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);

#[cfg(all(feature = "http_client", feature = "nanoserde_bin"))]
fn main() {
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

	println!("Checking health");
	rt.block_on(client.query(query::HealthCheck)).unwrap();

	{
		println!("Querying cloud stats");
		let d = rt.block_on(client.query(query::Sessions)).unwrap();

		std::fs::write("local/cloud-stats.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/cloud-stats.bin", &s).unwrap();
	}

	{
		println!("Querying online statistics");
		let d = rt.block_on(client.query(query::OnlineStatistics)).unwrap();

		std::fs::write("local/online-stats.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/online-stats.bin", &s).unwrap();
	}

	{
		println!("Querying all sessions");
		let d = rt.block_on(client.query(query::Sessions)).unwrap();

		std::fs::write("local/all-sessions.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/all-sessions.bin", &s).unwrap();
	}

	{
		println!("Querying contacts");
		let d = rt.block_on(client.query(query::Contacts)).unwrap();

		std::fs::write("local/contacts.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/contacts.bin", &s).unwrap();
	}

	{
		println!("Querying messages");
		let q = query::Messages { max_amount: 100, ..Default::default() };
		let d = rt.block_on(client.query(q)).unwrap();

		std::fs::write("local/messages.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/messages.bin", &s).unwrap();
	}

	{
		println!("Querying self user info");
		let q = query::UserInfo { user: user_session.user_id.into() };
		let d = rt.block_on(client.query(q)).unwrap();

		std::fs::write("local/user-info.json", serde_json::to_vec(&d).unwrap())
			.unwrap();

		let mut s = Vec::new();
		d.ser_bin(&mut s);
		std::fs::write("local/user-info.bin", &s).unwrap();
	}
}
