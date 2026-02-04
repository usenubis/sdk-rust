use crate::error::{NubisError, Result};
use crate::types::NubisConfig;
use reqwest::{Client, Response};
use std::time::Duration;

/// Main Nubis SDK client
pub struct NubisClient {
    client: Client,
    base_url: String,
}

impl NubisClient {
    /// Create a new Nubis client with the given API key
    pub fn new(api_key: String) -> Self {
        Self::with_config(NubisConfig {
            api_key,
            ..Default::default()
        })
    }

    /// Create a new Nubis client with custom configuration
    pub fn with_config(config: NubisConfig) -> Self {
        let client_builder = Client::builder()
            .timeout(config.timeout.unwrap_or(Duration::from_secs(30)))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", config.api_key).parse().unwrap(),
                );
                headers
            });

        let client = client_builder.build().expect("Failed to create HTTP client");

        Self {
            client,
            base_url: config.base_url,
        }
    }

    /// Make a GET request
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Make a GET request with query parameters
    pub(crate) async fn get_with_params<T>(&self, path: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.get(&url);
        for (key, value) in params {
            request = request.query(&[(key, value)]);
        }
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request
    pub(crate) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.post(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Make a PUT request
    pub(crate) async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.put(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub(crate) async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.delete(&url).send().await?;
        self.handle_response(response).await
    }

    /// Handle HTTP response and extract JSON or error
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            if text.is_empty() {
                // Empty response for DELETE operations
                serde_json::from_str("{}").map_err(NubisError::Serialization)
            } else {
                serde_json::from_str(&text).map_err(NubisError::Serialization)
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            let error_message = if let Ok(api_error) = serde_json::from_str::<crate::types::ApiErrorResponse>(&error_text) {
                api_error.error.message
            } else {
                format!("HTTP {}: {}", status, error_text)
            };

            Err(NubisError::Api {
                status: status.as_u16(),
                message: error_message,
            })
        }
    }
}
