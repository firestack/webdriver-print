use webdriver::command::PrintParameters;

#[doc(hidden)]
pub type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

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
