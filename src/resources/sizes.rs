use crate::client::NubisClient;
use crate::error::Result;
use crate::types::Size;

/// Sizes (VM plans) resource API
pub struct Sizes<'a> {
    client: &'a NubisClient,
}

impl<'a> Sizes<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all available sizes
    pub async fn list(&self, region: Option<&str>) -> Result<Vec<Size>> {
        let path = "/api/v1/catalog/sizes";
        let response = if let Some(region) = region {
            self.client
                .get_with_params(path, &[("region", region)])
                .await?
        } else {
            self.client.get(path).await?
        };

        let response: serde_json::Value = response;

        if let Some(sizes) = response.get("sizes").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(sizes.clone()))?)
        } else if let Some(sizes) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(sizes.clone()))?)
        } else {
            Ok(vec![])
        }
    }
}
