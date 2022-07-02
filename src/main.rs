use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	{
		use tracing_subscriber::{EnvFilter, fmt, prelude::*};

		tracing_subscriber::registry()
			.with(fmt::layer())
			.with(EnvFilter::from_default_env())
			.init();
	}

	let (browser, mut handler) =
		Browser::launch(BrowserConfig::builder().with_head().chrome_executable(std::env::var_os("BROWSER").expect("Requires browser")).build()?).await?;

	let handle = async_std::task::spawn(async move {
		loop {
			let _event = handler.next().await.unwrap();
		}
	});

	let page = browser.new_page("https://en.wikipedia.org").await?;

	// type into the search field and hit `Enter`,
	// this triggers a navigation to the search result page
	page.find_element("input#searchInput")
		.await?
		.click()
		.await?
		.type_str("Rust programming language")
		.await?
		.press_key("Enter")
		.await?;

	let html = page.wait_for_navigation().await?.content().await?;

	handle.await;
	Ok(())
}
