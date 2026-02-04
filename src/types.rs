use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the Nubis client
#[derive(Debug, Clone)]
pub struct NubisConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Option<std::time::Duration>,
}

impl Default for NubisConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.usenubis.com".to_string(),
            timeout: Some(std::time::Duration::from_secs(30)),
        }
    }
}

/// Request to create a new VM
#[derive(Debug, Clone, Serialize)]
pub struct CreateVmRequest {
    pub project_id: String,
    pub name: String,
    pub size: String,
    pub region: String,
    pub image: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_password_auth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ssh_allowed_cidrs: Vec<String>,
}

/// VM representation
#[derive(Debug, Clone, Deserialize)]
pub struct Vm {
    pub id: String,
    pub name: String,
    pub status: String,
    pub size: String,
    pub region: String,
    pub image: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Project representation
#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub org_id: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Organization representation
#[derive(Debug, Clone, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Region representation
#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub available: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Size (VM plan) representation
#[derive(Debug, Clone, Deserialize)]
pub struct Size {
    pub id: String,
    pub slug: String,
    pub memory: u64,
    pub vcpus: u32,
    pub disk: u64,
    pub transfer: u64,
    #[serde(default)]
    pub price_monthly: f64,
    #[serde(default)]
    pub price_hourly: f64,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Image (distribution) representation
#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// API error response
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
