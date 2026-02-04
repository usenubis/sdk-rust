use crate::client::NubisClient;
use crate::error::Result;
use crate::types::Image;

/// Images (distributions) resource API
pub struct Images<'a> {
    client: &'a NubisClient,
}

impl<'a> Images<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all available images
    pub async fn list(&self) -> Result<Vec<Image>> {
        let response: serde_json::Value = self.client.get("/api/v1/catalog/images").await?;

        if let Some(images) = response.get("images").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(
                images.clone(),
            ))?)
        } else if let Some(images) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(
                images.clone(),
            ))?)
        } else {
            Ok(vec![])
        }
    }
}
