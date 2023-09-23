// Something's funky with checking if these are used or not.
#![allow(dead_code)]

use once_cell::sync::Lazy;

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-TestRunner/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);
