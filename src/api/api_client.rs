use dotenv::dotenv;
use reqwest::{header, Client, Error, StatusCode};

struct Api {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl Api {
    pub fn new(base_url: String) -> Self {
        Api {
            client: Client::new(),
            base_url,
            token: None,
        }
    }

    pub fn set_token(&mut self, new_token: String) {
        self.token = Some(new_token);
    }

    async fn make_request(&self, endpoint: &str) -> Result<String, Error> {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request_builder = self.client.get(&url);

        if let Some(ref token) = self.token {
            request_builder =
                request_builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }

        let response = request_builder.send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            todo!()
        }
    }
}
