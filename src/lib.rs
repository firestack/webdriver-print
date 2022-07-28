use std::path::PathBuf;
use url::Url;
use webdriver::command::PrintParameters;

/// This `Result` type allows us to dynamically return anything implementing `Error` in the `Err(E)` enum
#[doc(hidden)]
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> = core::result::Result<T, E>;

#[derive(clap::Parser, Debug)]
pub struct Options {
	pub input_url: Url,

	#[clap(short, long, default_value = "http://localhost:4444")]
	pub webdriver_url: Url,

	#[clap(short, long, default_value = "./output.pdf")]
	pub output_filename: PathBuf,

	#[clap(short, long)]
	pub keep_failure: bool,

	#[clap(short, long, action)]
	pub headless: bool,
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
	parameters: PrintParameters
}

impl From<PrintParameters> for PrintPDF
{
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
