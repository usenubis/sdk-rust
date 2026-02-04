use crate::client::NubisClient;
use crate::error::Result;
use crate::types::Region;

/// Regions resource API
pub struct Regions<'a> {
    client: &'a NubisClient,
}

impl<'a> Regions<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all available regions
    pub async fn list(&self) -> Result<Vec<Region>> {
        let response: serde_json::Value = self.client.get("/api/v1/catalog/regions").await?;

        if let Some(regions) = response.get("regions").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(
                regions.clone(),
            ))?)
        } else if let Some(regions) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(
                regions.clone(),
            ))?)
        } else {
            Ok(vec![])
        }
    }
}
