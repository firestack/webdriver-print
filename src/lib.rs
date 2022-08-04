use std::{path::PathBuf, time::Duration};
use tokio::time::sleep;
use url::Url;
use webdriver::command::PrintParameters;

/// This `Result` type allows us to dynamically return anything implementing `Error` in the `Err(E)` enum
#[doc(hidden)]
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> =
	core::result::Result<T, E>;

#[derive(clap::Parser, Debug)]
pub struct Options {
	#[clap(env = "WDP_INPUT_URL")]
	pub input_url: Url,

	#[clap(
		short,
		long,
		default_value = "http://localhost:4444",
		env = "WDP_WEBDRIVER_URL"
	)]
	pub webdriver_url: Url,

	#[clap(
		short,
		long,
		default_value = "./output.pdf",
		env = "WDP_OUTPUT_FILENAME"
	)]
	pub output_filename: PathBuf,

	#[clap(short, long, env = "WDP_KEEP_FAILURE")]
	pub keep_failure: bool,

	#[clap(short, long, action, env = "WDP_HEADLESS")]
	pub headless: bool,

	#[clap(
		short = 'c',
		long,
		default_value = "./capabilities.json",
		env = "WDP_BROWSER_CAPABILITIES_CONFIG"
	)]
	pub browser_capabilities_config: PathBuf,

	#[clap(
		short = 'p',
		long,
		default_value = "./print_parameters.json",
		env = "WDP_PRINT_PARAMETERS_CONFIG"
	)]
	pub print_parameters_config: PathBuf,

	#[clap(long, env = "WDP_DEBUG_SLEEP_LENGTH")]
	pub _debug_sleep_length: Option<std::num::NonZeroU16>,
}

pub async fn write_pdf(
	client: &fantoccini::Client,
	opt: &Options,
	parameters: PrintParameters,
) -> Result<()> {
	// Open target page
	client.goto(opt.input_url.as_str()).await?;

	loop {
		// Wait for documentReadystate == complete
		let result = client.execute_async(r#"
			const [callback] = arguments;

			let predicate = readyState => {
				const state = readyState == "complete";
				if (state) { callback(true); };
				return state;
			};

			if (!predicate(document.readyState)) {
				document.addEventListener('readystatechange', (event) => predicate(event.target.readyState));
			}
		"#, Default::default()).await;

		if result.is_ok() { break; }
	}

	if let Some(length) = opt._debug_sleep_length {
		sleep(Duration::from_secs(length.get().into())).await;
	}

	// get current page as PDF via byte array
	let pdf_data = print_pdf(client, parameters).await?;
	// write byte array to file
	std::fs::write(&opt.output_filename, pdf_data)?;

	Ok(())
}

pub async fn print_pdf(c: &fantoccini::Client, parameters: PrintParameters) -> Result<Vec<u8>> {
	// let print_command: WDPrint = parameters.into();

	let json_cmd_result = c.issue_cmd(PrintPDF { parameters }).await?;
	let string_value = json_cmd_result.as_str().map(ToOwned::to_owned);
	let encoded_pdf: String = string_value.ok_or("Printed result was empty")?;

	Ok(base64::decode(&encoded_pdf)?)
}

#[derive(Debug)]
pub struct PrintPDF {
	parameters: PrintParameters,
}

impl From<PrintParameters> for PrintPDF {
	fn from(parameters: PrintParameters) -> Self {
		PrintPDF { parameters }
	}
}

impl fantoccini::wd::WebDriverCompatibleCommand for PrintPDF {
	fn endpoint(
		&self,
		base_url: &url::Url,
		session_id: Option<&str>,
	) -> Result<url::Url, url::ParseError> {
		base_url.join(&format!("session/{}/print", session_id.as_ref().unwrap()))
	}

	fn method_and_body(&self, _request_url: &url::Url) -> (http::Method, Option<String>) {
		(
			http::Method::POST,
			Some(serde_json::to_string(&self.parameters).unwrap()),
		)
	}
}
