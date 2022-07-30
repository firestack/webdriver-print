use std::time::Duration;

use fantoccini::{wd::Capabilities, ClientBuilder};
use wdp::{write_pdf, Options, Result};
use webdriver::command::PrintParameters;

#[tokio::main]
async fn main() -> Result<()> {
	let options: Options = clap::Parser::parse();

	let capabilities = get_browser_capabilities(&options);

	let webdriver = ClientBuilder::rustls()
		.capabilities(capabilities)
		.connect(options.webdriver_url.as_str())
		.await?;

	let pdf_print_parameters = get_print_parameters(&options);

	let pdf_result = write_pdf(&webdriver, &options, pdf_print_parameters).await;
	if pdf_result.is_err() && options.keep_failure {
		tokio::time::sleep(Duration::from_secs(10)).await;
	}

	webdriver.close().await?;

	pdf_result
}

fn get_print_parameters(options: &Options) -> PrintParameters {
	let parameters: Result<PrintParameters> = read_json_to_type(&options.print_parameters_config);

	parameters.unwrap_or_else(|err| {
		eprintln!(
			"Errored attempting to read print parameters configuration file (`{}`): {err}",
			options.print_parameters_config.display()
		);
		PrintParameters {
			background: true,
			..Default::default()
		}
	})
}

fn get_browser_capabilities(options: &Options) -> Capabilities {
	let mut capabilities: Capabilities = {
		let caps: Result<Capabilities> = read_json_to_type(&options.browser_capabilities_config);

		caps.unwrap_or_else(|err| {
			eprintln!(
				"Error attempting to read capabilities configuration file(`{}`): {err}",
				options.browser_capabilities_config.display()
			);

			Default::default()
		})
	};

	if options.headless {
		let ff_options = capabilities
			.entry("moz:firefoxOptions")
			.or_insert_with(|| Default::default());

		if ff_options["args"].is_null() {
			ff_options["args"] = json!([]);
		}
		assert_eq!(true, ff_options["args"].is_array());
		// if !ff_options["args"].is_array() { panic!(); }

		ff_options["args"]
			.as_array_mut()
			.map(|arr| arr.push(json!("-headless")));
	}

	capabilities
}

fn read_json_to_type<FT, P>(path: P) -> Result<FT>
where
	P: AsRef<Path>,
	FT: serde::de::DeserializeOwned,
{
	let data = std::fs::read(path)?;
	Ok(serde_json::from_slice::<FT>(&data)?)
}
