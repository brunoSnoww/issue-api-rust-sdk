use reqwest::{Client, Error, RequestBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Api {
    client: Client,
    base_url: String,
    token: Arc<Mutex<Option<String>>>,
}

impl Api {
    pub fn new(base_url: &str) -> Self {
        Api {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn set_token(&self, new_token: &str) {
        let mut token = self.token.lock().await;
        *token = Some(new_token.to_string());
    }

    async fn request_builder(&self, method: reqwest::Method, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        let token = self.token.lock().await;
        let mut req_builder = self.client.request(method, &url);

        if let Some(ref t) = *token {
            req_builder = req_builder.bearer_auth(t);
        }

        req_builder
    }

    pub async fn get(&self, endpoint: &str) -> Result<String, Error> {
        let builder = self.request_builder(reqwest::Method::GET, endpoint).await;
        let response = builder.send().await?;
        response.json().await.map_err(Error::from)
    }

    pub async fn post(&self, endpoint: &str, body: &str) -> Result<String, Error> {
        let builder = self.request_builder(reqwest::Method::POST, endpoint).await;
        let response = builder.body(body.to_string()).send().await?;
        response.text().await.map_err(Error::from)
    }

    // Additional methods (put, delete, etc.) can be defined in a similar way.
}
