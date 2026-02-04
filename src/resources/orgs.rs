use crate::client::NubisClient;
use crate::error::Result;
use crate::types::Organization;

/// Organizations resource API
pub struct Orgs<'a> {
    client: &'a NubisClient,
}

impl<'a> Orgs<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all organizations
    pub async fn list(&self) -> Result<Vec<Organization>> {
        let response: serde_json::Value = self.client.get("/api/v1/orgs").await?;

        if let Some(orgs) = response.get("orgs").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(orgs.clone()))?)
        } else if let Some(orgs) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(orgs.clone()))?)
        } else {
            Ok(vec![])
        }
    }

    /// Get a specific organization by ID
    pub async fn get(&self, org_id: &str) -> Result<Organization> {
        self.client.get(&format!("/api/v1/orgs/{}", org_id)).await
    }

    /// Create a new organization
    pub async fn create(&self, name: &str, slug: &str) -> Result<Organization> {
        self.client
            .post(
                "/api/v1/orgs",
                &serde_json::json!({
                    "name": name,
                    "slug": slug,
                }),
            )
            .await
    }

    /// Delete an organization
    pub async fn delete(&self, org_id: &str) -> Result<()> {
        self.client
            .delete::<serde_json::Value>(&format!("/api/v1/orgs/{}", org_id))
            .await?;
        Ok(())
    }
}
