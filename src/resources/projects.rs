use crate::client::NubisClient;
use crate::error::Result;
use crate::types::Project;

/// Projects resource API
pub struct Projects<'a> {
    client: &'a NubisClient,
}

impl<'a> Projects<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all projects in an organization
    pub async fn list(&self, org_id: &str) -> Result<Vec<Project>> {
        let response: serde_json::Value = self
            .client
            .get(&format!("/api/v1/orgs/{}/projects", org_id))
            .await?;

        if let Some(projects) = response.get("projects").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(
                projects.clone(),
            ))?)
        } else if let Some(projects) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(
                projects.clone(),
            ))?)
        } else {
            Ok(vec![])
        }
    }

    /// Get a specific project by ID
    pub async fn get(&self, org_id: &str, project_id: &str) -> Result<Project> {
        self.client
            .get(&format!("/api/v1/orgs/{}/projects/{}", org_id, project_id))
            .await
    }

    /// Create a new project
    pub async fn create(&self, org_id: &str, name: &str, slug: &str) -> Result<Project> {
        self.client
            .post(
                &format!("/api/v1/orgs/{}/projects", org_id),
                &serde_json::json!({
                    "name": name,
                    "slug": slug,
                }),
            )
            .await
    }

    /// Delete a project
    pub async fn delete(&self, org_id: &str, project_id: &str) -> Result<()> {
        self.client
            .delete::<serde_json::Value>(&format!(
                "/api/v1/orgs/{}/projects/{}",
                org_id, project_id
            ))
            .await?;
        Ok(())
    }
}
