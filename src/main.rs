use std::time::Duration;

use ::wdp::{Result, Options, write_pdf};
use webdriver::command::PrintParameters;
use fantoccini::{ClientBuilder, wd::Capabilities};

#[tokio::main]
async fn main() -> Result<()> {
	let options: Options = clap::Parser::parse();

	let capabilities: Capabilities = {
		let headless_arg = match options.headless { true => Some("-headless"), false => None }.unwrap_or("");
		let ff_cap = serde_json::json!({
			"args": [
				headless_arg
			]
		});

		[
			("moz:firefoxOptions".to_string(), ff_cap)
		].into_iter().collect()
	};

	let c = ClientBuilder::rustls()
		.capabilities(capabilities)
		.connect(options.webdriver_url.as_str())
		.await?;


	c.close().await?;
}
