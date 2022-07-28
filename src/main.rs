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

	let webdriver = ClientBuilder::rustls()
		.capabilities(capabilities)
		.connect(options.webdriver_url.as_str())
		.await?;


	let pdf_print_parameters = PrintParameters {
		background: true,
		..Default::default()
	};

	let pdf_result = write_pdf(&webdriver, &options, pdf_print_parameters).await;
	if pdf_result.is_err() && options.keep_failure {
		tokio::time::sleep(Duration::from_secs(10)).await;
	}

	webdriver.close().await?;

	pdf_result
}
