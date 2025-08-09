# Rust's curl 🦀

- v1 - simple http get with optional headers

## Usage

- Using the short option
  `curll -u "https://jsonplaceholder.typicode.com/posts/1"`
- Using the long option
  `curll --url "https://jsonplaceholder.typicode.com/posts/1"`

## With Headers

- Single header
  `curll -u "https://api.example.com/data" -H "Authorization: Bearer token123"`
- Multiple headers
  `curll -u "https://api.example.com/data" -H "Authorization: Bearer token123" -H "Content-Type: application/json"`
- Using long option for headers
  `curll --url "https://api.example.com/data" --header "Authorization: Bearer token123" --header "Content-Type: application/json"`
