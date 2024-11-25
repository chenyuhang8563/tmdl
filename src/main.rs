//! This example illustrates the way to send and receive arbitrary JSON.
//!
//! This is useful for some ad-hoc experiments and situations when you don't
//! really care about the structure of the JSON and just need to display it or
//! process it at runtime.



use std::io;

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error");
    let echo_json = reqwest::Client::new()
        .get(format!("https://api.themoviedb.org/3/tv/{}?api_key=813659d1715d4e488fe5d6210e455234&language=zh-CN",num.trim()))
        .headers(headers)
        .send()
        .await?;
    let res = echo_json.error_for_status()?;
    // bytes -> u8
    let body = res.text().await?;





    println!("{body:#?}");
    Ok(())
}