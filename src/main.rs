use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut url = String::new();
    match args.url {
        Some(x) => {
            url = x;
        }
        None => println!("Please pass a valid url"),
    }
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => println!("Response Body:\n{}", body),
                    Err(e) => eprintln!("Error reading response body: {}", e),
                }
            } else {
                eprintln!("Error: Received non-success status: {}", response.status());
            }
        }
        Err(e) => eprintln!("Error making request: {}", e),
    }
}
