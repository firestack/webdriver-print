use std::time::Duration;

use ::wdp::{Result, Options, write_pdf};
use webdriver::command::PrintParameters;
use fantoccini::{ClientBuilder, wd::Capabilities};

#[tokio::main]
async fn main() -> Result<()> {
	let options: Options = clap::Parser::parse();


	let c = ClientBuilder::rustls()
		.connect(options.webdriver_url.as_str())
		.await?;


	c.close().await?;
}
