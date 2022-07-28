use webdriver::command::PrintParameters;

/// This `Result` type allows us to dynamically return anything implementing `Error` in the `Err(E)` enum
#[doc(hidden)]
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> = core::result::Result<T, E>;


#[derive(Debug)]
pub struct WDPrint {
	parameters: PrintParameters
}

impl From<PrintParameters> for WDPrint
{
	fn from(parameters: PrintParameters) -> Self {
		WDPrint { parameters }
	}
}

impl fantoccini::wd::WebDriverCompatibleCommand for WDPrint {
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
