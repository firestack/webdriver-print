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
