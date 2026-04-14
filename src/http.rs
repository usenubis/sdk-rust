use std::time::Duration;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

use crate::NubisError;

const DEFAULT_BASE_URL: &str = "https://nubis-core.onrender.com";
const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Clone)]
pub struct NubisClient {
    http: Client,
    base_url: String,
    api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NubisClientBuilder {
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
    default_headers: HeaderMap,
}

impl NubisClientBuilder {
    pub fn new() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            default_headers,
        }
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn default_header(mut self, key: &'static str, value: impl AsRef<str>) -> Self {
        if let Ok(header_value) = HeaderValue::from_str(value.as_ref()) {
            self.default_headers.insert(key, header_value);
        }
        self
    }

    pub fn build(self) -> Result<NubisClient, NubisError> {
        let http = Client::builder()
            .timeout(self.timeout)
            .default_headers(self.default_headers)
            .build()?;

        Ok(NubisClient {
            http,
            base_url: self.base_url,
            api_key: self.api_key,
        })
    }
}

impl Default for NubisClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NubisClient {
    pub fn builder() -> NubisClientBuilder {
        NubisClientBuilder::new()
    }

    pub fn new(api_key: impl Into<String>) -> Result<Self, NubisError> {
        Self::builder().api_key(api_key).build()
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self, NubisError> {
        Self::builder().base_url(base_url).build()
    }

    pub async fn request_value<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path_template: &str,
        path_params: &[(&str, &str)],
        query: Option<&[(&str, &str)]>,
        body: Option<&B>,
    ) -> Result<Value, NubisError> {
        let rendered_path = render_path(path_template, path_params);
        let url = build_url(&self.base_url, &rendered_path);

        let mut req = self.http.request(method, url);
        if let Some(api_key) = &self.api_key {
            req = req.header(AUTHORIZATION, format!("Bearer {api_key}"));
        }
        if let Some(query) = query {
            req = req.query(query);
        }
        if let Some(body) = body {
            req = req.header(CONTENT_TYPE, "application/json").json(body);
        }

        let response = req.send().await?;
        let status = response.status();
        let text = response.text().await?;
        let body_json = parse_body(&text);

        if !status.is_success() {
            let message = extract_error_message(
                &body_json,
                status.canonical_reason().unwrap_or("Request failed"),
            );
            return Err(NubisError::Http {
                status: status.as_u16(),
                message,
                body: body_json,
            });
        }

        Ok(body_json)
    }

    pub async fn request<T, B>(
        &self,
        method: Method,
        path_template: &str,
        path_params: &[(&str, &str)],
        query: Option<&[(&str, &str)]>,
        body: Option<&B>,
    ) -> Result<T, NubisError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let value = self
            .request_value(method, path_template, path_params, query, body)
            .await?;
        Ok(serde_json::from_value(value)?)
    }
}

fn parse_body(text: &str) -> Value {
    if text.trim().is_empty() {
        Value::Null
    } else {
        serde_json::from_str(text).unwrap_or_else(|_| json!(text))
    }
}

fn extract_error_message(body: &Value, fallback: &str) -> String {
    if let Some(message) = body
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
    {
        return message.to_string();
    }

    if let Some(message) = body
        .get("message")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
    {
        return message.to_string();
    }

    if let Some(message) = body.as_str().filter(|value| !value.trim().is_empty()) {
        return message.to_string();
    }

    fallback.to_string()
}

pub(crate) fn render_path(path_template: &str, path_params: &[(&str, &str)]) -> String {
    let mut rendered = path_template.to_string();
    for (name, value) in path_params {
        let placeholder = format!(":{name}");
        let encoded = utf8_percent_encode(value, NON_ALPHANUMERIC).to_string();
        rendered = rendered.replace(&placeholder, &encoded);
    }
    rendered
}

fn build_url(base_url: &str, path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}
