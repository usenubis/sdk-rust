use crate::client::NubisClient;
use crate::error::Result;
use crate::types::{CreateVmRequest, Vm};

/// VMs resource API
pub struct Vms<'a> {
    client: &'a NubisClient,
}

impl<'a> Vms<'a> {
    pub(crate) fn new(client: &'a NubisClient) -> Self {
        Self { client }
    }

    /// List all VMs in a project
    pub async fn list(&self, project_id: &str) -> Result<Vec<Vm>> {
        let response: serde_json::Value = self
            .client
            .get(&format!("/api/v1/projects/{}/vms", project_id))
            .await?;

        // Handle both { vms: [...] } and [...] formats
        if let Some(vms) = response.get("vms").and_then(|v| v.as_array()) {
            Ok(serde_json::from_value(serde_json::Value::Array(
                vms.clone(),
            ))?)
        } else if let Some(vms) = response.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(
                vms.clone(),
            ))?)
        } else {
            Ok(vec![])
        }
    }

    /// Get a specific VM by ID
    pub async fn get(&self, project_id: &str, vm_id: &str) -> Result<Vm> {
        self.client
            .get(&format!("/api/v1/projects/{}/vms/{}", project_id, vm_id))
            .await
    }

    /// Create a new VM
    pub async fn create(&self, request: CreateVmRequest) -> Result<Vm> {
        let project_id = request.project_id.clone();
        let payload = serde_json::json!({
            "name": request.name,
            "size": request.size,
            "region": request.region,
            "image": request.image,
            "ssh_keys": request.ssh_keys,
            "ssh_public_key": request.ssh_public_key,
            "ssh_public_key_id": request.ssh_public_key_id,
            "network_id": request.network_id,
            "firewall_id": request.firewall_id,
            "public_ip": request.public_ip.unwrap_or(true),
            "tags": request.tags,
            "enable_password_auth": request.enable_password_auth.unwrap_or(false),
            "admin_password": request.admin_password,
            "ssh_allowed_cidrs": request.ssh_allowed_cidrs,
        });

        self.client
            .post(&format!("/api/v1/projects/{}/vms", project_id), &payload)
            .await
    }

    /// Delete a VM
    pub async fn delete(&self, project_id: &str, vm_id: &str) -> Result<()> {
        self.client
            .delete::<serde_json::Value>(&format!(
                "/api/v1/projects/{}/vms/{}",
                project_id, vm_id
            ))
            .await?;
        Ok(())
    }

    /// Start a VM
    pub async fn start(&self, project_id: &str, vm_id: &str) -> Result<Vm> {
        self.client
            .post(
                &format!("/api/v1/projects/{}/vms/{}/start", project_id, vm_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Stop a VM
    pub async fn stop(&self, project_id: &str, vm_id: &str) -> Result<Vm> {
        self.client
            .post(
                &format!("/api/v1/projects/{}/vms/{}/stop", project_id, vm_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reboot a VM
    pub async fn reboot(&self, project_id: &str, vm_id: &str) -> Result<Vm> {
        self.client
            .post(
                &format!("/api/v1/projects/{}/vms/{}/reboot", project_id, vm_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Resize a VM
    pub async fn resize(&self, project_id: &str, vm_id: &str, size: &str) -> Result<Vm> {
        self.client
            .post(
                &format!("/api/v1/projects/{}/vms/{}/resize", project_id, vm_id),
                &serde_json::json!({ "size": size }),
            )
            .await
    }

    /// Get VM metrics
    pub async fn metrics(&self, project_id: &str, vm_id: &str) -> Result<serde_json::Value> {
        self.client
            .get(&format!(
                "/api/v1/projects/{}/vms/{}/metrics",
                project_id, vm_id
            ))
            .await
    }
}
