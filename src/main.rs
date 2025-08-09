mod get;
use clap::Parser;
use get::get_req;
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,
    
    #[arg(short = 'H', long, value_delimiter = '=', num_args = 0..)]
    header: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let url = match args.url {
        Some(x) => x,
        None => {
            println!("Please pass a valid url");
            return;
        }
    };
    
    // Parse headers into HashMap
    let mut headers = HashMap::new();
    for header in args.header {
        if let Some(separator_index) = header.find(':') {
            let key = header[..separator_index].trim().to_string();
            let value = header[separator_index + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }
    
    get_req(&url, &headers).await;
}
