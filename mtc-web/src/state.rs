static API_URL: &str = "https://localhost/api";

pub struct AppState {
    pub api_url: String,
    pub api_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> Self {
        let api_client = reqwest::Client::builder().build().unwrap();

        Self {
            api_url: API_URL.to_string(),
            api_client,
        }
    }
}