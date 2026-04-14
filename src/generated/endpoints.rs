// GENERATED FILE - DO NOT EDIT BY HAND
// Source: services/api-gateway/src/main.rs routes

use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

use crate::{NubisClient, NubisError};

impl NubisClient {
    /// `DELETE /api/v1/api-keys/:key_id`
    pub async fn delete_api_v1_api_keys_by_key_id<B: Serialize + ?Sized>(
        &self,
        key_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/api-keys/:key_id",
            &[("key_id", key_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id`
    pub async fn delete_api_v1_orgs_by_org_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/billing/payment-method`
    pub async fn delete_api_v1_orgs_by_org_id_billing_payment_method<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/billing/payment-method",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/billing/payment-methods/:method_id`
    pub async fn delete_api_v1_orgs_by_org_id_billing_payment_methods_by_method_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        method_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/billing/payment-methods/:method_id",
            &[("org_id", org_id), ("method_id", method_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/iac/webhooks/:webhook_id`
    pub async fn delete_api_v1_orgs_by_org_id_iac_webhooks_by_webhook_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        webhook_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/iac/webhooks/:webhook_id",
            &[("org_id", org_id), ("webhook_id", webhook_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/iam/service-accounts/:sa_id`
    pub async fn delete_api_v1_orgs_by_org_id_iam_service_accounts_by_sa_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        sa_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/iam/service-accounts/:sa_id",
            &[("org_id", org_id), ("sa_id", sa_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/invitations/:invite_id`
    pub async fn delete_api_v1_orgs_by_org_id_invitations_by_invite_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        invite_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/invitations/:invite_id",
            &[("org_id", org_id), ("invite_id", invite_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/launch/github/connection`
    pub async fn delete_api_v1_orgs_by_org_id_launch_github_connection<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/launch/github/connection",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/members/:member_id`
    pub async fn delete_api_v1_orgs_by_org_id_members_by_member_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        member_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/members/:member_id",
            &[("org_id", org_id), ("member_id", member_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/observability/alerts/:policy_id`
    pub async fn delete_api_v1_orgs_by_org_id_observability_alerts_by_policy_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        policy_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/observability/alerts/:policy_id",
            &[("org_id", org_id), ("policy_id", policy_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/observability/destinations/:destination_id`
    pub async fn delete_api_v1_orgs_by_org_id_observability_destinations_by_destination_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        destination_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/observability/destinations/:destination_id",
            &[("org_id", org_id), ("destination_id", destination_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/orgs/:org_id/projects/:project_id`
    pub async fn delete_api_v1_orgs_by_org_id_projects_by_project_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/orgs/:org_id/projects/:project_id",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/backups/policies/:policy_id`
    pub async fn delete_api_v1_projects_by_project_id_backups_policies_by_policy_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        policy_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/backups/policies/:policy_id",
            &[("project_id", project_id), ("policy_id", policy_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/certificates/:cert_id`
    pub async fn delete_api_v1_projects_by_project_id_certificates_by_cert_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cert_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/certificates/:cert_id",
            &[("project_id", project_id), ("cert_id", cert_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/databases/:database_id`
    pub async fn delete_api_v1_projects_by_project_id_databases_by_database_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/databases/:database_id",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/databases/:database_id/dbs/:db_name`
    pub async fn delete_api_v1_projects_by_project_id_databases_by_database_id_dbs_by_db_name<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        db_name: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/databases/:database_id/dbs/:db_name",
            &[
                ("project_id", project_id),
                ("database_id", database_id),
                ("db_name", db_name),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/databases/:database_id/pools/:pool_name`
    pub async fn delete_api_v1_projects_by_project_id_databases_by_database_id_pools_by_pool_name<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        pool_name: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/databases/:database_id/pools/:pool_name",
            &[
                ("project_id", project_id),
                ("database_id", database_id),
                ("pool_name", pool_name),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/databases/:database_id/replicas/:replica_name`
    pub async fn delete_api_v1_projects_by_project_id_databases_by_database_id_replicas_by_replica_name<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        replica_name: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/databases/:database_id/replicas/:replica_name",
            &[
                ("project_id", project_id),
                ("database_id", database_id),
                ("replica_name", replica_name),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/databases/:database_id/users/:username`
    pub async fn delete_api_v1_projects_by_project_id_databases_by_database_id_users_by_username<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        username: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/databases/:database_id/users/:username",
            &[
                ("project_id", project_id),
                ("database_id", database_id),
                ("username", username),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/firewalls/:firewall_id`
    pub async fn delete_api_v1_projects_by_project_id_firewalls_by_firewall_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        firewall_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/firewalls/:firewall_id",
            &[("project_id", project_id), ("firewall_id", firewall_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/firewalls/:firewall_id/rules/:rule_id`
    pub async fn delete_api_v1_projects_by_project_id_firewalls_by_firewall_id_rules_by_rule_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        firewall_id: &str,
        rule_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/firewalls/:firewall_id/rules/:rule_id",
            &[
                ("project_id", project_id),
                ("firewall_id", firewall_id),
                ("rule_id", rule_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/floating-ips/:eip_id`
    pub async fn delete_api_v1_projects_by_project_id_floating_ips_by_eip_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        eip_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/floating-ips/:eip_id",
            &[("project_id", project_id), ("eip_id", eip_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/images/:image_id`
    pub async fn delete_api_v1_projects_by_project_id_images_by_image_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        image_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/images/:image_id",
            &[("project_id", project_id), ("image_id", image_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/k8s/:cluster_id`
    pub async fn delete_api_v1_projects_by_project_id_k8s_by_cluster_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/k8s/:cluster_id",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/k8s/:cluster_id/node_pools/:pool_id`
    pub async fn delete_api_v1_projects_by_project_id_k8s_by_cluster_id_node_pools_by_pool_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        pool_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/k8s/:cluster_id/node_pools/:pool_id",
            &[
                ("project_id", project_id),
                ("cluster_id", cluster_id),
                ("pool_id", pool_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/load-balancers/:lb_id`
    pub async fn delete_api_v1_projects_by_project_id_load_balancers_by_lb_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        lb_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/load-balancers/:lb_id",
            &[("project_id", project_id), ("lb_id", lb_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/targets`
    pub async fn delete_api_v1_projects_by_project_id_load_balancers_by_lb_id_target_groups_by_tg_id_targets<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        lb_id: &str,
        tg_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/targets",
            &[
                ("project_id", project_id),
                ("lb_id", lb_id),
                ("tg_id", tg_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/load-balancers/:lb_id/targets/:target_id`
    pub async fn delete_api_v1_projects_by_project_id_load_balancers_by_lb_id_targets_by_target_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        lb_id: &str,
        target_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/targets/:target_id",
            &[
                ("project_id", project_id),
                ("lb_id", lb_id),
                ("target_id", target_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/nat-gateways/:nat_id`
    pub async fn delete_api_v1_projects_by_project_id_nat_gateways_by_nat_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        nat_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/nat-gateways/:nat_id",
            &[("project_id", project_id), ("nat_id", nat_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/networks/:network_id`
    pub async fn delete_api_v1_projects_by_project_id_networks_by_network_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        network_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/networks/:network_id",
            &[("project_id", project_id), ("network_id", network_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/networks/:network_id/subnets/:subnet_id`
    pub async fn delete_api_v1_projects_by_project_id_networks_by_network_id_subnets_by_subnet_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        network_id: &str,
        subnet_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/networks/:network_id/subnets/:subnet_id",
            &[
                ("project_id", project_id),
                ("network_id", network_id),
                ("subnet_id", subnet_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/scaling-groups/:group_id`
    pub async fn delete_api_v1_projects_by_project_id_scaling_groups_by_group_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        group_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/scaling-groups/:group_id",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/security-groups/:sg_id`
    pub async fn delete_api_v1_projects_by_project_id_security_groups_by_sg_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        sg_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/security-groups/:sg_id",
            &[("project_id", project_id), ("sg_id", sg_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/security-groups/:sg_id/rules/:rule_id`
    pub async fn delete_api_v1_projects_by_project_id_security_groups_by_sg_id_rules_by_rule_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        sg_id: &str,
        rule_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/security-groups/:sg_id/rules/:rule_id",
            &[
                ("project_id", project_id),
                ("sg_id", sg_id),
                ("rule_id", rule_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/snapshots/:snapshot_id`
    pub async fn delete_api_v1_projects_by_project_id_snapshots_by_snapshot_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        snapshot_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/snapshots/:snapshot_id",
            &[("project_id", project_id), ("snapshot_id", snapshot_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/ssh_keys/:key_id`
    pub async fn delete_api_v1_projects_by_project_id_ssh_keys_by_key_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        key_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/ssh_keys/:key_id",
            &[("project_id", project_id), ("key_id", key_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/storage/access-keys/:key_id`
    pub async fn delete_api_v1_projects_by_project_id_storage_access_keys_by_key_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        key_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/storage/access-keys/:key_id",
            &[("project_id", project_id), ("key_id", key_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/storage/buckets/:bucket_id`
    pub async fn delete_api_v1_projects_by_project_id_storage_buckets_by_bucket_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/storage/buckets/:bucket_id/cors/:rule_id`
    pub async fn delete_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_cors_by_rule_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        rule_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/cors/:rule_id",
            &[
                ("project_id", project_id),
                ("bucket_id", bucket_id),
                ("rule_id", rule_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle/:rule_id`
    pub async fn delete_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_lifecycle_by_rule_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        rule_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle/:rule_id",
            &[
                ("project_id", project_id),
                ("bucket_id", bucket_id),
                ("rule_id", rule_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/storage/buckets/:bucket_id/objects/*key`
    pub async fn delete_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_objects_key<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/objects/*key",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/templates/:template_id`
    pub async fn delete_api_v1_projects_by_project_id_templates_by_template_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        template_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/templates/:template_id",
            &[("project_id", project_id), ("template_id", template_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/vms/:vm_id`
    pub async fn delete_api_v1_projects_by_project_id_vms_by_vm_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/vms/:vm_id",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id`
    pub async fn delete_api_v1_projects_by_project_id_vms_by_vm_id_disks_by_disk_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        disk_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id",
            &[
                ("project_id", project_id),
                ("vm_id", vm_id),
                ("disk_id", disk_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/volumes/:volume_id`
    pub async fn delete_api_v1_projects_by_project_id_volumes_by_volume_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        volume_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/volumes/:volume_id",
            &[("project_id", project_id), ("volume_id", volume_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/zones/:zone_id`
    pub async fn delete_api_v1_projects_by_project_id_zones_by_zone_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        zone_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/zones/:zone_id",
            &[("project_id", project_id), ("zone_id", zone_id)],
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/projects/:project_id/zones/:zone_id/records/:record_id`
    pub async fn delete_api_v1_projects_by_project_id_zones_by_zone_id_records_by_record_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        zone_id: &str,
        record_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::DELETE,
            "/api/v1/projects/:project_id/zones/:zone_id/records/:record_id",
            &[
                ("project_id", project_id),
                ("zone_id", zone_id),
                ("record_id", record_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `GET /api/v1/account`
    pub async fn get_api_v1_account(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/account",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/api-keys`
    pub async fn get_api_v1_api_keys(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/api-keys",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/audit-logs`
    pub async fn get_api_v1_audit_logs(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/audit-logs",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/audit-logs/export`
    pub async fn get_api_v1_audit_logs_export(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/audit-logs/export",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/databases/options`
    pub async fn get_api_v1_catalog_databases_options(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/databases/options",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/images`
    pub async fn get_api_v1_catalog_images(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/images",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/k8s-node-sizes`
    pub async fn get_api_v1_catalog_k8s_node_sizes(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/k8s-node-sizes",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/k8s-versions`
    pub async fn get_api_v1_catalog_k8s_versions(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/k8s-versions",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/regions`
    pub async fn get_api_v1_catalog_regions(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/regions",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/catalog/sizes`
    pub async fn get_api_v1_catalog_sizes(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/catalog/sizes",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/domains/verify-renewal`
    pub async fn get_api_v1_domains_verify_renewal(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/domains/verify-renewal",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/exchange-rates`
    pub async fn get_api_v1_exchange_rates(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/exchange-rates",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/iam/roles`
    pub async fn get_api_v1_iam_roles(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/iam/roles",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/images`
    pub async fn get_api_v1_images(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/images",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/images/:image_id`
    pub async fn get_api_v1_images_by_image_id(
        &self,
        image_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/images/:image_id",
            &[("image_id", image_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/incidents`
    pub async fn get_api_v1_incidents(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/incidents",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/incidents/:id`
    pub async fn get_api_v1_incidents_by_id(
        &self,
        id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/incidents/:id",
            &[("id", id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/invitations/preview`
    pub async fn get_api_v1_invitations_preview(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/invitations/preview",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/kyc/status`
    pub async fn get_api_v1_kyc_status(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/kyc/status",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/launch/github/callback`
    pub async fn get_api_v1_launch_github_callback(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/launch/github/callback",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs`
    pub async fn get_api_v1_orgs(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id`
    pub async fn get_api_v1_orgs_by_org_id(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/account-status`
    pub async fn get_api_v1_orgs_by_org_id_billing_account_status(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/account-status",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/bandwidth`
    pub async fn get_api_v1_orgs_by_org_id_billing_bandwidth(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/bandwidth",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/credits`
    pub async fn get_api_v1_orgs_by_org_id_billing_credits(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/credits",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/credits/balance`
    pub async fn get_api_v1_orgs_by_org_id_billing_credits_balance(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/credits/balance",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/credits/usage`
    pub async fn get_api_v1_orgs_by_org_id_billing_credits_usage(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/credits/usage",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/enforcement`
    pub async fn get_api_v1_orgs_by_org_id_billing_enforcement(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/enforcement",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/enforcement-settings`
    pub async fn get_api_v1_orgs_by_org_id_billing_enforcement_settings(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/enforcement-settings",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/history`
    pub async fn get_api_v1_orgs_by_org_id_billing_history(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/history",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/payment-method`
    pub async fn get_api_v1_orgs_by_org_id_billing_payment_method(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/payment-method",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/payment-methods`
    pub async fn get_api_v1_orgs_by_org_id_billing_payment_methods(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/payment-methods",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/resources`
    pub async fn get_api_v1_orgs_by_org_id_billing_resources(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/resources",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/spend-limit`
    pub async fn get_api_v1_orgs_by_org_id_billing_spend_limit(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/spend-limit",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/status`
    pub async fn get_api_v1_orgs_by_org_id_billing_status(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/status",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/tax-info`
    pub async fn get_api_v1_orgs_by_org_id_billing_tax_info(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/tax-info",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/usage`
    pub async fn get_api_v1_orgs_by_org_id_billing_usage(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/usage",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/usage/breakdown`
    pub async fn get_api_v1_orgs_by_org_id_billing_usage_breakdown(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/usage/breakdown",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/usage/export`
    pub async fn get_api_v1_orgs_by_org_id_billing_usage_export(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/usage/export",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/usage/summary`
    pub async fn get_api_v1_orgs_by_org_id_billing_usage_summary(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/usage/summary",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/usage/timeseries`
    pub async fn get_api_v1_orgs_by_org_id_billing_usage_timeseries(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/usage/timeseries",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/billing/verification`
    pub async fn get_api_v1_orgs_by_org_id_billing_verification(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/billing/verification",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iac/webhooks`
    pub async fn get_api_v1_orgs_by_org_id_iac_webhooks(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iac/webhooks",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iac/webhooks/:webhook_id/deliveries`
    pub async fn get_api_v1_orgs_by_org_id_iac_webhooks_by_webhook_id_deliveries(
        &self,
        org_id: &str,
        webhook_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iac/webhooks/:webhook_id/deliveries",
            &[("org_id", org_id), ("webhook_id", webhook_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/me`
    pub async fn get_api_v1_orgs_by_org_id_iam_me(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/me",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/permissions`
    pub async fn get_api_v1_orgs_by_org_id_iam_permissions(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/permissions",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/roles`
    pub async fn get_api_v1_orgs_by_org_id_iam_roles(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/roles",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/roles/:role_id/permissions`
    pub async fn get_api_v1_orgs_by_org_id_iam_roles_by_role_id_permissions(
        &self,
        org_id: &str,
        role_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/roles/:role_id/permissions",
            &[("org_id", org_id), ("role_id", role_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/service-accounts`
    pub async fn get_api_v1_orgs_by_org_id_iam_service_accounts(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/service-accounts",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/iam/sso`
    pub async fn get_api_v1_orgs_by_org_id_iam_sso(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/iam/sso",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/invitations`
    pub async fn get_api_v1_orgs_by_org_id_invitations(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/invitations",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/invoices`
    pub async fn get_api_v1_orgs_by_org_id_invoices(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/invoices",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/invoices/:invoice_id/download`
    pub async fn get_api_v1_orgs_by_org_id_invoices_by_invoice_id_download(
        &self,
        org_id: &str,
        invoice_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/invoices/:invoice_id/download",
            &[("org_id", org_id), ("invoice_id", invoice_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/access`
    pub async fn get_api_v1_orgs_by_org_id_launch_access(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/access",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/deployments/:deployment_id`
    pub async fn get_api_v1_orgs_by_org_id_launch_deployments_by_deployment_id(
        &self,
        org_id: &str,
        deployment_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/deployments/:deployment_id",
            &[("org_id", org_id), ("deployment_id", deployment_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/deployments/:deployment_id/events`
    pub async fn get_api_v1_orgs_by_org_id_launch_deployments_by_deployment_id_events(
        &self,
        org_id: &str,
        deployment_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/deployments/:deployment_id/events",
            &[("org_id", org_id), ("deployment_id", deployment_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/deployments/:deployment_id/logs`
    pub async fn get_api_v1_orgs_by_org_id_launch_deployments_by_deployment_id_logs(
        &self,
        org_id: &str,
        deployment_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/deployments/:deployment_id/logs",
            &[("org_id", org_id), ("deployment_id", deployment_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/github/connection`
    pub async fn get_api_v1_orgs_by_org_id_launch_github_connection(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/github/connection",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/github/repositories`
    pub async fn get_api_v1_orgs_by_org_id_launch_github_repositories(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/github/repositories",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/projects`
    pub async fn get_api_v1_orgs_by_org_id_launch_projects(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/projects",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/projects/:project_id`
    pub async fn get_api_v1_orgs_by_org_id_launch_projects_by_project_id(
        &self,
        org_id: &str,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/projects/:project_id",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/projects/:project_id/services`
    pub async fn get_api_v1_orgs_by_org_id_launch_projects_by_project_id_services(
        &self,
        org_id: &str,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/projects/:project_id/services",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/services/:service_id`
    pub async fn get_api_v1_orgs_by_org_id_launch_services_by_service_id(
        &self,
        org_id: &str,
        service_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/services/:service_id",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/services/:service_id/deployments`
    pub async fn get_api_v1_orgs_by_org_id_launch_services_by_service_id_deployments(
        &self,
        org_id: &str,
        service_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/services/:service_id/deployments",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/services/:service_id/domains`
    pub async fn get_api_v1_orgs_by_org_id_launch_services_by_service_id_domains(
        &self,
        org_id: &str,
        service_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/services/:service_id/domains",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/services/:service_id/logs`
    pub async fn get_api_v1_orgs_by_org_id_launch_services_by_service_id_logs(
        &self,
        org_id: &str,
        service_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/services/:service_id/logs",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/launch/services/:service_id/previews`
    pub async fn get_api_v1_orgs_by_org_id_launch_services_by_service_id_previews(
        &self,
        org_id: &str,
        service_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/launch/services/:service_id/previews",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/enforcement`
    pub async fn get_api_v1_orgs_by_org_id_limits_enforcement(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/enforcement",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/history`
    pub async fn get_api_v1_orgs_by_org_id_limits_history(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/history",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/overview`
    pub async fn get_api_v1_orgs_by_org_id_limits_overview(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/overview",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/projects`
    pub async fn get_api_v1_orgs_by_org_id_limits_projects(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/projects",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/requests`
    pub async fn get_api_v1_orgs_by_org_id_limits_requests(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/requests",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/resources`
    pub async fn get_api_v1_orgs_by_org_id_limits_resources(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/resources",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/limits/spend`
    pub async fn get_api_v1_orgs_by_org_id_limits_spend(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/limits/spend",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/members`
    pub async fn get_api_v1_orgs_by_org_id_members(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/members",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/metrics/compute`
    pub async fn get_api_v1_orgs_by_org_id_metrics_compute(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/metrics/compute",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/observability/alerts`
    pub async fn get_api_v1_orgs_by_org_id_observability_alerts(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/observability/alerts",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/observability/alerts/events`
    pub async fn get_api_v1_orgs_by_org_id_observability_alerts_events(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/observability/alerts/events",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/observability/destinations`
    pub async fn get_api_v1_orgs_by_org_id_observability_destinations(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/observability/destinations",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/projects`
    pub async fn get_api_v1_orgs_by_org_id_projects(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/projects",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/projects/:project_id`
    pub async fn get_api_v1_orgs_by_org_id_projects_by_project_id(
        &self,
        org_id: &str,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/projects/:project_id",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/projects/:project_id/billing/spend-limit`
    pub async fn get_api_v1_orgs_by_org_id_projects_by_project_id_billing_spend_limit(
        &self,
        org_id: &str,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/projects/:project_id/billing/spend-limit",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/projects/:project_id/metrics`
    pub async fn get_api_v1_orgs_by_org_id_projects_by_project_id_metrics(
        &self,
        org_id: &str,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/projects/:project_id/metrics",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/subscription`
    pub async fn get_api_v1_orgs_by_org_id_subscription(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/subscription",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/support/plan`
    pub async fn get_api_v1_orgs_by_org_id_support_plan(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/support/plan",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/tickets`
    pub async fn get_api_v1_orgs_by_org_id_tickets(
        &self,
        org_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/tickets",
            &[("org_id", org_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/tickets/:ticket_id`
    pub async fn get_api_v1_orgs_by_org_id_tickets_by_ticket_id(
        &self,
        org_id: &str,
        ticket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/tickets/:ticket_id",
            &[("org_id", org_id), ("ticket_id", ticket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/orgs/:org_id/tickets/:ticket_id/attachments/:attachment_id`
    pub async fn get_api_v1_orgs_by_org_id_tickets_by_ticket_id_attachments_by_attachment_id(
        &self,
        org_id: &str,
        ticket_id: &str,
        attachment_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/orgs/:org_id/tickets/:ticket_id/attachments/:attachment_id",
            &[
                ("org_id", org_id),
                ("ticket_id", ticket_id),
                ("attachment_id", attachment_id),
            ],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/platform/config`
    pub async fn get_api_v1_platform_config(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/platform/config",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/platform/health`
    pub async fn get_api_v1_platform_health(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/platform/health",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id`
    pub async fn get_api_v1_projects_by_project_id(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/costs`
    pub async fn get_api_v1_projects_by_project_id_backups_costs(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/costs",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/overview`
    pub async fn get_api_v1_projects_by_project_id_backups_overview(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/overview",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/policies`
    pub async fn get_api_v1_projects_by_project_id_backups_policies(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/policies",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/policies/:policy_id`
    pub async fn get_api_v1_projects_by_project_id_backups_policies_by_policy_id(
        &self,
        project_id: &str,
        policy_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/policies/:policy_id",
            &[("project_id", project_id), ("policy_id", policy_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/policies/:policy_id/runs`
    pub async fn get_api_v1_projects_by_project_id_backups_policies_by_policy_id_runs(
        &self,
        project_id: &str,
        policy_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/policies/:policy_id/runs",
            &[("project_id", project_id), ("policy_id", policy_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/backups/snapshots`
    pub async fn get_api_v1_projects_by_project_id_backups_snapshots(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/backups/snapshots",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/certificates`
    pub async fn get_api_v1_projects_by_project_id_certificates(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/certificates",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases`
    pub async fn get_api_v1_projects_by_project_id_databases(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/backups`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_backups(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/backups",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/config`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_config(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/config",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/dbs`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_dbs(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/dbs",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/firewall`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_firewall(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/firewall",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/logs`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_logs(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/logs",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/logs/download`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_logs_download(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/logs/download",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/pools`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_pools(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/pools",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/replicas`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_replicas(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/replicas",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/databases/:database_id/users`
    pub async fn get_api_v1_projects_by_project_id_databases_by_database_id_users(
        &self,
        project_id: &str,
        database_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/databases/:database_id/users",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/domains/:domain_id/auth-code`
    pub async fn get_api_v1_projects_by_project_id_domains_by_domain_id_auth_code(
        &self,
        project_id: &str,
        domain_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/domains/:domain_id/auth-code",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/domains/:domain_id/registrar-info`
    pub async fn get_api_v1_projects_by_project_id_domains_by_domain_id_registrar_info(
        &self,
        project_id: &str,
        domain_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/domains/:domain_id/registrar-info",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/domains/orders`
    pub async fn get_api_v1_projects_by_project_id_domains_orders(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/domains/orders",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/firewalls`
    pub async fn get_api_v1_projects_by_project_id_firewalls(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/firewalls",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/firewalls/:firewall_id`
    pub async fn get_api_v1_projects_by_project_id_firewalls_by_firewall_id(
        &self,
        project_id: &str,
        firewall_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/firewalls/:firewall_id",
            &[("project_id", project_id), ("firewall_id", firewall_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/floating-ips`
    pub async fn get_api_v1_projects_by_project_id_floating_ips(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/floating-ips",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/floating-ips/:eip_id`
    pub async fn get_api_v1_projects_by_project_id_floating_ips_by_eip_id(
        &self,
        project_id: &str,
        eip_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/floating-ips/:eip_id",
            &[("project_id", project_id), ("eip_id", eip_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/images`
    pub async fn get_api_v1_projects_by_project_id_images(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/images",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s`
    pub async fn get_api_v1_projects_by_project_id_k8s(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id/available-upgrades`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id_available_upgrades(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id/available-upgrades",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id/clusterlint`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id_clusterlint(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id/clusterlint",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id/deletion-resources`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id_deletion_resources(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id/deletion-resources",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id/kubeconfig`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id_kubeconfig(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id/kubeconfig",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/k8s/:cluster_id/status-messages`
    pub async fn get_api_v1_projects_by_project_id_k8s_by_cluster_id_status_messages(
        &self,
        project_id: &str,
        cluster_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/k8s/:cluster_id/status-messages",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/load-balancers`
    pub async fn get_api_v1_projects_by_project_id_load_balancers(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/load-balancers",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/load-balancers/:lb_id`
    pub async fn get_api_v1_projects_by_project_id_load_balancers_by_lb_id(
        &self,
        project_id: &str,
        lb_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/load-balancers/:lb_id",
            &[("project_id", project_id), ("lb_id", lb_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/load-balancers/:lb_id/listeners`
    pub async fn get_api_v1_projects_by_project_id_load_balancers_by_lb_id_listeners(
        &self,
        project_id: &str,
        lb_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/listeners",
            &[("project_id", project_id), ("lb_id", lb_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/load-balancers/:lb_id/target-groups`
    pub async fn get_api_v1_projects_by_project_id_load_balancers_by_lb_id_target_groups(
        &self,
        project_id: &str,
        lb_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/target-groups",
            &[("project_id", project_id), ("lb_id", lb_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/health`
    pub async fn get_api_v1_projects_by_project_id_load_balancers_by_lb_id_target_groups_by_tg_id_health(
        &self,
        project_id: &str,
        lb_id: &str,
        tg_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/health",
            &[
                ("project_id", project_id),
                ("lb_id", lb_id),
                ("tg_id", tg_id),
            ],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/nat-gateways`
    pub async fn get_api_v1_projects_by_project_id_nat_gateways(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/nat-gateways",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/nat-gateways/:nat_id`
    pub async fn get_api_v1_projects_by_project_id_nat_gateways_by_nat_id(
        &self,
        project_id: &str,
        nat_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/nat-gateways/:nat_id",
            &[("project_id", project_id), ("nat_id", nat_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/networks`
    pub async fn get_api_v1_projects_by_project_id_networks(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/networks",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/networks/:network_id`
    pub async fn get_api_v1_projects_by_project_id_networks_by_network_id(
        &self,
        project_id: &str,
        network_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/networks/:network_id",
            &[("project_id", project_id), ("network_id", network_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/networks/:network_id/subnets`
    pub async fn get_api_v1_projects_by_project_id_networks_by_network_id_subnets(
        &self,
        project_id: &str,
        network_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/networks/:network_id/subnets",
            &[("project_id", project_id), ("network_id", network_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/networks/:network_id/subnets/:subnet_id`
    pub async fn get_api_v1_projects_by_project_id_networks_by_network_id_subnets_by_subnet_id(
        &self,
        project_id: &str,
        network_id: &str,
        subnet_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/networks/:network_id/subnets/:subnet_id",
            &[
                ("project_id", project_id),
                ("network_id", network_id),
                ("subnet_id", subnet_id),
            ],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/scaling-groups`
    pub async fn get_api_v1_projects_by_project_id_scaling_groups(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/scaling-groups",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/scaling-groups/:group_id`
    pub async fn get_api_v1_projects_by_project_id_scaling_groups_by_group_id(
        &self,
        project_id: &str,
        group_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/scaling-groups/:group_id",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/scaling-groups/:group_id/activities`
    pub async fn get_api_v1_projects_by_project_id_scaling_groups_by_group_id_activities(
        &self,
        project_id: &str,
        group_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/scaling-groups/:group_id/activities",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/scaling-groups/:group_id/policies`
    pub async fn get_api_v1_projects_by_project_id_scaling_groups_by_group_id_policies(
        &self,
        project_id: &str,
        group_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/scaling-groups/:group_id/policies",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/security-groups`
    pub async fn get_api_v1_projects_by_project_id_security_groups(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/security-groups",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/security-groups/:sg_id`
    pub async fn get_api_v1_projects_by_project_id_security_groups_by_sg_id(
        &self,
        project_id: &str,
        sg_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/security-groups/:sg_id",
            &[("project_id", project_id), ("sg_id", sg_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/security-groups/:sg_id/rules`
    pub async fn get_api_v1_projects_by_project_id_security_groups_by_sg_id_rules(
        &self,
        project_id: &str,
        sg_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/security-groups/:sg_id/rules",
            &[("project_id", project_id), ("sg_id", sg_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/snapshots`
    pub async fn get_api_v1_projects_by_project_id_snapshots(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/snapshots",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/ssh_keys`
    pub async fn get_api_v1_projects_by_project_id_ssh_keys(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/ssh_keys",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/access-keys`
    pub async fn get_api_v1_projects_by_project_id_storage_access_keys(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/access-keys",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id/cdn`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_cdn(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/cdn",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id/cors`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_cors(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/cors",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_lifecycle(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id/objects`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_objects(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/objects",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/storage/buckets/:bucket_id/signed-download/*key`
    pub async fn get_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_signed_download_key(
        &self,
        project_id: &str,
        bucket_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/signed-download/*key",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/templates`
    pub async fn get_api_v1_projects_by_project_id_templates(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/templates",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/templates/:template_id`
    pub async fn get_api_v1_projects_by_project_id_templates_by_template_id(
        &self,
        project_id: &str,
        template_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/templates/:template_id",
            &[("project_id", project_id), ("template_id", template_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms`
    pub async fn get_api_v1_projects_by_project_id_vms(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/access`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_access(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/access",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/availability`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_availability(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/availability",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/cost`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_cost(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/cost",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/detail`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_detail(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/detail",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/disks`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_disks(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/disks",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/interfaces`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_interfaces(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/interfaces",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/logs`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_logs(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/logs",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/logs/download`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_logs_download(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/logs/download",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/metrics`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_metrics(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/metrics",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/resize-history`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_resize_history(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/resize-history",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/sessions`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_sessions(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/sessions",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/vms/:vm_id/state-transitions`
    pub async fn get_api_v1_projects_by_project_id_vms_by_vm_id_state_transitions(
        &self,
        project_id: &str,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/vms/:vm_id/state-transitions",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/volumes`
    pub async fn get_api_v1_projects_by_project_id_volumes(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/volumes",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/zones`
    pub async fn get_api_v1_projects_by_project_id_zones(
        &self,
        project_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/zones",
            &[("project_id", project_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/zones/:zone_id`
    pub async fn get_api_v1_projects_by_project_id_zones_by_zone_id(
        &self,
        project_id: &str,
        zone_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/zones/:zone_id",
            &[("project_id", project_id), ("zone_id", zone_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/projects/:project_id/zones/:zone_id/records`
    pub async fn get_api_v1_projects_by_project_id_zones_by_zone_id_records(
        &self,
        project_id: &str,
        zone_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/projects/:project_id/zones/:zone_id/records",
            &[("project_id", project_id), ("zone_id", zone_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /api/v1/status`
    pub async fn get_api_v1_status(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/api/v1/status",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /health`
    pub async fn get_health(&self, query: Option<&[(&str, &str)]>) -> Result<Value, NubisError> {
        self.request_value(Method::GET, "/health", &[], query, Option::<&Value>::None)
            .await
    }

    /// `GET /metrics`
    pub async fn get_metrics(&self, query: Option<&[(&str, &str)]>) -> Result<Value, NubisError> {
        self.request_value(Method::GET, "/metrics", &[], query, Option::<&Value>::None)
            .await
    }

    /// `GET /ws/realtime`
    pub async fn get_ws_realtime(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/ws/realtime",
            &[],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /ws/vms/:vm_id/console`
    pub async fn get_ws_vms_by_vm_id_console(
        &self,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/ws/vms/:vm_id/console",
            &[("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `GET /ws/vms/:vm_id/ssh`
    pub async fn get_ws_vms_by_vm_id_ssh(
        &self,
        vm_id: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::GET,
            "/ws/vms/:vm_id/ssh",
            &[("vm_id", vm_id)],
            query,
            Option::<&Value>::None,
        )
        .await
    }

    /// `PATCH /api/v1/orgs/:org_id/launch/services/:service_id/source`
    pub async fn patch_api_v1_orgs_by_org_id_launch_services_by_service_id_source<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        service_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PATCH,
            "/api/v1/orgs/:org_id/launch/services/:service_id/source",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            body,
        )
        .await
    }

    /// `PATCH /api/v1/orgs/:org_id/support`
    pub async fn patch_api_v1_orgs_by_org_id_support<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PATCH,
            "/api/v1/orgs/:org_id/support",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PATCH /api/v1/projects/:project_id/databases/:database_id/config`
    pub async fn patch_api_v1_projects_by_project_id_databases_by_database_id_config<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PATCH,
            "/api/v1/projects/:project_id/databases/:database_id/config",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `PATCH /api/v1/projects/:project_id/k8s/:cluster_id`
    pub async fn patch_api_v1_projects_by_project_id_k8s_by_cluster_id<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PATCH,
            "/api/v1/projects/:project_id/k8s/:cluster_id",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/admin/incidents`
    pub async fn post_api_v1_admin_incidents<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/admin/incidents", &[], query, body)
            .await
    }

    /// `POST /api/v1/admin/incidents/:id/resolve`
    pub async fn post_api_v1_admin_incidents_by_id_resolve<B: Serialize + ?Sized>(
        &self,
        id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/admin/incidents/:id/resolve",
            &[("id", id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/admin/incidents/:id/updates`
    pub async fn post_api_v1_admin_incidents_by_id_updates<B: Serialize + ?Sized>(
        &self,
        id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/admin/incidents/:id/updates",
            &[("id", id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/api-keys`
    pub async fn post_api_v1_api_keys<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/api-keys", &[], query, body)
            .await
    }

    /// `POST /api/v1/invitations/accept`
    pub async fn post_api_v1_invitations_accept<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/invitations/accept", &[], query, body)
            .await
    }

    /// `POST /api/v1/invitations/preview`
    pub async fn post_api_v1_invitations_preview<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/invitations/preview",
            &[],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/kyc/start`
    pub async fn post_api_v1_kyc_start<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/kyc/start", &[], query, body)
            .await
    }

    /// `POST /api/v1/logs/ingest`
    pub async fn post_api_v1_logs_ingest<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/logs/ingest", &[], query, body)
            .await
    }

    /// `POST /api/v1/logs/ingest/batch`
    pub async fn post_api_v1_logs_ingest_batch<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/logs/ingest/batch", &[], query, body)
            .await
    }

    /// `POST /api/v1/me/init`
    pub async fn post_api_v1_me_init<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/me/init", &[], query, body)
            .await
    }

    /// `POST /api/v1/metrics/ingest`
    pub async fn post_api_v1_metrics_ingest<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/metrics/ingest", &[], query, body)
            .await
    }

    /// `POST /api/v1/orgs`
    pub async fn post_api_v1_orgs<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/orgs", &[], query, body)
            .await
    }

    /// `POST /api/v1/orgs/:org_id/billing/credits/redeem`
    pub async fn post_api_v1_orgs_by_org_id_billing_credits_redeem<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/billing/credits/redeem",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/billing/payment-method`
    pub async fn post_api_v1_orgs_by_org_id_billing_payment_method<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/billing/payment-method",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/billing/payment-methods`
    pub async fn post_api_v1_orgs_by_org_id_billing_payment_methods<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/billing/payment-methods",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/billing/prepay`
    pub async fn post_api_v1_orgs_by_org_id_billing_prepay<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/billing/prepay",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/billing/verify-transaction`
    pub async fn post_api_v1_orgs_by_org_id_billing_verify_transaction<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/billing/verify-transaction",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/iac/webhooks`
    pub async fn post_api_v1_orgs_by_org_id_iac_webhooks<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/iac/webhooks",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/iam/service-accounts`
    pub async fn post_api_v1_orgs_by_org_id_iam_service_accounts<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/iam/service-accounts",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/iam/sso/verify`
    pub async fn post_api_v1_orgs_by_org_id_iam_sso_verify<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/iam/sso/verify",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/invoices/:invoice_id/pay`
    pub async fn post_api_v1_orgs_by_org_id_invoices_by_invoice_id_pay<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        invoice_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/invoices/:invoice_id/pay",
            &[("org_id", org_id), ("invoice_id", invoice_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/invoices/reissue-malformed`
    pub async fn post_api_v1_orgs_by_org_id_invoices_reissue_malformed<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/invoices/reissue-malformed",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/github/connect`
    pub async fn post_api_v1_orgs_by_org_id_launch_github_connect<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/github/connect",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/github/repositories/sync`
    pub async fn post_api_v1_orgs_by_org_id_launch_github_repositories_sync<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/github/repositories/sync",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/projects`
    pub async fn post_api_v1_orgs_by_org_id_launch_projects<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/projects",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/projects/:project_id/services`
    pub async fn post_api_v1_orgs_by_org_id_launch_projects_by_project_id_services<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/projects/:project_id/services",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/services/:service_id/deployments`
    pub async fn post_api_v1_orgs_by_org_id_launch_services_by_service_id_deployments<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        service_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/services/:service_id/deployments",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/services/:service_id/domains`
    pub async fn post_api_v1_orgs_by_org_id_launch_services_by_service_id_domains<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        service_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/services/:service_id/domains",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/services/:service_id/rollback`
    pub async fn post_api_v1_orgs_by_org_id_launch_services_by_service_id_rollback<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        service_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/services/:service_id/rollback",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/launch/services/:service_id/source/repair-webhook`
    pub async fn post_api_v1_orgs_by_org_id_launch_services_by_service_id_source_repair_webhook<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        service_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/launch/services/:service_id/source/repair-webhook",
            &[("org_id", org_id), ("service_id", service_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/members`
    pub async fn post_api_v1_orgs_by_org_id_members<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/members",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/observability/alerts`
    pub async fn post_api_v1_orgs_by_org_id_observability_alerts<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/observability/alerts",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/observability/destinations`
    pub async fn post_api_v1_orgs_by_org_id_observability_destinations<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/observability/destinations",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/observability/destinations/:destination_id/test`
    pub async fn post_api_v1_orgs_by_org_id_observability_destinations_by_destination_id_test<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        destination_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/observability/destinations/:destination_id/test",
            &[("org_id", org_id), ("destination_id", destination_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/phone/send-code`
    pub async fn post_api_v1_orgs_by_org_id_phone_send_code<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/phone/send-code",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/phone/verify`
    pub async fn post_api_v1_orgs_by_org_id_phone_verify<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/phone/verify",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/projects`
    pub async fn post_api_v1_orgs_by_org_id_projects<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/projects",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/projects/:project_id/infrastructure/apply`
    pub async fn post_api_v1_orgs_by_org_id_projects_by_project_id_infrastructure_apply<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/projects/:project_id/infrastructure/apply",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/subscription`
    pub async fn post_api_v1_orgs_by_org_id_subscription<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/subscription",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/support/upgrade`
    pub async fn post_api_v1_orgs_by_org_id_support_upgrade<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/support/upgrade",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/tickets`
    pub async fn post_api_v1_orgs_by_org_id_tickets<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/tickets",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/tickets/:ticket_id/attachments`
    pub async fn post_api_v1_orgs_by_org_id_tickets_by_ticket_id_attachments<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        ticket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/tickets/:ticket_id/attachments",
            &[("org_id", org_id), ("ticket_id", ticket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/tickets/:ticket_id/close`
    pub async fn post_api_v1_orgs_by_org_id_tickets_by_ticket_id_close<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        ticket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/tickets/:ticket_id/close",
            &[("org_id", org_id), ("ticket_id", ticket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/orgs/:org_id/tickets/:ticket_id/messages`
    pub async fn post_api_v1_orgs_by_org_id_tickets_by_ticket_id_messages<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        ticket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/orgs/:org_id/tickets/:ticket_id/messages",
            &[("org_id", org_id), ("ticket_id", ticket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/advisor/recommendation`
    pub async fn post_api_v1_projects_by_project_id_advisor_recommendation<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/advisor/recommendation",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/backups/policies`
    pub async fn post_api_v1_projects_by_project_id_backups_policies<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/backups/policies",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/backups/policies/:policy_id/pause`
    pub async fn post_api_v1_projects_by_project_id_backups_policies_by_policy_id_pause<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        policy_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/backups/policies/:policy_id/pause",
            &[("project_id", project_id), ("policy_id", policy_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/backups/policies/:policy_id/resume`
    pub async fn post_api_v1_projects_by_project_id_backups_policies_by_policy_id_resume<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        policy_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/backups/policies/:policy_id/resume",
            &[("project_id", project_id), ("policy_id", policy_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/backups/restore`
    pub async fn post_api_v1_projects_by_project_id_backups_restore<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/backups/restore",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/certificates`
    pub async fn post_api_v1_projects_by_project_id_certificates<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/certificates",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases`
    pub async fn post_api_v1_projects_by_project_id_databases<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/dbs`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_dbs<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/dbs",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/pools`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_pools<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/pools",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/replicas`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_replicas<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/replicas",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/resize`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_resize<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/resize",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/restore`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_restore<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/restore",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/users`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_users<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/users",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/databases/:database_id/users/:username/reset_auth`
    pub async fn post_api_v1_projects_by_project_id_databases_by_database_id_users_by_username_reset_auth<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        username: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/databases/:database_id/users/:username/reset_auth",
            &[
                ("project_id", project_id),
                ("database_id", database_id),
                ("username", username),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/:domain_id/pay`
    pub async fn post_api_v1_projects_by_project_id_domains_by_domain_id_pay<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/:domain_id/pay",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/:domain_id/recheck-registrar`
    pub async fn post_api_v1_projects_by_project_id_domains_by_domain_id_recheck_registrar<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/:domain_id/recheck-registrar",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/:domain_id/renew`
    pub async fn post_api_v1_projects_by_project_id_domains_by_domain_id_renew<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/:domain_id/renew",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/:domain_id/set-nameservers`
    pub async fn post_api_v1_projects_by_project_id_domains_by_domain_id_set_nameservers<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/:domain_id/set-nameservers",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/checkout`
    pub async fn post_api_v1_projects_by_project_id_domains_checkout<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/checkout",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/search`
    pub async fn post_api_v1_projects_by_project_id_domains_search<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/search",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/domains/verify-checkout`
    pub async fn post_api_v1_projects_by_project_id_domains_verify_checkout<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/domains/verify-checkout",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/firewalls`
    pub async fn post_api_v1_projects_by_project_id_firewalls<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/firewalls",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/firewalls/:firewall_id/attach/:vm_id`
    pub async fn post_api_v1_projects_by_project_id_firewalls_by_firewall_id_attach_by_vm_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        firewall_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/firewalls/:firewall_id/attach/:vm_id",
            &[
                ("project_id", project_id),
                ("firewall_id", firewall_id),
                ("vm_id", vm_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/firewalls/:firewall_id/detach/:vm_id`
    pub async fn post_api_v1_projects_by_project_id_firewalls_by_firewall_id_detach_by_vm_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        firewall_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/firewalls/:firewall_id/detach/:vm_id",
            &[
                ("project_id", project_id),
                ("firewall_id", firewall_id),
                ("vm_id", vm_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/firewalls/:firewall_id/rules`
    pub async fn post_api_v1_projects_by_project_id_firewalls_by_firewall_id_rules<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        firewall_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/firewalls/:firewall_id/rules",
            &[("project_id", project_id), ("firewall_id", firewall_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/floating-ips`
    pub async fn post_api_v1_projects_by_project_id_floating_ips<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/floating-ips",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/floating-ips/:eip_id/attach/:vm_id`
    pub async fn post_api_v1_projects_by_project_id_floating_ips_by_eip_id_attach_by_vm_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        eip_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/floating-ips/:eip_id/attach/:vm_id",
            &[
                ("project_id", project_id),
                ("eip_id", eip_id),
                ("vm_id", vm_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/floating-ips/:eip_id/detach`
    pub async fn post_api_v1_projects_by_project_id_floating_ips_by_eip_id_detach<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        eip_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/floating-ips/:eip_id/detach",
            &[("project_id", project_id), ("eip_id", eip_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/images/import-url`
    pub async fn post_api_v1_projects_by_project_id_images_import_url<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/images/import-url",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/k8s`
    pub async fn post_api_v1_projects_by_project_id_k8s<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/k8s",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/k8s/:cluster_id/clusterlint`
    pub async fn post_api_v1_projects_by_project_id_k8s_by_cluster_id_clusterlint<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/k8s/:cluster_id/clusterlint",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/k8s/:cluster_id/destroy-with-resources/dangerous`
    pub async fn post_api_v1_projects_by_project_id_k8s_by_cluster_id_destroy_with_resources_dangerous<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/k8s/:cluster_id/destroy-with-resources/dangerous",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/k8s/:cluster_id/destroy-with-resources/selective`
    pub async fn post_api_v1_projects_by_project_id_k8s_by_cluster_id_destroy_with_resources_selective<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/k8s/:cluster_id/destroy-with-resources/selective",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/k8s/:cluster_id/node_pools`
    pub async fn post_api_v1_projects_by_project_id_k8s_by_cluster_id_node_pools<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/k8s/:cluster_id/node_pools",
            &[("project_id", project_id), ("cluster_id", cluster_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/load-balancers`
    pub async fn post_api_v1_projects_by_project_id_load_balancers<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/load-balancers",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/targets`
    pub async fn post_api_v1_projects_by_project_id_load_balancers_by_lb_id_target_groups_by_tg_id_targets<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        lb_id: &str,
        tg_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/load-balancers/:lb_id/target-groups/:tg_id/targets",
            &[
                ("project_id", project_id),
                ("lb_id", lb_id),
                ("tg_id", tg_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/nat-gateways`
    pub async fn post_api_v1_projects_by_project_id_nat_gateways<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/nat-gateways",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/networks`
    pub async fn post_api_v1_projects_by_project_id_networks<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/networks",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/networks/:network_id/subnets`
    pub async fn post_api_v1_projects_by_project_id_networks_by_network_id_subnets<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        network_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/networks/:network_id/subnets",
            &[("project_id", project_id), ("network_id", network_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/scaling-groups`
    pub async fn post_api_v1_projects_by_project_id_scaling_groups<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/scaling-groups",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/scaling-groups/:group_id/policies`
    pub async fn post_api_v1_projects_by_project_id_scaling_groups_by_group_id_policies<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        group_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/scaling-groups/:group_id/policies",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/security-groups`
    pub async fn post_api_v1_projects_by_project_id_security_groups<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/security-groups",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/security-groups/:sg_id/attach/:vm_id`
    pub async fn post_api_v1_projects_by_project_id_security_groups_by_sg_id_attach_by_vm_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        sg_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/security-groups/:sg_id/attach/:vm_id",
            &[
                ("project_id", project_id),
                ("sg_id", sg_id),
                ("vm_id", vm_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/security-groups/:sg_id/detach/:vm_id`
    pub async fn post_api_v1_projects_by_project_id_security_groups_by_sg_id_detach_by_vm_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        sg_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/security-groups/:sg_id/detach/:vm_id",
            &[
                ("project_id", project_id),
                ("sg_id", sg_id),
                ("vm_id", vm_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/security-groups/:sg_id/rules`
    pub async fn post_api_v1_projects_by_project_id_security_groups_by_sg_id_rules<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        sg_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/security-groups/:sg_id/rules",
            &[("project_id", project_id), ("sg_id", sg_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/ssh_keys`
    pub async fn post_api_v1_projects_by_project_id_ssh_keys<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/ssh_keys",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/storage/access-keys`
    pub async fn post_api_v1_projects_by_project_id_storage_access_keys<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/storage/access-keys",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/storage/buckets`
    pub async fn post_api_v1_projects_by_project_id_storage_buckets<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/storage/buckets",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/storage/buckets/:bucket_id/cors`
    pub async fn post_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_cors<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/cors",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle`
    pub async fn post_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_lifecycle<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/lifecycle",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/storage/buckets/:bucket_id/objects/signed-upload`
    pub async fn post_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_objects_signed_upload<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/objects/signed-upload",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/templates`
    pub async fn post_api_v1_projects_by_project_id_templates<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/templates",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms`
    pub async fn post_api_v1_projects_by_project_id_vms<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/access/reset-password`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_access_reset_password<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/access/reset-password",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/console-session`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_console_session<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/console-session",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/disks`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_disks<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/disks",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/attach`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_disks_by_disk_id_attach<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        disk_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/attach",
            &[
                ("project_id", project_id),
                ("vm_id", vm_id),
                ("disk_id", disk_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/detach`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_disks_by_disk_id_detach<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        disk_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/detach",
            &[
                ("project_id", project_id),
                ("vm_id", vm_id),
                ("disk_id", disk_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/resize`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_disks_by_disk_id_resize<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        disk_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/disks/:disk_id/resize",
            &[
                ("project_id", project_id),
                ("vm_id", vm_id),
                ("disk_id", disk_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/reboot`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_reboot<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/reboot",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/refresh`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_refresh<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/refresh",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/rename`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_rename<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/rename",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/resize`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_resize<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/resize",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/snapshots`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_snapshots<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/snapshots",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/ssh-session`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_ssh_session<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/ssh-session",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/start`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_start<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/start",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/vms/:vm_id/stop`
    pub async fn post_api_v1_projects_by_project_id_vms_by_vm_id_stop<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/vms/:vm_id/stop",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/volumes`
    pub async fn post_api_v1_projects_by_project_id_volumes<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/volumes",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/zones`
    pub async fn post_api_v1_projects_by_project_id_zones<B: Serialize + ?Sized>(
        &self,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/zones",
            &[("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/zones/:zone_id/recheck-dns`
    pub async fn post_api_v1_projects_by_project_id_zones_by_zone_id_recheck_dns<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        zone_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/zones/:zone_id/recheck-dns",
            &[("project_id", project_id), ("zone_id", zone_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/projects/:project_id/zones/:zone_id/records`
    pub async fn post_api_v1_projects_by_project_id_zones_by_zone_id_records<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        zone_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/projects/:project_id/zones/:zone_id/records",
            &[("project_id", project_id), ("zone_id", zone_id)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/webhooks/github/launch/:delivery_key`
    pub async fn post_api_v1_webhooks_github_launch_by_delivery_key<B: Serialize + ?Sized>(
        &self,
        delivery_key: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::POST,
            "/api/v1/webhooks/github/launch/:delivery_key",
            &[("delivery_key", delivery_key)],
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/webhooks/intercom`
    pub async fn post_api_v1_webhooks_intercom<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/webhooks/intercom", &[], query, body)
            .await
    }

    /// `POST /api/v1/webhooks/paystack`
    pub async fn post_api_v1_webhooks_paystack<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/webhooks/paystack", &[], query, body)
            .await
    }

    /// `POST /api/v1/webhooks/polar`
    pub async fn post_api_v1_webhooks_polar<B: Serialize + ?Sized>(
        &self,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(Method::POST, "/api/v1/webhooks/polar", &[], query, body)
            .await
    }

    /// `PUT /api/v1/orgs/:org_id/billing/enforcement-settings`
    pub async fn put_api_v1_orgs_by_org_id_billing_enforcement_settings<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/billing/enforcement-settings",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/billing/payment-methods/:method_id/default`
    pub async fn put_api_v1_orgs_by_org_id_billing_payment_methods_by_method_id_default<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        method_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/billing/payment-methods/:method_id/default",
            &[("org_id", org_id), ("method_id", method_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/billing/spend-limit`
    pub async fn put_api_v1_orgs_by_org_id_billing_spend_limit<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/billing/spend-limit",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/billing/tax-info`
    pub async fn put_api_v1_orgs_by_org_id_billing_tax_info<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/billing/tax-info",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/currency`
    pub async fn put_api_v1_orgs_by_org_id_currency<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/currency",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/iac/webhooks/:webhook_id`
    pub async fn put_api_v1_orgs_by_org_id_iac_webhooks_by_webhook_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        webhook_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/iac/webhooks/:webhook_id",
            &[("org_id", org_id), ("webhook_id", webhook_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/iam/roles/:role_id/permissions`
    pub async fn put_api_v1_orgs_by_org_id_iam_roles_by_role_id_permissions<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        role_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/iam/roles/:role_id/permissions",
            &[("org_id", org_id), ("role_id", role_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/iam/service-accounts/:sa_id`
    pub async fn put_api_v1_orgs_by_org_id_iam_service_accounts_by_sa_id<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        sa_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/iam/service-accounts/:sa_id",
            &[("org_id", org_id), ("sa_id", sa_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/iam/sso`
    pub async fn put_api_v1_orgs_by_org_id_iam_sso<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/iam/sso",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/limits/spend`
    pub async fn put_api_v1_orgs_by_org_id_limits_spend<B: Serialize + ?Sized>(
        &self,
        org_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/limits/spend",
            &[("org_id", org_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/observability/alerts/:policy_id`
    pub async fn put_api_v1_orgs_by_org_id_observability_alerts_by_policy_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        policy_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/observability/alerts/:policy_id",
            &[("org_id", org_id), ("policy_id", policy_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/observability/destinations/:destination_id`
    pub async fn put_api_v1_orgs_by_org_id_observability_destinations_by_destination_id<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        destination_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/observability/destinations/:destination_id",
            &[("org_id", org_id), ("destination_id", destination_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/projects/:project_id/billing/spend-limit`
    pub async fn put_api_v1_orgs_by_org_id_projects_by_project_id_billing_spend_limit<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/projects/:project_id/billing/spend-limit",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/orgs/:org_id/projects/:project_id/compliance`
    pub async fn put_api_v1_orgs_by_org_id_projects_by_project_id_compliance<
        B: Serialize + ?Sized,
    >(
        &self,
        org_id: &str,
        project_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/orgs/:org_id/projects/:project_id/compliance",
            &[("org_id", org_id), ("project_id", project_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/databases/:database_id/firewall`
    pub async fn put_api_v1_projects_by_project_id_databases_by_database_id_firewall<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/databases/:database_id/firewall",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/databases/:database_id/maintenance`
    pub async fn put_api_v1_projects_by_project_id_databases_by_database_id_maintenance<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        database_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/databases/:database_id/maintenance",
            &[("project_id", project_id), ("database_id", database_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/domains/:domain_id/auto-renew`
    pub async fn put_api_v1_projects_by_project_id_domains_by_domain_id_auto_renew<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/domains/:domain_id/auto-renew",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/domains/:domain_id/contact`
    pub async fn put_api_v1_projects_by_project_id_domains_by_domain_id_contact<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/domains/:domain_id/contact",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/domains/:domain_id/lock`
    pub async fn put_api_v1_projects_by_project_id_domains_by_domain_id_lock<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/domains/:domain_id/lock",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/domains/:domain_id/privacy`
    pub async fn put_api_v1_projects_by_project_id_domains_by_domain_id_privacy<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        domain_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/domains/:domain_id/privacy",
            &[("project_id", project_id), ("domain_id", domain_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/k8s/:cluster_id/node_pools/:pool_id`
    pub async fn put_api_v1_projects_by_project_id_k8s_by_cluster_id_node_pools_by_pool_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        cluster_id: &str,
        pool_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/k8s/:cluster_id/node_pools/:pool_id",
            &[
                ("project_id", project_id),
                ("cluster_id", cluster_id),
                ("pool_id", pool_id),
            ],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/scaling-groups/:group_id/desired`
    pub async fn put_api_v1_projects_by_project_id_scaling_groups_by_group_id_desired<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        group_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/scaling-groups/:group_id/desired",
            &[("project_id", project_id), ("group_id", group_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/storage/buckets/:bucket_id`
    pub async fn put_api_v1_projects_by_project_id_storage_buckets_by_bucket_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/storage/buckets/:bucket_id/cdn`
    pub async fn put_api_v1_projects_by_project_id_storage_buckets_by_bucket_id_cdn<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        bucket_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/storage/buckets/:bucket_id/cdn",
            &[("project_id", project_id), ("bucket_id", bucket_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/vms/:vm_id/access/ssh-key`
    pub async fn put_api_v1_projects_by_project_id_vms_by_vm_id_access_ssh_key<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        vm_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/vms/:vm_id/access/ssh-key",
            &[("project_id", project_id), ("vm_id", vm_id)],
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/projects/:project_id/zones/:zone_id/records/:record_id`
    pub async fn put_api_v1_projects_by_project_id_zones_by_zone_id_records_by_record_id<
        B: Serialize + ?Sized,
    >(
        &self,
        project_id: &str,
        zone_id: &str,
        record_id: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {
        self.request_value(
            Method::PUT,
            "/api/v1/projects/:project_id/zones/:zone_id/records/:record_id",
            &[
                ("project_id", project_id),
                ("zone_id", zone_id),
                ("record_id", record_id),
            ],
            query,
            body,
        )
        .await
    }
}
