use std::collections::HashMap;

pub async fn get_req(url: &str, headers: &HashMap<String, String>) -> () {
    let client = reqwest::Client::new();
    let mut request_builder = client.get(url);
    
    // Add headers to the request
    for (key, value) in headers {
        request_builder = request_builder.header(key, value);
    }
    
    match request_builder.send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => println!("Response:\n{}", body),
                    Err(e) => eprintln!("Error reading response body: {}", e),
                }
            } else {
                eprintln!("Error: Received non-success status: {}", response.status());
            }
        }
        Err(e) => eprintln!("Error making request: {}", e),
    }
}
