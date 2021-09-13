use std::error::Error;
use reqwest::Client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let client = Client::builder().build()?;
    let request = client.get("http://www.bilibili.com").build()?;
    let response = client.execute(request).await?;
    print!("{}", response.text().await?);

    Ok(())
}
