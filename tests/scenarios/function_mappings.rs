use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;
use datadog_api_client::datadogV1;
use datadog_api_client::datadogV2;

#[derive(Debug, Default)]
pub struct ApiInstances {
    pub v1_api_ip_ranges: Option<datadogV1::api::api_ip_ranges::IPRangesAPI>,
    pub v1_api_key_management: Option<datadogV1::api::api_key_management::KeyManagementAPI>,
    pub v1_api_service_checks: Option<datadogV1::api::api_service_checks::ServiceChecksAPI>,
    pub v1_api_usage_metering: Option<datadogV1::api::api_usage_metering::UsageMeteringAPI>,
    pub v1_api_dashboards: Option<datadogV1::api::api_dashboards::DashboardsAPI>,
    pub v1_api_dashboard_lists: Option<datadogV1::api::api_dashboard_lists::DashboardListsAPI>,
    pub v1_api_metrics: Option<datadogV1::api::api_metrics::MetricsAPI>,
    pub v1_api_downtimes: Option<datadogV1::api::api_downtimes::DowntimesAPI>,
    pub v1_api_events: Option<datadogV1::api::api_events::EventsAPI>,
    pub v1_api_snapshots: Option<datadogV1::api::api_snapshots::SnapshotsAPI>,
    pub v1_api_hosts: Option<datadogV1::api::api_hosts::HostsAPI>,
    pub v1_api_aws_integration: Option<datadogV1::api::api_aws_integration::AWSIntegrationAPI>,
    pub v1_api_aws_logs_integration: Option<datadogV1::api::api_aws_logs_integration::AWSLogsIntegrationAPI>,
    pub v1_api_azure_integration: Option<datadogV1::api::api_azure_integration::AzureIntegrationAPI>,
    pub v1_api_gcp_integration: Option<datadogV1::api::api_gcp_integration::GCPIntegrationAPI>,
    pub v1_api_pager_duty_integration: Option<datadogV1::api::api_pager_duty_integration::PagerDutyIntegrationAPI>,
    pub v1_api_slack_integration: Option<datadogV1::api::api_slack_integration::SlackIntegrationAPI>,
    pub v1_api_webhooks_integration: Option<datadogV1::api::api_webhooks_integration::WebhooksIntegrationAPI>,
    pub v1_api_logs: Option<datadogV1::api::api_logs::LogsAPI>,
    pub v1_api_logs_indexes: Option<datadogV1::api::api_logs_indexes::LogsIndexesAPI>,
    pub v1_api_logs_pipelines: Option<datadogV1::api::api_logs_pipelines::LogsPipelinesAPI>,
    pub v1_api_monitors: Option<datadogV1::api::api_monitors::MonitorsAPI>,
    pub v1_api_notebooks: Option<datadogV1::api::api_notebooks::NotebooksAPI>,
    pub v1_api_organizations: Option<datadogV1::api::api_organizations::OrganizationsAPI>,
    pub v1_api_security_monitoring: Option<datadogV1::api::api_security_monitoring::SecurityMonitoringAPI>,
    pub v1_api_service_level_objectives: Option<datadogV1::api::api_service_level_objectives::ServiceLevelObjectivesAPI>,
    pub v1_api_service_level_objective_corrections: Option<datadogV1::api::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI>,
    pub v1_api_synthetics: Option<datadogV1::api::api_synthetics::SyntheticsAPI>,
    pub v1_api_tags: Option<datadogV1::api::api_tags::TagsAPI>,
    pub v1_api_users: Option<datadogV1::api::api_users::UsersAPI>,
    pub v1_api_authentication: Option<datadogV1::api::api_authentication::AuthenticationAPI>,
    pub v2_api_key_management: Option<datadogV2::api::api_key_management::KeyManagementAPI>,
    pub v2_api_spans_metrics: Option<datadogV2::api::api_spans_metrics::SpansMetricsAPI>,
    pub v2_api_apm_retention_filters: Option<datadogV2::api::api_apm_retention_filters::APMRetentionFiltersAPI>,
    pub v2_api_audit: Option<datadogV2::api::api_audit::AuditAPI>,
    pub v2_api_auth_n_mappings: Option<datadogV2::api::api_auth_n_mappings::AuthNMappingsAPI>,
    pub v2_api_ci_visibility_pipelines: Option<datadogV2::api::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI>,
    pub v2_api_ci_visibility_tests: Option<datadogV2::api::api_ci_visibility_tests::CIVisibilityTestsAPI>,
    pub v2_api_container_images: Option<datadogV2::api::api_container_images::ContainerImagesAPI>,
    pub v2_api_containers: Option<datadogV2::api::api_containers::ContainersAPI>,
    pub v2_api_cloud_cost_management: Option<datadogV2::api::api_cloud_cost_management::CloudCostManagementAPI>,
    pub v2_api_usage_metering: Option<datadogV2::api::api_usage_metering::UsageMeteringAPI>,
    pub v2_api_dashboard_lists: Option<datadogV2::api::api_dashboard_lists::DashboardListsAPI>,
    pub v2_api_dora_metrics: Option<datadogV2::api::api_dora_metrics::DORAMetricsAPI>,
    pub v2_api_downtimes: Option<datadogV2::api::api_downtimes::DowntimesAPI>,
    pub v2_api_events: Option<datadogV2::api::api_events::EventsAPI>,
    pub v2_api_incidents: Option<datadogV2::api::api_incidents::IncidentsAPI>,
    pub v2_api_gcp_integration: Option<datadogV2::api::api_gcp_integration::GCPIntegrationAPI>,
    pub v2_api_opsgenie_integration: Option<datadogV2::api::api_opsgenie_integration::OpsgenieIntegrationAPI>,
    pub v2_api_cloudflare_integration: Option<datadogV2::api::api_cloudflare_integration::CloudflareIntegrationAPI>,
    pub v2_api_confluent_cloud: Option<datadogV2::api::api_confluent_cloud::ConfluentCloudAPI>,
    pub v2_api_fastly_integration: Option<datadogV2::api::api_fastly_integration::FastlyIntegrationAPI>,
    pub v2_api_okta_integration: Option<datadogV2::api::api_okta_integration::OktaIntegrationAPI>,
    pub v2_api_ip_allowlist: Option<datadogV2::api::api_ip_allowlist::IPAllowlistAPI>,
    pub v2_api_logs: Option<datadogV2::api::api_logs::LogsAPI>,
    pub v2_api_logs_archives: Option<datadogV2::api::api_logs_archives::LogsArchivesAPI>,
    pub v2_api_logs_metrics: Option<datadogV2::api::api_logs_metrics::LogsMetricsAPI>,
    pub v2_api_metrics: Option<datadogV2::api::api_metrics::MetricsAPI>,
    pub v2_api_monitors: Option<datadogV2::api::api_monitors::MonitorsAPI>,
    pub v2_api_roles: Option<datadogV2::api::api_roles::RolesAPI>,
    pub v2_api_security_monitoring: Option<datadogV2::api::api_security_monitoring::SecurityMonitoringAPI>,
    pub v2_api_powerpack: Option<datadogV2::api::api_powerpack::PowerpackAPI>,
    pub v2_api_processes: Option<datadogV2::api::api_processes::ProcessesAPI>,
    pub v2_api_restriction_policies: Option<datadogV2::api::api_restriction_policies::RestrictionPoliciesAPI>,
    pub v2_api_rum: Option<datadogV2::api::api_rum::RUMAPI>,
    pub v2_api_organizations: Option<datadogV2::api::api_organizations::OrganizationsAPI>,
    pub v2_api_service_scorecards: Option<datadogV2::api::api_service_scorecards::ServiceScorecardsAPI>,
    pub v2_api_cloud_workload_security: Option<datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI>,
    pub v2_api_sensitive_data_scanner: Option<datadogV2::api::api_sensitive_data_scanner::SensitiveDataScannerAPI>,
    pub v2_api_service_accounts: Option<datadogV2::api::api_service_accounts::ServiceAccountsAPI>,
    pub v2_api_incident_services: Option<datadogV2::api::api_incident_services::IncidentServicesAPI>,
    pub v2_api_service_definition: Option<datadogV2::api::api_service_definition::ServiceDefinitionAPI>,
    pub v2_api_spans: Option<datadogV2::api::api_spans::SpansAPI>,
    pub v2_api_synthetics: Option<datadogV2::api::api_synthetics::SyntheticsAPI>,
    pub v2_api_teams: Option<datadogV2::api::api_teams::TeamsAPI>,
    pub v2_api_incident_teams: Option<datadogV2::api::api_incident_teams::IncidentTeamsAPI>,
    pub v2_api_users: Option<datadogV2::api::api_users::UsersAPI>,
}

pub fn initialize_api_instance(world: &mut DatadogWorld, api: String) {
    match api.as_str() {
        "IPRanges" => {
            if world.api_instances.v1_api_ip_ranges.is_none() {
                world.api_instances.v1_api_ip_ranges = Some(
                    datadogV1::api::api_ip_ranges::IPRangesAPI::with_config(world.config.clone()),
                );
            }
        }
        "KeyManagement" => {
            if world.api_instances.v1_api_key_management.is_none() {
                world.api_instances.v1_api_key_management = Some(
                    datadogV1::api::api_key_management::KeyManagementAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_key_management.is_none() {
                world.api_instances.v2_api_key_management = Some(
                    datadogV2::api::api_key_management::KeyManagementAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "ServiceChecks" => {
            if world.api_instances.v1_api_service_checks.is_none() {
                world.api_instances.v1_api_service_checks = Some(
                    datadogV1::api::api_service_checks::ServiceChecksAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "UsageMetering" => {
            if world.api_instances.v1_api_usage_metering.is_none() {
                world.api_instances.v1_api_usage_metering = Some(
                    datadogV1::api::api_usage_metering::UsageMeteringAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_usage_metering.is_none() {
                world.api_instances.v2_api_usage_metering = Some(
                    datadogV2::api::api_usage_metering::UsageMeteringAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Dashboards" => {
            if world.api_instances.v1_api_dashboards.is_none() {
                world.api_instances.v1_api_dashboards =
                    Some(datadogV1::api::api_dashboards::DashboardsAPI::with_config(
                        world.config.clone(),
                    ));
            }
        }
        "DashboardLists" => {
            if world.api_instances.v1_api_dashboard_lists.is_none() {
                world.api_instances.v1_api_dashboard_lists = Some(
                    datadogV1::api::api_dashboard_lists::DashboardListsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_dashboard_lists.is_none() {
                world.api_instances.v2_api_dashboard_lists = Some(
                    datadogV2::api::api_dashboard_lists::DashboardListsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Metrics" => {
            if world.api_instances.v1_api_metrics.is_none() {
                world.api_instances.v1_api_metrics = Some(
                    datadogV1::api::api_metrics::MetricsAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_metrics.is_none() {
                world.api_instances.v2_api_metrics = Some(
                    datadogV2::api::api_metrics::MetricsAPI::with_config(world.config.clone()),
                );
            }
        }
        "Downtimes" => {
            if world.api_instances.v1_api_downtimes.is_none() {
                world.api_instances.v1_api_downtimes = Some(
                    datadogV1::api::api_downtimes::DowntimesAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_downtimes.is_none() {
                world.api_instances.v2_api_downtimes = Some(
                    datadogV2::api::api_downtimes::DowntimesAPI::with_config(world.config.clone()),
                );
            }
        }
        "Events" => {
            if world.api_instances.v1_api_events.is_none() {
                world.api_instances.v1_api_events = Some(
                    datadogV1::api::api_events::EventsAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_events.is_none() {
                world.api_instances.v2_api_events = Some(
                    datadogV2::api::api_events::EventsAPI::with_config(world.config.clone()),
                );
            }
        }
        "Snapshots" => {
            if world.api_instances.v1_api_snapshots.is_none() {
                world.api_instances.v1_api_snapshots = Some(
                    datadogV1::api::api_snapshots::SnapshotsAPI::with_config(world.config.clone()),
                );
            }
        }
        "Hosts" => {
            if world.api_instances.v1_api_hosts.is_none() {
                world.api_instances.v1_api_hosts = Some(
                    datadogV1::api::api_hosts::HostsAPI::with_config(world.config.clone()),
                );
            }
        }
        "AWSIntegration" => {
            if world.api_instances.v1_api_aws_integration.is_none() {
                world.api_instances.v1_api_aws_integration = Some(
                    datadogV1::api::api_aws_integration::AWSIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "AWSLogsIntegration" => {
            if world.api_instances.v1_api_aws_logs_integration.is_none() {
                world.api_instances.v1_api_aws_logs_integration = Some(
                    datadogV1::api::api_aws_logs_integration::AWSLogsIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "AzureIntegration" => {
            if world.api_instances.v1_api_azure_integration.is_none() {
                world.api_instances.v1_api_azure_integration = Some(
                    datadogV1::api::api_azure_integration::AzureIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "GCPIntegration" => {
            if world.api_instances.v1_api_gcp_integration.is_none() {
                world.api_instances.v1_api_gcp_integration = Some(
                    datadogV1::api::api_gcp_integration::GCPIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_gcp_integration.is_none() {
                world.api_instances.v2_api_gcp_integration = Some(
                    datadogV2::api::api_gcp_integration::GCPIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "PagerDutyIntegration" => {
            if world.api_instances.v1_api_pager_duty_integration.is_none() {
                world.api_instances.v1_api_pager_duty_integration = Some(datadogV1::api::api_pager_duty_integration::PagerDutyIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "SlackIntegration" => {
            if world.api_instances.v1_api_slack_integration.is_none() {
                world.api_instances.v1_api_slack_integration = Some(
                    datadogV1::api::api_slack_integration::SlackIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "WebhooksIntegration" => {
            if world.api_instances.v1_api_webhooks_integration.is_none() {
                world.api_instances.v1_api_webhooks_integration = Some(
                    datadogV1::api::api_webhooks_integration::WebhooksIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Logs" => {
            if world.api_instances.v1_api_logs.is_none() {
                world.api_instances.v1_api_logs = Some(
                    datadogV1::api::api_logs::LogsAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_logs.is_none() {
                world.api_instances.v2_api_logs = Some(
                    datadogV2::api::api_logs::LogsAPI::with_config(world.config.clone()),
                );
            }
        }
        "LogsIndexes" => {
            if world.api_instances.v1_api_logs_indexes.is_none() {
                world.api_instances.v1_api_logs_indexes = Some(
                    datadogV1::api::api_logs_indexes::LogsIndexesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "LogsPipelines" => {
            if world.api_instances.v1_api_logs_pipelines.is_none() {
                world.api_instances.v1_api_logs_pipelines = Some(
                    datadogV1::api::api_logs_pipelines::LogsPipelinesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Monitors" => {
            if world.api_instances.v1_api_monitors.is_none() {
                world.api_instances.v1_api_monitors = Some(
                    datadogV1::api::api_monitors::MonitorsAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_monitors.is_none() {
                world.api_instances.v2_api_monitors = Some(
                    datadogV2::api::api_monitors::MonitorsAPI::with_config(world.config.clone()),
                );
            }
        }
        "Notebooks" => {
            if world.api_instances.v1_api_notebooks.is_none() {
                world.api_instances.v1_api_notebooks = Some(
                    datadogV1::api::api_notebooks::NotebooksAPI::with_config(world.config.clone()),
                );
            }
        }
        "Organizations" => {
            if world.api_instances.v1_api_organizations.is_none() {
                world.api_instances.v1_api_organizations = Some(
                    datadogV1::api::api_organizations::OrganizationsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_organizations.is_none() {
                world.api_instances.v2_api_organizations = Some(
                    datadogV2::api::api_organizations::OrganizationsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "SecurityMonitoring" => {
            if world.api_instances.v1_api_security_monitoring.is_none() {
                world.api_instances.v1_api_security_monitoring = Some(
                    datadogV1::api::api_security_monitoring::SecurityMonitoringAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_security_monitoring.is_none() {
                world.api_instances.v2_api_security_monitoring = Some(
                    datadogV2::api::api_security_monitoring::SecurityMonitoringAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "ServiceLevelObjectives" => {
            if world
                .api_instances
                .v1_api_service_level_objectives
                .is_none()
            {
                world.api_instances.v1_api_service_level_objectives = Some(datadogV1::api::api_service_level_objectives::ServiceLevelObjectivesAPI::with_config(world.config.clone()));
            }
        }
        "ServiceLevelObjectiveCorrections" => {
            if world
                .api_instances
                .v1_api_service_level_objective_corrections
                .is_none()
            {
                world.api_instances.v1_api_service_level_objective_corrections = Some(datadogV1::api::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI::with_config(world.config.clone()));
            }
        }
        "Synthetics" => {
            if world.api_instances.v1_api_synthetics.is_none() {
                world.api_instances.v1_api_synthetics =
                    Some(datadogV1::api::api_synthetics::SyntheticsAPI::with_config(
                        world.config.clone(),
                    ));
            }
            if world.api_instances.v2_api_synthetics.is_none() {
                world.api_instances.v2_api_synthetics =
                    Some(datadogV2::api::api_synthetics::SyntheticsAPI::with_config(
                        world.config.clone(),
                    ));
            }
        }
        "Tags" => {
            if world.api_instances.v1_api_tags.is_none() {
                world.api_instances.v1_api_tags = Some(
                    datadogV1::api::api_tags::TagsAPI::with_config(world.config.clone()),
                );
            }
        }
        "Users" => {
            if world.api_instances.v1_api_users.is_none() {
                world.api_instances.v1_api_users = Some(
                    datadogV1::api::api_users::UsersAPI::with_config(world.config.clone()),
                );
            }
            if world.api_instances.v2_api_users.is_none() {
                world.api_instances.v2_api_users = Some(
                    datadogV2::api::api_users::UsersAPI::with_config(world.config.clone()),
                );
            }
        }
        "Authentication" => {
            if world.api_instances.v1_api_authentication.is_none() {
                world.api_instances.v1_api_authentication = Some(
                    datadogV1::api::api_authentication::AuthenticationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "SpansMetrics" => {
            if world.api_instances.v2_api_spans_metrics.is_none() {
                world.api_instances.v2_api_spans_metrics = Some(
                    datadogV2::api::api_spans_metrics::SpansMetricsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "APMRetentionFilters" => {
            if world.api_instances.v2_api_apm_retention_filters.is_none() {
                world.api_instances.v2_api_apm_retention_filters = Some(
                    datadogV2::api::api_apm_retention_filters::APMRetentionFiltersAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Audit" => {
            if world.api_instances.v2_api_audit.is_none() {
                world.api_instances.v2_api_audit = Some(
                    datadogV2::api::api_audit::AuditAPI::with_config(world.config.clone()),
                );
            }
        }
        "AuthNMappings" => {
            if world.api_instances.v2_api_auth_n_mappings.is_none() {
                world.api_instances.v2_api_auth_n_mappings = Some(
                    datadogV2::api::api_auth_n_mappings::AuthNMappingsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "CIVisibilityPipelines" => {
            if world.api_instances.v2_api_ci_visibility_pipelines.is_none() {
                world.api_instances.v2_api_ci_visibility_pipelines = Some(datadogV2::api::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI::with_config(world.config.clone()));
            }
        }
        "CIVisibilityTests" => {
            if world.api_instances.v2_api_ci_visibility_tests.is_none() {
                world.api_instances.v2_api_ci_visibility_tests = Some(
                    datadogV2::api::api_ci_visibility_tests::CIVisibilityTestsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "ContainerImages" => {
            if world.api_instances.v2_api_container_images.is_none() {
                world.api_instances.v2_api_container_images = Some(
                    datadogV2::api::api_container_images::ContainerImagesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Containers" => {
            if world.api_instances.v2_api_containers.is_none() {
                world.api_instances.v2_api_containers =
                    Some(datadogV2::api::api_containers::ContainersAPI::with_config(
                        world.config.clone(),
                    ));
            }
        }
        "CloudCostManagement" => {
            if world.api_instances.v2_api_cloud_cost_management.is_none() {
                world.api_instances.v2_api_cloud_cost_management = Some(
                    datadogV2::api::api_cloud_cost_management::CloudCostManagementAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "DORAMetrics" => {
            if world.api_instances.v2_api_dora_metrics.is_none() {
                world.api_instances.v2_api_dora_metrics = Some(
                    datadogV2::api::api_dora_metrics::DORAMetricsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Incidents" => {
            if world.api_instances.v2_api_incidents.is_none() {
                world.api_instances.v2_api_incidents = Some(
                    datadogV2::api::api_incidents::IncidentsAPI::with_config(world.config.clone()),
                );
            }
        }
        "OpsgenieIntegration" => {
            if world.api_instances.v2_api_opsgenie_integration.is_none() {
                world.api_instances.v2_api_opsgenie_integration = Some(
                    datadogV2::api::api_opsgenie_integration::OpsgenieIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "CloudflareIntegration" => {
            if world.api_instances.v2_api_cloudflare_integration.is_none() {
                world.api_instances.v2_api_cloudflare_integration = Some(datadogV2::api::api_cloudflare_integration::CloudflareIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "ConfluentCloud" => {
            if world.api_instances.v2_api_confluent_cloud.is_none() {
                world.api_instances.v2_api_confluent_cloud = Some(
                    datadogV2::api::api_confluent_cloud::ConfluentCloudAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "FastlyIntegration" => {
            if world.api_instances.v2_api_fastly_integration.is_none() {
                world.api_instances.v2_api_fastly_integration = Some(
                    datadogV2::api::api_fastly_integration::FastlyIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "OktaIntegration" => {
            if world.api_instances.v2_api_okta_integration.is_none() {
                world.api_instances.v2_api_okta_integration = Some(
                    datadogV2::api::api_okta_integration::OktaIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "IPAllowlist" => {
            if world.api_instances.v2_api_ip_allowlist.is_none() {
                world.api_instances.v2_api_ip_allowlist = Some(
                    datadogV2::api::api_ip_allowlist::IPAllowlistAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "LogsArchives" => {
            if world.api_instances.v2_api_logs_archives.is_none() {
                world.api_instances.v2_api_logs_archives = Some(
                    datadogV2::api::api_logs_archives::LogsArchivesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "LogsMetrics" => {
            if world.api_instances.v2_api_logs_metrics.is_none() {
                world.api_instances.v2_api_logs_metrics = Some(
                    datadogV2::api::api_logs_metrics::LogsMetricsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Roles" => {
            if world.api_instances.v2_api_roles.is_none() {
                world.api_instances.v2_api_roles = Some(
                    datadogV2::api::api_roles::RolesAPI::with_config(world.config.clone()),
                );
            }
        }
        "Powerpack" => {
            if world.api_instances.v2_api_powerpack.is_none() {
                world.api_instances.v2_api_powerpack = Some(
                    datadogV2::api::api_powerpack::PowerpackAPI::with_config(world.config.clone()),
                );
            }
        }
        "Processes" => {
            if world.api_instances.v2_api_processes.is_none() {
                world.api_instances.v2_api_processes = Some(
                    datadogV2::api::api_processes::ProcessesAPI::with_config(world.config.clone()),
                );
            }
        }
        "RestrictionPolicies" => {
            if world.api_instances.v2_api_restriction_policies.is_none() {
                world.api_instances.v2_api_restriction_policies = Some(
                    datadogV2::api::api_restriction_policies::RestrictionPoliciesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "RUM" => {
            if world.api_instances.v2_api_rum.is_none() {
                world.api_instances.v2_api_rum = Some(
                    datadogV2::api::api_rum::RUMAPI::with_config(world.config.clone()),
                );
            }
        }
        "ServiceScorecards" => {
            if world.api_instances.v2_api_service_scorecards.is_none() {
                world.api_instances.v2_api_service_scorecards = Some(
                    datadogV2::api::api_service_scorecards::ServiceScorecardsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "CloudWorkloadSecurity" => {
            if world.api_instances.v2_api_cloud_workload_security.is_none() {
                world.api_instances.v2_api_cloud_workload_security = Some(datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI::with_config(world.config.clone()));
            }
        }
        "SensitiveDataScanner" => {
            if world.api_instances.v2_api_sensitive_data_scanner.is_none() {
                world.api_instances.v2_api_sensitive_data_scanner = Some(datadogV2::api::api_sensitive_data_scanner::SensitiveDataScannerAPI::with_config(world.config.clone()));
            }
        }
        "ServiceAccounts" => {
            if world.api_instances.v2_api_service_accounts.is_none() {
                world.api_instances.v2_api_service_accounts = Some(
                    datadogV2::api::api_service_accounts::ServiceAccountsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "IncidentServices" => {
            if world.api_instances.v2_api_incident_services.is_none() {
                world.api_instances.v2_api_incident_services = Some(
                    datadogV2::api::api_incident_services::IncidentServicesAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "ServiceDefinition" => {
            if world.api_instances.v2_api_service_definition.is_none() {
                world.api_instances.v2_api_service_definition = Some(
                    datadogV2::api::api_service_definition::ServiceDefinitionAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "Spans" => {
            if world.api_instances.v2_api_spans.is_none() {
                world.api_instances.v2_api_spans = Some(
                    datadogV2::api::api_spans::SpansAPI::with_config(world.config.clone()),
                );
            }
        }
        "Teams" => {
            if world.api_instances.v2_api_teams.is_none() {
                world.api_instances.v2_api_teams = Some(
                    datadogV2::api::api_teams::TeamsAPI::with_config(world.config.clone()),
                );
            }
        }
        "IncidentTeams" => {
            if world.api_instances.v2_api_incident_teams.is_none() {
                world.api_instances.v2_api_incident_teams = Some(
                    datadogV2::api::api_incident_teams::IncidentTeamsAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        _ => panic!("{api} API instance not found"),
    }
}

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("v1.GetIPRanges".to_string(), test_v1_get_ip_ranges);
    world
        .function_mappings
        .insert("v1.ListAPIKeys".to_string(), test_v1_list_api_keys);
    world
        .function_mappings
        .insert("v1.CreateAPIKey".to_string(), test_v1_create_api_key);
    world
        .function_mappings
        .insert("v1.DeleteAPIKey".to_string(), test_v1_delete_api_key);
    world
        .function_mappings
        .insert("v1.GetAPIKey".to_string(), test_v1_get_api_key);
    world
        .function_mappings
        .insert("v1.UpdateAPIKey".to_string(), test_v1_update_api_key);
    world.function_mappings.insert(
        "v1.ListApplicationKeys".to_string(),
        test_v1_list_application_keys,
    );
    world.function_mappings.insert(
        "v1.CreateApplicationKey".to_string(),
        test_v1_create_application_key,
    );
    world.function_mappings.insert(
        "v1.DeleteApplicationKey".to_string(),
        test_v1_delete_application_key,
    );
    world.function_mappings.insert(
        "v1.GetApplicationKey".to_string(),
        test_v1_get_application_key,
    );
    world.function_mappings.insert(
        "v1.UpdateApplicationKey".to_string(),
        test_v1_update_application_key,
    );
    world.function_mappings.insert(
        "v1.SubmitServiceCheck".to_string(),
        test_v1_submit_service_check,
    );
    world.function_mappings.insert(
        "v1.GetDailyCustomReports".to_string(),
        test_v1_get_daily_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetSpecifiedDailyCustomReports".to_string(),
        test_v1_get_specified_daily_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetMonthlyCustomReports".to_string(),
        test_v1_get_monthly_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetSpecifiedMonthlyCustomReports".to_string(),
        test_v1_get_specified_monthly_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetUsageAnalyzedLogs".to_string(),
        test_v1_get_usage_analyzed_logs,
    );
    world.function_mappings.insert(
        "v1.GetUsageAttribution".to_string(),
        test_v1_get_usage_attribution,
    );
    world.function_mappings.insert(
        "v1.GetUsageAuditLogs".to_string(),
        test_v1_get_usage_audit_logs,
    );
    world
        .function_mappings
        .insert("v1.GetUsageLambda".to_string(), test_v1_get_usage_lambda);
    world.function_mappings.insert(
        "v1.GetUsageBillableSummary".to_string(),
        test_v1_get_usage_billable_summary,
    );
    world
        .function_mappings
        .insert("v1.GetUsageCIApp".to_string(), test_v1_get_usage_ci_app);
    world.function_mappings.insert(
        "v1.GetUsageCloudSecurityPostureManagement".to_string(),
        test_v1_get_usage_cloud_security_posture_management,
    );
    world
        .function_mappings
        .insert("v1.GetUsageCWS".to_string(), test_v1_get_usage_cws);
    world
        .function_mappings
        .insert("v1.GetUsageDBM".to_string(), test_v1_get_usage_dbm);
    world
        .function_mappings
        .insert("v1.GetUsageFargate".to_string(), test_v1_get_usage_fargate);
    world
        .function_mappings
        .insert("v1.GetUsageHosts".to_string(), test_v1_get_usage_hosts);
    world.function_mappings.insert(
        "v1.GetHourlyUsageAttribution".to_string(),
        test_v1_get_hourly_usage_attribution,
    );
    world.function_mappings.insert(
        "v1.GetIncidentManagement".to_string(),
        test_v1_get_incident_management,
    );
    world.function_mappings.insert(
        "v1.GetUsageIndexedSpans".to_string(),
        test_v1_get_usage_indexed_spans,
    );
    world.function_mappings.insert(
        "v1.GetIngestedSpans".to_string(),
        test_v1_get_ingested_spans,
    );
    world.function_mappings.insert(
        "v1.GetUsageInternetOfThings".to_string(),
        test_v1_get_usage_internet_of_things,
    );
    world
        .function_mappings
        .insert("v1.GetUsageLogs".to_string(), test_v1_get_usage_logs);
    world.function_mappings.insert(
        "v1.GetUsageLogsByRetention".to_string(),
        test_v1_get_usage_logs_by_retention,
    );
    world.function_mappings.insert(
        "v1.GetUsageLogsByIndex".to_string(),
        test_v1_get_usage_logs_by_index,
    );
    world.function_mappings.insert(
        "v1.GetMonthlyUsageAttribution".to_string(),
        test_v1_get_monthly_usage_attribution,
    );
    world.function_mappings.insert(
        "v1.GetUsageNetworkFlows".to_string(),
        test_v1_get_usage_network_flows,
    );
    world.function_mappings.insert(
        "v1.GetUsageNetworkHosts".to_string(),
        test_v1_get_usage_network_hosts,
    );
    world.function_mappings.insert(
        "v1.GetUsageOnlineArchive".to_string(),
        test_v1_get_usage_online_archive,
    );
    world.function_mappings.insert(
        "v1.GetUsageProfiling".to_string(),
        test_v1_get_usage_profiling,
    );
    world.function_mappings.insert(
        "v1.GetUsageRumUnits".to_string(),
        test_v1_get_usage_rum_units,
    );
    world.function_mappings.insert(
        "v1.GetUsageRumSessions".to_string(),
        test_v1_get_usage_rum_sessions,
    );
    world
        .function_mappings
        .insert("v1.GetUsageSDS".to_string(), test_v1_get_usage_sds);
    world
        .function_mappings
        .insert("v1.GetUsageSNMP".to_string(), test_v1_get_usage_snmp);
    world
        .function_mappings
        .insert("v1.GetUsageSummary".to_string(), test_v1_get_usage_summary);
    world.function_mappings.insert(
        "v1.GetUsageSynthetics".to_string(),
        test_v1_get_usage_synthetics,
    );
    world.function_mappings.insert(
        "v1.GetUsageSyntheticsAPI".to_string(),
        test_v1_get_usage_synthetics_api,
    );
    world.function_mappings.insert(
        "v1.GetUsageSyntheticsBrowser".to_string(),
        test_v1_get_usage_synthetics_browser,
    );
    world.function_mappings.insert(
        "v1.GetUsageTimeseries".to_string(),
        test_v1_get_usage_timeseries,
    );
    world.function_mappings.insert(
        "v1.GetUsageTopAvgMetrics".to_string(),
        test_v1_get_usage_top_avg_metrics,
    );
    world
        .function_mappings
        .insert("v1.DeleteDashboards".to_string(), test_v1_delete_dashboards);
    world
        .function_mappings
        .insert("v1.ListDashboards".to_string(), test_v1_list_dashboards);
    world.function_mappings.insert(
        "v1.RestoreDashboards".to_string(),
        test_v1_restore_dashboards,
    );
    world
        .function_mappings
        .insert("v1.CreateDashboard".to_string(), test_v1_create_dashboard);
    world.function_mappings.insert(
        "v1.CreatePublicDashboard".to_string(),
        test_v1_create_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.DeletePublicDashboard".to_string(),
        test_v1_delete_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.GetPublicDashboard".to_string(),
        test_v1_get_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.UpdatePublicDashboard".to_string(),
        test_v1_update_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.DeletePublicDashboardInvitation".to_string(),
        test_v1_delete_public_dashboard_invitation,
    );
    world.function_mappings.insert(
        "v1.GetPublicDashboardInvitations".to_string(),
        test_v1_get_public_dashboard_invitations,
    );
    world.function_mappings.insert(
        "v1.SendPublicDashboardInvitation".to_string(),
        test_v1_send_public_dashboard_invitation,
    );
    world
        .function_mappings
        .insert("v1.DeleteDashboard".to_string(), test_v1_delete_dashboard);
    world
        .function_mappings
        .insert("v1.GetDashboard".to_string(), test_v1_get_dashboard);
    world
        .function_mappings
        .insert("v1.UpdateDashboard".to_string(), test_v1_update_dashboard);
    world.function_mappings.insert(
        "v1.ListDashboardLists".to_string(),
        test_v1_list_dashboard_lists,
    );
    world.function_mappings.insert(
        "v1.CreateDashboardList".to_string(),
        test_v1_create_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.DeleteDashboardList".to_string(),
        test_v1_delete_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.GetDashboardList".to_string(),
        test_v1_get_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.UpdateDashboardList".to_string(),
        test_v1_update_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.SubmitDistributionPoints".to_string(),
        test_v1_submit_distribution_points,
    );
    world.function_mappings.insert(
        "v1.ListActiveMetrics".to_string(),
        test_v1_list_active_metrics,
    );
    world.function_mappings.insert(
        "v1.GetMetricMetadata".to_string(),
        test_v1_get_metric_metadata,
    );
    world.function_mappings.insert(
        "v1.UpdateMetricMetadata".to_string(),
        test_v1_update_metric_metadata,
    );
    world
        .function_mappings
        .insert("v1.QueryMetrics".to_string(), test_v1_query_metrics);
    world
        .function_mappings
        .insert("v1.ListMetrics".to_string(), test_v1_list_metrics);
    world
        .function_mappings
        .insert("v1.SubmitMetrics".to_string(), test_v1_submit_metrics);
    world
        .function_mappings
        .insert("v1.ListDowntimes".to_string(), test_v1_list_downtimes);
    world
        .function_mappings
        .insert("v1.CreateDowntime".to_string(), test_v1_create_downtime);
    world.function_mappings.insert(
        "v1.CancelDowntimesByScope".to_string(),
        test_v1_cancel_downtimes_by_scope,
    );
    world
        .function_mappings
        .insert("v1.CancelDowntime".to_string(), test_v1_cancel_downtime);
    world
        .function_mappings
        .insert("v1.GetDowntime".to_string(), test_v1_get_downtime);
    world
        .function_mappings
        .insert("v1.UpdateDowntime".to_string(), test_v1_update_downtime);
    world.function_mappings.insert(
        "v1.ListMonitorDowntimes".to_string(),
        test_v1_list_monitor_downtimes,
    );
    world
        .function_mappings
        .insert("v1.ListEvents".to_string(), test_v1_list_events);
    world
        .function_mappings
        .insert("v1.CreateEvent".to_string(), test_v1_create_event);
    world
        .function_mappings
        .insert("v1.GetEvent".to_string(), test_v1_get_event);
    world.function_mappings.insert(
        "v1.GetGraphSnapshot".to_string(),
        test_v1_get_graph_snapshot,
    );
    world
        .function_mappings
        .insert("v1.MuteHost".to_string(), test_v1_mute_host);
    world
        .function_mappings
        .insert("v1.UnmuteHost".to_string(), test_v1_unmute_host);
    world
        .function_mappings
        .insert("v1.ListHosts".to_string(), test_v1_list_hosts);
    world
        .function_mappings
        .insert("v1.GetHostTotals".to_string(), test_v1_get_host_totals);
    world.function_mappings.insert(
        "v1.DeleteAWSAccount".to_string(),
        test_v1_delete_aws_account,
    );
    world
        .function_mappings
        .insert("v1.ListAWSAccounts".to_string(), test_v1_list_aws_accounts);
    world.function_mappings.insert(
        "v1.CreateAWSAccount".to_string(),
        test_v1_create_aws_account,
    );
    world.function_mappings.insert(
        "v1.UpdateAWSAccount".to_string(),
        test_v1_update_aws_account,
    );
    world.function_mappings.insert(
        "v1.ListAvailableAWSNamespaces".to_string(),
        test_v1_list_available_aws_namespaces,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSEventBridgeSource".to_string(),
        test_v1_delete_aws_event_bridge_source,
    );
    world.function_mappings.insert(
        "v1.ListAWSEventBridgeSources".to_string(),
        test_v1_list_aws_event_bridge_sources,
    );
    world.function_mappings.insert(
        "v1.CreateAWSEventBridgeSource".to_string(),
        test_v1_create_aws_event_bridge_source,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSTagFilter".to_string(),
        test_v1_delete_aws_tag_filter,
    );
    world.function_mappings.insert(
        "v1.ListAWSTagFilters".to_string(),
        test_v1_list_aws_tag_filters,
    );
    world.function_mappings.insert(
        "v1.CreateAWSTagFilter".to_string(),
        test_v1_create_aws_tag_filter,
    );
    world.function_mappings.insert(
        "v1.CreateNewAWSExternalID".to_string(),
        test_v1_create_new_aws_external_id,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSLambdaARN".to_string(),
        test_v1_delete_aws_lambda_arn,
    );
    world.function_mappings.insert(
        "v1.ListAWSLogsIntegrations".to_string(),
        test_v1_list_aws_logs_integrations,
    );
    world.function_mappings.insert(
        "v1.CreateAWSLambdaARN".to_string(),
        test_v1_create_aws_lambda_arn,
    );
    world.function_mappings.insert(
        "v1.CheckAWSLogsLambdaAsync".to_string(),
        test_v1_check_aws_logs_lambda_async,
    );
    world.function_mappings.insert(
        "v1.ListAWSLogsServices".to_string(),
        test_v1_list_aws_logs_services,
    );
    world.function_mappings.insert(
        "v1.EnableAWSLogServices".to_string(),
        test_v1_enable_aws_log_services,
    );
    world.function_mappings.insert(
        "v1.CheckAWSLogsServicesAsync".to_string(),
        test_v1_check_aws_logs_services_async,
    );
    world.function_mappings.insert(
        "v1.DeleteAzureIntegration".to_string(),
        test_v1_delete_azure_integration,
    );
    world.function_mappings.insert(
        "v1.ListAzureIntegration".to_string(),
        test_v1_list_azure_integration,
    );
    world.function_mappings.insert(
        "v1.CreateAzureIntegration".to_string(),
        test_v1_create_azure_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateAzureIntegration".to_string(),
        test_v1_update_azure_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateAzureHostFilters".to_string(),
        test_v1_update_azure_host_filters,
    );
    world.function_mappings.insert(
        "v1.DeleteGCPIntegration".to_string(),
        test_v1_delete_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.ListGCPIntegration".to_string(),
        test_v1_list_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.CreateGCPIntegration".to_string(),
        test_v1_create_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateGCPIntegration".to_string(),
        test_v1_update_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.CreatePagerDutyIntegrationService".to_string(),
        test_v1_create_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.DeletePagerDutyIntegrationService".to_string(),
        test_v1_delete_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.GetPagerDutyIntegrationService".to_string(),
        test_v1_get_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.UpdatePagerDutyIntegrationService".to_string(),
        test_v1_update_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.GetSlackIntegrationChannels".to_string(),
        test_v1_get_slack_integration_channels,
    );
    world.function_mappings.insert(
        "v1.CreateSlackIntegrationChannel".to_string(),
        test_v1_create_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.RemoveSlackIntegrationChannel".to_string(),
        test_v1_remove_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.GetSlackIntegrationChannel".to_string(),
        test_v1_get_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.UpdateSlackIntegrationChannel".to_string(),
        test_v1_update_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.CreateWebhooksIntegrationCustomVariable".to_string(),
        test_v1_create_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.DeleteWebhooksIntegrationCustomVariable".to_string(),
        test_v1_delete_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.GetWebhooksIntegrationCustomVariable".to_string(),
        test_v1_get_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.UpdateWebhooksIntegrationCustomVariable".to_string(),
        test_v1_update_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.CreateWebhooksIntegration".to_string(),
        test_v1_create_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.DeleteWebhooksIntegration".to_string(),
        test_v1_delete_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.GetWebhooksIntegration".to_string(),
        test_v1_get_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateWebhooksIntegration".to_string(),
        test_v1_update_webhooks_integration,
    );
    world
        .function_mappings
        .insert("v1.ListLogs".to_string(), test_v1_list_logs);
    world
        .function_mappings
        .insert("v1.SubmitLog".to_string(), test_v1_submit_log);
    world.function_mappings.insert(
        "v1.GetLogsIndexOrder".to_string(),
        test_v1_get_logs_index_order,
    );
    world.function_mappings.insert(
        "v1.UpdateLogsIndexOrder".to_string(),
        test_v1_update_logs_index_order,
    );
    world
        .function_mappings
        .insert("v1.ListLogIndexes".to_string(), test_v1_list_log_indexes);
    world
        .function_mappings
        .insert("v1.CreateLogsIndex".to_string(), test_v1_create_logs_index);
    world
        .function_mappings
        .insert("v1.GetLogsIndex".to_string(), test_v1_get_logs_index);
    world
        .function_mappings
        .insert("v1.UpdateLogsIndex".to_string(), test_v1_update_logs_index);
    world.function_mappings.insert(
        "v1.GetLogsPipelineOrder".to_string(),
        test_v1_get_logs_pipeline_order,
    );
    world.function_mappings.insert(
        "v1.UpdateLogsPipelineOrder".to_string(),
        test_v1_update_logs_pipeline_order,
    );
    world.function_mappings.insert(
        "v1.ListLogsPipelines".to_string(),
        test_v1_list_logs_pipelines,
    );
    world.function_mappings.insert(
        "v1.CreateLogsPipeline".to_string(),
        test_v1_create_logs_pipeline,
    );
    world.function_mappings.insert(
        "v1.DeleteLogsPipeline".to_string(),
        test_v1_delete_logs_pipeline,
    );
    world
        .function_mappings
        .insert("v1.GetLogsPipeline".to_string(), test_v1_get_logs_pipeline);
    world.function_mappings.insert(
        "v1.UpdateLogsPipeline".to_string(),
        test_v1_update_logs_pipeline,
    );
    world
        .function_mappings
        .insert("v1.ListMonitors".to_string(), test_v1_list_monitors);
    world
        .function_mappings
        .insert("v1.CreateMonitor".to_string(), test_v1_create_monitor);
    world.function_mappings.insert(
        "v1.CheckCanDeleteMonitor".to_string(),
        test_v1_check_can_delete_monitor,
    );
    world.function_mappings.insert(
        "v1.SearchMonitorGroups".to_string(),
        test_v1_search_monitor_groups,
    );
    world
        .function_mappings
        .insert("v1.SearchMonitors".to_string(), test_v1_search_monitors);
    world
        .function_mappings
        .insert("v1.ValidateMonitor".to_string(), test_v1_validate_monitor);
    world
        .function_mappings
        .insert("v1.DeleteMonitor".to_string(), test_v1_delete_monitor);
    world
        .function_mappings
        .insert("v1.GetMonitor".to_string(), test_v1_get_monitor);
    world
        .function_mappings
        .insert("v1.UpdateMonitor".to_string(), test_v1_update_monitor);
    world.function_mappings.insert(
        "v1.ValidateExistingMonitor".to_string(),
        test_v1_validate_existing_monitor,
    );
    world
        .function_mappings
        .insert("v1.ListNotebooks".to_string(), test_v1_list_notebooks);
    world
        .function_mappings
        .insert("v1.CreateNotebook".to_string(), test_v1_create_notebook);
    world
        .function_mappings
        .insert("v1.DeleteNotebook".to_string(), test_v1_delete_notebook);
    world
        .function_mappings
        .insert("v1.GetNotebook".to_string(), test_v1_get_notebook);
    world
        .function_mappings
        .insert("v1.UpdateNotebook".to_string(), test_v1_update_notebook);
    world
        .function_mappings
        .insert("v1.ListOrgs".to_string(), test_v1_list_orgs);
    world
        .function_mappings
        .insert("v1.CreateChildOrg".to_string(), test_v1_create_child_org);
    world
        .function_mappings
        .insert("v1.GetOrg".to_string(), test_v1_get_org);
    world
        .function_mappings
        .insert("v1.UpdateOrg".to_string(), test_v1_update_org);
    world
        .function_mappings
        .insert("v1.DowngradeOrg".to_string(), test_v1_downgrade_org);
    world.function_mappings.insert(
        "v1.UploadIdPForOrg".to_string(),
        test_v1_upload_id_p_for_org,
    );
    world.function_mappings.insert(
        "v1.AddSecurityMonitoringSignalToIncident".to_string(),
        test_v1_add_security_monitoring_signal_to_incident,
    );
    world.function_mappings.insert(
        "v1.EditSecurityMonitoringSignalAssignee".to_string(),
        test_v1_edit_security_monitoring_signal_assignee,
    );
    world.function_mappings.insert(
        "v1.EditSecurityMonitoringSignalState".to_string(),
        test_v1_edit_security_monitoring_signal_state,
    );
    world
        .function_mappings
        .insert("v1.ListSLOs".to_string(), test_v1_list_sl_os);
    world
        .function_mappings
        .insert("v1.CreateSLO".to_string(), test_v1_create_slo);
    world.function_mappings.insert(
        "v1.DeleteSLOTimeframeInBulk".to_string(),
        test_v1_delete_slo_timeframe_in_bulk,
    );
    world.function_mappings.insert(
        "v1.CheckCanDeleteSLO".to_string(),
        test_v1_check_can_delete_slo,
    );
    world
        .function_mappings
        .insert("v1.SearchSLO".to_string(), test_v1_search_slo);
    world
        .function_mappings
        .insert("v1.DeleteSLO".to_string(), test_v1_delete_slo);
    world
        .function_mappings
        .insert("v1.GetSLO".to_string(), test_v1_get_slo);
    world
        .function_mappings
        .insert("v1.UpdateSLO".to_string(), test_v1_update_slo);
    world.function_mappings.insert(
        "v1.GetSLOCorrections".to_string(),
        test_v1_get_slo_corrections,
    );
    world
        .function_mappings
        .insert("v1.GetSLOHistory".to_string(), test_v1_get_slo_history);
    world.function_mappings.insert(
        "v1.ListSLOCorrection".to_string(),
        test_v1_list_slo_correction,
    );
    world.function_mappings.insert(
        "v1.CreateSLOCorrection".to_string(),
        test_v1_create_slo_correction,
    );
    world.function_mappings.insert(
        "v1.DeleteSLOCorrection".to_string(),
        test_v1_delete_slo_correction,
    );
    world.function_mappings.insert(
        "v1.GetSLOCorrection".to_string(),
        test_v1_get_slo_correction,
    );
    world.function_mappings.insert(
        "v1.UpdateSLOCorrection".to_string(),
        test_v1_update_slo_correction,
    );
    world.function_mappings.insert(
        "v1.GetSyntheticsCIBatch".to_string(),
        test_v1_get_synthetics_ci_batch,
    );
    world
        .function_mappings
        .insert("v1.ListLocations".to_string(), test_v1_list_locations);
    world.function_mappings.insert(
        "v1.CreatePrivateLocation".to_string(),
        test_v1_create_private_location,
    );
    world.function_mappings.insert(
        "v1.DeletePrivateLocation".to_string(),
        test_v1_delete_private_location,
    );
    world.function_mappings.insert(
        "v1.GetPrivateLocation".to_string(),
        test_v1_get_private_location,
    );
    world.function_mappings.insert(
        "v1.UpdatePrivateLocation".to_string(),
        test_v1_update_private_location,
    );
    world.function_mappings.insert(
        "v1.GetSyntheticsDefaultLocations".to_string(),
        test_v1_get_synthetics_default_locations,
    );
    world
        .function_mappings
        .insert("v1.ListTests".to_string(), test_v1_list_tests);
    world.function_mappings.insert(
        "v1.CreateSyntheticsAPITest".to_string(),
        test_v1_create_synthetics_api_test,
    );
    world
        .function_mappings
        .insert("v1.GetAPITest".to_string(), test_v1_get_api_test);
    world
        .function_mappings
        .insert("v1.UpdateAPITest".to_string(), test_v1_update_api_test);
    world.function_mappings.insert(
        "v1.CreateSyntheticsBrowserTest".to_string(),
        test_v1_create_synthetics_browser_test,
    );
    world
        .function_mappings
        .insert("v1.GetBrowserTest".to_string(), test_v1_get_browser_test);
    world.function_mappings.insert(
        "v1.UpdateBrowserTest".to_string(),
        test_v1_update_browser_test,
    );
    world.function_mappings.insert(
        "v1.GetBrowserTestLatestResults".to_string(),
        test_v1_get_browser_test_latest_results,
    );
    world.function_mappings.insert(
        "v1.GetBrowserTestResult".to_string(),
        test_v1_get_browser_test_result,
    );
    world
        .function_mappings
        .insert("v1.DeleteTests".to_string(), test_v1_delete_tests);
    world
        .function_mappings
        .insert("v1.TriggerTests".to_string(), test_v1_trigger_tests);
    world
        .function_mappings
        .insert("v1.TriggerCITests".to_string(), test_v1_trigger_ci_tests);
    world
        .function_mappings
        .insert("v1.GetTest".to_string(), test_v1_get_test);
    world
        .function_mappings
        .insert("v1.PatchTest".to_string(), test_v1_patch_test);
    world.function_mappings.insert(
        "v1.GetAPITestLatestResults".to_string(),
        test_v1_get_api_test_latest_results,
    );
    world.function_mappings.insert(
        "v1.GetAPITestResult".to_string(),
        test_v1_get_api_test_result,
    );
    world.function_mappings.insert(
        "v1.UpdateTestPauseStatus".to_string(),
        test_v1_update_test_pause_status,
    );
    world.function_mappings.insert(
        "v1.ListGlobalVariables".to_string(),
        test_v1_list_global_variables,
    );
    world.function_mappings.insert(
        "v1.CreateGlobalVariable".to_string(),
        test_v1_create_global_variable,
    );
    world.function_mappings.insert(
        "v1.DeleteGlobalVariable".to_string(),
        test_v1_delete_global_variable,
    );
    world.function_mappings.insert(
        "v1.GetGlobalVariable".to_string(),
        test_v1_get_global_variable,
    );
    world.function_mappings.insert(
        "v1.EditGlobalVariable".to_string(),
        test_v1_edit_global_variable,
    );
    world
        .function_mappings
        .insert("v1.ListHostTags".to_string(), test_v1_list_host_tags);
    world
        .function_mappings
        .insert("v1.DeleteHostTags".to_string(), test_v1_delete_host_tags);
    world
        .function_mappings
        .insert("v1.GetHostTags".to_string(), test_v1_get_host_tags);
    world
        .function_mappings
        .insert("v1.CreateHostTags".to_string(), test_v1_create_host_tags);
    world
        .function_mappings
        .insert("v1.UpdateHostTags".to_string(), test_v1_update_host_tags);
    world
        .function_mappings
        .insert("v1.ListUsers".to_string(), test_v1_list_users);
    world
        .function_mappings
        .insert("v1.CreateUser".to_string(), test_v1_create_user);
    world
        .function_mappings
        .insert("v1.DisableUser".to_string(), test_v1_disable_user);
    world
        .function_mappings
        .insert("v1.GetUser".to_string(), test_v1_get_user);
    world
        .function_mappings
        .insert("v1.UpdateUser".to_string(), test_v1_update_user);
    world
        .function_mappings
        .insert("v1.Validate".to_string(), test_v1_validate);
    world
        .function_mappings
        .insert("v2.ListAPIKeys".to_string(), test_v2_list_api_keys);
    world
        .function_mappings
        .insert("v2.CreateAPIKey".to_string(), test_v2_create_api_key);
    world
        .function_mappings
        .insert("v2.DeleteAPIKey".to_string(), test_v2_delete_api_key);
    world
        .function_mappings
        .insert("v2.GetAPIKey".to_string(), test_v2_get_api_key);
    world
        .function_mappings
        .insert("v2.UpdateAPIKey".to_string(), test_v2_update_api_key);
    world.function_mappings.insert(
        "v2.ListApplicationKeys".to_string(),
        test_v2_list_application_keys,
    );
    world.function_mappings.insert(
        "v2.DeleteApplicationKey".to_string(),
        test_v2_delete_application_key,
    );
    world.function_mappings.insert(
        "v2.GetApplicationKey".to_string(),
        test_v2_get_application_key,
    );
    world.function_mappings.insert(
        "v2.UpdateApplicationKey".to_string(),
        test_v2_update_application_key,
    );
    world.function_mappings.insert(
        "v2.ListCurrentUserApplicationKeys".to_string(),
        test_v2_list_current_user_application_keys,
    );
    world.function_mappings.insert(
        "v2.CreateCurrentUserApplicationKey".to_string(),
        test_v2_create_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.DeleteCurrentUserApplicationKey".to_string(),
        test_v2_delete_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.GetCurrentUserApplicationKey".to_string(),
        test_v2_get_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.UpdateCurrentUserApplicationKey".to_string(),
        test_v2_update_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.ListSpansMetrics".to_string(),
        test_v2_list_spans_metrics,
    );
    world.function_mappings.insert(
        "v2.CreateSpansMetric".to_string(),
        test_v2_create_spans_metric,
    );
    world.function_mappings.insert(
        "v2.DeleteSpansMetric".to_string(),
        test_v2_delete_spans_metric,
    );
    world
        .function_mappings
        .insert("v2.GetSpansMetric".to_string(), test_v2_get_spans_metric);
    world.function_mappings.insert(
        "v2.UpdateSpansMetric".to_string(),
        test_v2_update_spans_metric,
    );
    world.function_mappings.insert(
        "v2.ListApmRetentionFilters".to_string(),
        test_v2_list_apm_retention_filters,
    );
    world.function_mappings.insert(
        "v2.CreateApmRetentionFilter".to_string(),
        test_v2_create_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.ReorderApmRetentionFilters".to_string(),
        test_v2_reorder_apm_retention_filters,
    );
    world.function_mappings.insert(
        "v2.DeleteApmRetentionFilter".to_string(),
        test_v2_delete_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.GetApmRetentionFilter".to_string(),
        test_v2_get_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.UpdateApmRetentionFilter".to_string(),
        test_v2_update_apm_retention_filter,
    );
    world
        .function_mappings
        .insert("v2.ListAuditLogs".to_string(), test_v2_list_audit_logs);
    world
        .function_mappings
        .insert("v2.SearchAuditLogs".to_string(), test_v2_search_audit_logs);
    world.function_mappings.insert(
        "v2.ListAuthNMappings".to_string(),
        test_v2_list_auth_n_mappings,
    );
    world.function_mappings.insert(
        "v2.CreateAuthNMapping".to_string(),
        test_v2_create_auth_n_mapping,
    );
    world.function_mappings.insert(
        "v2.DeleteAuthNMapping".to_string(),
        test_v2_delete_auth_n_mapping,
    );
    world
        .function_mappings
        .insert("v2.GetAuthNMapping".to_string(), test_v2_get_auth_n_mapping);
    world.function_mappings.insert(
        "v2.UpdateAuthNMapping".to_string(),
        test_v2_update_auth_n_mapping,
    );
    world.function_mappings.insert(
        "v2.CreateCIAppPipelineEvent".to_string(),
        test_v2_create_ci_app_pipeline_event,
    );
    world.function_mappings.insert(
        "v2.AggregateCIAppPipelineEvents".to_string(),
        test_v2_aggregate_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppPipelineEvents".to_string(),
        test_v2_list_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppPipelineEvents".to_string(),
        test_v2_search_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.AggregateCIAppTestEvents".to_string(),
        test_v2_aggregate_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppTestEvents".to_string(),
        test_v2_list_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppTestEvents".to_string(),
        test_v2_search_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.ListContainerImages".to_string(),
        test_v2_list_container_images,
    );
    world
        .function_mappings
        .insert("v2.ListContainers".to_string(), test_v2_list_containers);
    world.function_mappings.insert(
        "v2.ListCostAWSCURConfigs".to_string(),
        test_v2_list_cost_awscur_configs,
    );
    world.function_mappings.insert(
        "v2.CreateCostAWSCURConfig".to_string(),
        test_v2_create_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.DeleteCostAWSCURConfig".to_string(),
        test_v2_delete_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.UpdateCostAWSCURConfig".to_string(),
        test_v2_update_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.ListAWSRelatedAccounts".to_string(),
        test_v2_list_aws_related_accounts,
    );
    world.function_mappings.insert(
        "v2.ListCostAzureUCConfigs".to_string(),
        test_v2_list_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.CreateCostAzureUCConfigs".to_string(),
        test_v2_create_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.DeleteCostAzureUCConfig".to_string(),
        test_v2_delete_cost_azure_uc_config,
    );
    world.function_mappings.insert(
        "v2.UpdateCostAzureUCConfigs".to_string(),
        test_v2_update_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.GetCloudCostActivity".to_string(),
        test_v2_get_cloud_cost_activity,
    );
    world.function_mappings.insert(
        "v2.GetActiveBillingDimensions".to_string(),
        test_v2_get_active_billing_dimensions,
    );
    world.function_mappings.insert(
        "v2.GetMonthlyCostAttribution".to_string(),
        test_v2_get_monthly_cost_attribution,
    );
    world.function_mappings.insert(
        "v2.GetUsageApplicationSecurityMonitoring".to_string(),
        test_v2_get_usage_application_security_monitoring,
    );
    world
        .function_mappings
        .insert("v2.GetCostByOrg".to_string(), test_v2_get_cost_by_org);
    world.function_mappings.insert(
        "v2.GetEstimatedCostByOrg".to_string(),
        test_v2_get_estimated_cost_by_org,
    );
    world.function_mappings.insert(
        "v2.GetHistoricalCostByOrg".to_string(),
        test_v2_get_historical_cost_by_org,
    );
    world
        .function_mappings
        .insert("v2.GetHourlyUsage".to_string(), test_v2_get_hourly_usage);
    world.function_mappings.insert(
        "v2.GetUsageLambdaTracedInvocations".to_string(),
        test_v2_get_usage_lambda_traced_invocations,
    );
    world.function_mappings.insert(
        "v2.GetUsageObservabilityPipelines".to_string(),
        test_v2_get_usage_observability_pipelines,
    );
    world.function_mappings.insert(
        "v2.GetProjectedCost".to_string(),
        test_v2_get_projected_cost,
    );
    world.function_mappings.insert(
        "v2.DeleteDashboardListItems".to_string(),
        test_v2_delete_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.GetDashboardListItems".to_string(),
        test_v2_get_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.CreateDashboardListItems".to_string(),
        test_v2_create_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.UpdateDashboardListItems".to_string(),
        test_v2_update_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.CreateDORADeployment".to_string(),
        test_v2_create_dora_deployment,
    );
    world.function_mappings.insert(
        "v2.CreateDORAIncident".to_string(),
        test_v2_create_dora_incident,
    );
    world
        .function_mappings
        .insert("v2.ListDowntimes".to_string(), test_v2_list_downtimes);
    world
        .function_mappings
        .insert("v2.CreateDowntime".to_string(), test_v2_create_downtime);
    world
        .function_mappings
        .insert("v2.CancelDowntime".to_string(), test_v2_cancel_downtime);
    world
        .function_mappings
        .insert("v2.GetDowntime".to_string(), test_v2_get_downtime);
    world
        .function_mappings
        .insert("v2.UpdateDowntime".to_string(), test_v2_update_downtime);
    world.function_mappings.insert(
        "v2.ListMonitorDowntimes".to_string(),
        test_v2_list_monitor_downtimes,
    );
    world
        .function_mappings
        .insert("v2.ListEvents".to_string(), test_v2_list_events);
    world
        .function_mappings
        .insert("v2.SearchEvents".to_string(), test_v2_search_events);
    world
        .function_mappings
        .insert("v2.ListIncidents".to_string(), test_v2_list_incidents);
    world
        .function_mappings
        .insert("v2.CreateIncident".to_string(), test_v2_create_incident);
    world
        .function_mappings
        .insert("v2.SearchIncidents".to_string(), test_v2_search_incidents);
    world
        .function_mappings
        .insert("v2.DeleteIncident".to_string(), test_v2_delete_incident);
    world
        .function_mappings
        .insert("v2.GetIncident".to_string(), test_v2_get_incident);
    world
        .function_mappings
        .insert("v2.UpdateIncident".to_string(), test_v2_update_incident);
    world.function_mappings.insert(
        "v2.ListIncidentAttachments".to_string(),
        test_v2_list_incident_attachments,
    );
    world.function_mappings.insert(
        "v2.UpdateIncidentAttachments".to_string(),
        test_v2_update_incident_attachments,
    );
    world.function_mappings.insert(
        "v2.ListIncidentIntegrations".to_string(),
        test_v2_list_incident_integrations,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentIntegration".to_string(),
        test_v2_create_incident_integration,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentIntegration".to_string(),
        test_v2_delete_incident_integration,
    );
    world.function_mappings.insert(
        "v2.GetIncidentIntegration".to_string(),
        test_v2_get_incident_integration,
    );
    world.function_mappings.insert(
        "v2.UpdateIncidentIntegration".to_string(),
        test_v2_update_incident_integration,
    );
    world.function_mappings.insert(
        "v2.ListIncidentTodos".to_string(),
        test_v2_list_incident_todos,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentTodo".to_string(),
        test_v2_create_incident_todo,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentTodo".to_string(),
        test_v2_delete_incident_todo,
    );
    world
        .function_mappings
        .insert("v2.GetIncidentTodo".to_string(), test_v2_get_incident_todo);
    world.function_mappings.insert(
        "v2.UpdateIncidentTodo".to_string(),
        test_v2_update_incident_todo,
    );
    world.function_mappings.insert(
        "v2.ListGCPSTSAccounts".to_string(),
        test_v2_list_gcpsts_accounts,
    );
    world.function_mappings.insert(
        "v2.CreateGCPSTSAccount".to_string(),
        test_v2_create_gcpsts_account,
    );
    world.function_mappings.insert(
        "v2.DeleteGCPSTSAccount".to_string(),
        test_v2_delete_gcpsts_account,
    );
    world.function_mappings.insert(
        "v2.UpdateGCPSTSAccount".to_string(),
        test_v2_update_gcpsts_account,
    );
    world.function_mappings.insert(
        "v2.GetGCPSTSDelegate".to_string(),
        test_v2_get_gcpsts_delegate,
    );
    world.function_mappings.insert(
        "v2.MakeGCPSTSDelegate".to_string(),
        test_v2_make_gcpsts_delegate,
    );
    world.function_mappings.insert(
        "v2.ListOpsgenieServices".to_string(),
        test_v2_list_opsgenie_services,
    );
    world.function_mappings.insert(
        "v2.CreateOpsgenieService".to_string(),
        test_v2_create_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.DeleteOpsgenieService".to_string(),
        test_v2_delete_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.GetOpsgenieService".to_string(),
        test_v2_get_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.UpdateOpsgenieService".to_string(),
        test_v2_update_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.ListCloudflareAccounts".to_string(),
        test_v2_list_cloudflare_accounts,
    );
    world.function_mappings.insert(
        "v2.CreateCloudflareAccount".to_string(),
        test_v2_create_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.DeleteCloudflareAccount".to_string(),
        test_v2_delete_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.GetCloudflareAccount".to_string(),
        test_v2_get_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.UpdateCloudflareAccount".to_string(),
        test_v2_update_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.ListConfluentAccount".to_string(),
        test_v2_list_confluent_account,
    );
    world.function_mappings.insert(
        "v2.CreateConfluentAccount".to_string(),
        test_v2_create_confluent_account,
    );
    world.function_mappings.insert(
        "v2.DeleteConfluentAccount".to_string(),
        test_v2_delete_confluent_account,
    );
    world.function_mappings.insert(
        "v2.GetConfluentAccount".to_string(),
        test_v2_get_confluent_account,
    );
    world.function_mappings.insert(
        "v2.UpdateConfluentAccount".to_string(),
        test_v2_update_confluent_account,
    );
    world.function_mappings.insert(
        "v2.ListConfluentResource".to_string(),
        test_v2_list_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.CreateConfluentResource".to_string(),
        test_v2_create_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.DeleteConfluentResource".to_string(),
        test_v2_delete_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.GetConfluentResource".to_string(),
        test_v2_get_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.UpdateConfluentResource".to_string(),
        test_v2_update_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.ListFastlyAccounts".to_string(),
        test_v2_list_fastly_accounts,
    );
    world.function_mappings.insert(
        "v2.CreateFastlyAccount".to_string(),
        test_v2_create_fastly_account,
    );
    world.function_mappings.insert(
        "v2.DeleteFastlyAccount".to_string(),
        test_v2_delete_fastly_account,
    );
    world.function_mappings.insert(
        "v2.GetFastlyAccount".to_string(),
        test_v2_get_fastly_account,
    );
    world.function_mappings.insert(
        "v2.UpdateFastlyAccount".to_string(),
        test_v2_update_fastly_account,
    );
    world.function_mappings.insert(
        "v2.ListFastlyServices".to_string(),
        test_v2_list_fastly_services,
    );
    world.function_mappings.insert(
        "v2.CreateFastlyService".to_string(),
        test_v2_create_fastly_service,
    );
    world.function_mappings.insert(
        "v2.DeleteFastlyService".to_string(),
        test_v2_delete_fastly_service,
    );
    world.function_mappings.insert(
        "v2.GetFastlyService".to_string(),
        test_v2_get_fastly_service,
    );
    world.function_mappings.insert(
        "v2.UpdateFastlyService".to_string(),
        test_v2_update_fastly_service,
    );
    world.function_mappings.insert(
        "v2.ListOktaAccounts".to_string(),
        test_v2_list_okta_accounts,
    );
    world.function_mappings.insert(
        "v2.CreateOktaAccount".to_string(),
        test_v2_create_okta_account,
    );
    world.function_mappings.insert(
        "v2.DeleteOktaAccount".to_string(),
        test_v2_delete_okta_account,
    );
    world
        .function_mappings
        .insert("v2.GetOktaAccount".to_string(), test_v2_get_okta_account);
    world.function_mappings.insert(
        "v2.UpdateOktaAccount".to_string(),
        test_v2_update_okta_account,
    );
    world
        .function_mappings
        .insert("v2.GetIPAllowlist".to_string(), test_v2_get_ip_allowlist);
    world.function_mappings.insert(
        "v2.UpdateIPAllowlist".to_string(),
        test_v2_update_ip_allowlist,
    );
    world
        .function_mappings
        .insert("v2.SubmitLog".to_string(), test_v2_submit_log);
    world
        .function_mappings
        .insert("v2.AggregateLogs".to_string(), test_v2_aggregate_logs);
    world
        .function_mappings
        .insert("v2.ListLogsGet".to_string(), test_v2_list_logs_get);
    world
        .function_mappings
        .insert("v2.ListLogs".to_string(), test_v2_list_logs);
    world.function_mappings.insert(
        "v2.GetLogsArchiveOrder".to_string(),
        test_v2_get_logs_archive_order,
    );
    world.function_mappings.insert(
        "v2.UpdateLogsArchiveOrder".to_string(),
        test_v2_update_logs_archive_order,
    );
    world.function_mappings.insert(
        "v2.ListLogsArchives".to_string(),
        test_v2_list_logs_archives,
    );
    world.function_mappings.insert(
        "v2.CreateLogsArchive".to_string(),
        test_v2_create_logs_archive,
    );
    world.function_mappings.insert(
        "v2.DeleteLogsArchive".to_string(),
        test_v2_delete_logs_archive,
    );
    world
        .function_mappings
        .insert("v2.GetLogsArchive".to_string(), test_v2_get_logs_archive);
    world.function_mappings.insert(
        "v2.UpdateLogsArchive".to_string(),
        test_v2_update_logs_archive,
    );
    world.function_mappings.insert(
        "v2.RemoveRoleFromArchive".to_string(),
        test_v2_remove_role_from_archive,
    );
    world.function_mappings.insert(
        "v2.ListArchiveReadRoles".to_string(),
        test_v2_list_archive_read_roles,
    );
    world.function_mappings.insert(
        "v2.AddReadRoleToArchive".to_string(),
        test_v2_add_read_role_to_archive,
    );
    world
        .function_mappings
        .insert("v2.ListLogsMetrics".to_string(), test_v2_list_logs_metrics);
    world.function_mappings.insert(
        "v2.CreateLogsMetric".to_string(),
        test_v2_create_logs_metric,
    );
    world.function_mappings.insert(
        "v2.DeleteLogsMetric".to_string(),
        test_v2_delete_logs_metric,
    );
    world
        .function_mappings
        .insert("v2.GetLogsMetric".to_string(), test_v2_get_logs_metric);
    world.function_mappings.insert(
        "v2.UpdateLogsMetric".to_string(),
        test_v2_update_logs_metric,
    );
    world.function_mappings.insert(
        "v2.ListTagConfigurations".to_string(),
        test_v2_list_tag_configurations,
    );
    world.function_mappings.insert(
        "v2.DeleteBulkTagsMetricsConfiguration".to_string(),
        test_v2_delete_bulk_tags_metrics_configuration,
    );
    world.function_mappings.insert(
        "v2.CreateBulkTagsMetricsConfiguration".to_string(),
        test_v2_create_bulk_tags_metrics_configuration,
    );
    world.function_mappings.insert(
        "v2.ListActiveMetricConfigurations".to_string(),
        test_v2_list_active_metric_configurations,
    );
    world.function_mappings.insert(
        "v2.ListTagsByMetricName".to_string(),
        test_v2_list_tags_by_metric_name,
    );
    world.function_mappings.insert(
        "v2.EstimateMetricsOutputSeries".to_string(),
        test_v2_estimate_metrics_output_series,
    );
    world.function_mappings.insert(
        "v2.DeleteTagConfiguration".to_string(),
        test_v2_delete_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.ListTagConfigurationByName".to_string(),
        test_v2_list_tag_configuration_by_name,
    );
    world.function_mappings.insert(
        "v2.UpdateTagConfiguration".to_string(),
        test_v2_update_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.CreateTagConfiguration".to_string(),
        test_v2_create_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.ListVolumesByMetricName".to_string(),
        test_v2_list_volumes_by_metric_name,
    );
    world
        .function_mappings
        .insert("v2.QueryScalarData".to_string(), test_v2_query_scalar_data);
    world.function_mappings.insert(
        "v2.QueryTimeseriesData".to_string(),
        test_v2_query_timeseries_data,
    );
    world
        .function_mappings
        .insert("v2.SubmitMetrics".to_string(), test_v2_submit_metrics);
    world.function_mappings.insert(
        "v2.ListMonitorConfigPolicies".to_string(),
        test_v2_list_monitor_config_policies,
    );
    world.function_mappings.insert(
        "v2.CreateMonitorConfigPolicy".to_string(),
        test_v2_create_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.DeleteMonitorConfigPolicy".to_string(),
        test_v2_delete_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.GetMonitorConfigPolicy".to_string(),
        test_v2_get_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.UpdateMonitorConfigPolicy".to_string(),
        test_v2_update_monitor_config_policy,
    );
    world
        .function_mappings
        .insert("v2.ListPermissions".to_string(), test_v2_list_permissions);
    world
        .function_mappings
        .insert("v2.ListRoles".to_string(), test_v2_list_roles);
    world
        .function_mappings
        .insert("v2.CreateRole".to_string(), test_v2_create_role);
    world
        .function_mappings
        .insert("v2.DeleteRole".to_string(), test_v2_delete_role);
    world
        .function_mappings
        .insert("v2.GetRole".to_string(), test_v2_get_role);
    world
        .function_mappings
        .insert("v2.UpdateRole".to_string(), test_v2_update_role);
    world
        .function_mappings
        .insert("v2.CloneRole".to_string(), test_v2_clone_role);
    world.function_mappings.insert(
        "v2.RemovePermissionFromRole".to_string(),
        test_v2_remove_permission_from_role,
    );
    world.function_mappings.insert(
        "v2.ListRolePermissions".to_string(),
        test_v2_list_role_permissions,
    );
    world.function_mappings.insert(
        "v2.AddPermissionToRole".to_string(),
        test_v2_add_permission_to_role,
    );
    world.function_mappings.insert(
        "v2.RemoveUserFromRole".to_string(),
        test_v2_remove_user_from_role,
    );
    world
        .function_mappings
        .insert("v2.ListRoleUsers".to_string(), test_v2_list_role_users);
    world
        .function_mappings
        .insert("v2.AddUserToRole".to_string(), test_v2_add_user_to_role);
    world
        .function_mappings
        .insert("v2.ListFindings".to_string(), test_v2_list_findings);
    world
        .function_mappings
        .insert("v2.MuteFindings".to_string(), test_v2_mute_findings);
    world
        .function_mappings
        .insert("v2.GetFinding".to_string(), test_v2_get_finding);
    world.function_mappings.insert(
        "v2.ListSecurityFilters".to_string(),
        test_v2_list_security_filters,
    );
    world.function_mappings.insert(
        "v2.CreateSecurityFilter".to_string(),
        test_v2_create_security_filter,
    );
    world.function_mappings.insert(
        "v2.DeleteSecurityFilter".to_string(),
        test_v2_delete_security_filter,
    );
    world.function_mappings.insert(
        "v2.GetSecurityFilter".to_string(),
        test_v2_get_security_filter,
    );
    world.function_mappings.insert(
        "v2.UpdateSecurityFilter".to_string(),
        test_v2_update_security_filter,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringRules".to_string(),
        test_v2_list_security_monitoring_rules,
    );
    world.function_mappings.insert(
        "v2.CreateSecurityMonitoringRule".to_string(),
        test_v2_create_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteSecurityMonitoringRule".to_string(),
        test_v2_delete_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.GetSecurityMonitoringRule".to_string(),
        test_v2_get_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateSecurityMonitoringRule".to_string(),
        test_v2_update_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringSignals".to_string(),
        test_v2_list_security_monitoring_signals,
    );
    world.function_mappings.insert(
        "v2.SearchSecurityMonitoringSignals".to_string(),
        test_v2_search_security_monitoring_signals,
    );
    world.function_mappings.insert(
        "v2.GetSecurityMonitoringSignal".to_string(),
        test_v2_get_security_monitoring_signal,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalAssignee".to_string(),
        test_v2_edit_security_monitoring_signal_assignee,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalIncidents".to_string(),
        test_v2_edit_security_monitoring_signal_incidents,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalState".to_string(),
        test_v2_edit_security_monitoring_signal_state,
    );
    world
        .function_mappings
        .insert("v2.ListPowerpacks".to_string(), test_v2_list_powerpacks);
    world
        .function_mappings
        .insert("v2.CreatePowerpack".to_string(), test_v2_create_powerpack);
    world
        .function_mappings
        .insert("v2.DeletePowerpack".to_string(), test_v2_delete_powerpack);
    world
        .function_mappings
        .insert("v2.GetPowerpack".to_string(), test_v2_get_powerpack);
    world
        .function_mappings
        .insert("v2.UpdatePowerpack".to_string(), test_v2_update_powerpack);
    world
        .function_mappings
        .insert("v2.ListProcesses".to_string(), test_v2_list_processes);
    world.function_mappings.insert(
        "v2.DeleteRestrictionPolicy".to_string(),
        test_v2_delete_restriction_policy,
    );
    world.function_mappings.insert(
        "v2.GetRestrictionPolicy".to_string(),
        test_v2_get_restriction_policy,
    );
    world.function_mappings.insert(
        "v2.UpdateRestrictionPolicy".to_string(),
        test_v2_update_restriction_policy,
    );
    world.function_mappings.insert(
        "v2.AggregateRUMEvents".to_string(),
        test_v2_aggregate_rum_events,
    );
    world.function_mappings.insert(
        "v2.GetRUMApplications".to_string(),
        test_v2_get_rum_applications,
    );
    world.function_mappings.insert(
        "v2.CreateRUMApplication".to_string(),
        test_v2_create_rum_application,
    );
    world.function_mappings.insert(
        "v2.DeleteRUMApplication".to_string(),
        test_v2_delete_rum_application,
    );
    world.function_mappings.insert(
        "v2.GetRUMApplication".to_string(),
        test_v2_get_rum_application,
    );
    world.function_mappings.insert(
        "v2.UpdateRUMApplication".to_string(),
        test_v2_update_rum_application,
    );
    world
        .function_mappings
        .insert("v2.ListRUMEvents".to_string(), test_v2_list_rum_events);
    world
        .function_mappings
        .insert("v2.SearchRUMEvents".to_string(), test_v2_search_rum_events);
    world.function_mappings.insert(
        "v2.UploadIdPMetadata".to_string(),
        test_v2_upload_id_p_metadata,
    );
    world.function_mappings.insert(
        "v2.ListScorecardOutcomes".to_string(),
        test_v2_list_scorecard_outcomes,
    );
    world.function_mappings.insert(
        "v2.CreateScorecardOutcomesBatch".to_string(),
        test_v2_create_scorecard_outcomes_batch,
    );
    world.function_mappings.insert(
        "v2.ListScorecardRules".to_string(),
        test_v2_list_scorecard_rules,
    );
    world.function_mappings.insert(
        "v2.CreateScorecardRule".to_string(),
        test_v2_create_scorecard_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteScorecardRule".to_string(),
        test_v2_delete_scorecard_rule,
    );
    world.function_mappings.insert(
        "v2.DownloadCloudWorkloadPolicyFile".to_string(),
        test_v2_download_cloud_workload_policy_file,
    );
    world.function_mappings.insert(
        "v2.ListCloudWorkloadSecurityAgentRules".to_string(),
        test_v2_list_cloud_workload_security_agent_rules,
    );
    world.function_mappings.insert(
        "v2.CreateCloudWorkloadSecurityAgentRule".to_string(),
        test_v2_create_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteCloudWorkloadSecurityAgentRule".to_string(),
        test_v2_delete_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.GetCloudWorkloadSecurityAgentRule".to_string(),
        test_v2_get_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateCloudWorkloadSecurityAgentRule".to_string(),
        test_v2_update_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.ListScanningGroups".to_string(),
        test_v2_list_scanning_groups,
    );
    world.function_mappings.insert(
        "v2.ReorderScanningGroups".to_string(),
        test_v2_reorder_scanning_groups,
    );
    world.function_mappings.insert(
        "v2.CreateScanningGroup".to_string(),
        test_v2_create_scanning_group,
    );
    world.function_mappings.insert(
        "v2.DeleteScanningGroup".to_string(),
        test_v2_delete_scanning_group,
    );
    world.function_mappings.insert(
        "v2.UpdateScanningGroup".to_string(),
        test_v2_update_scanning_group,
    );
    world.function_mappings.insert(
        "v2.CreateScanningRule".to_string(),
        test_v2_create_scanning_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteScanningRule".to_string(),
        test_v2_delete_scanning_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateScanningRule".to_string(),
        test_v2_update_scanning_rule,
    );
    world.function_mappings.insert(
        "v2.ListStandardPatterns".to_string(),
        test_v2_list_standard_patterns,
    );
    world.function_mappings.insert(
        "v2.CreateServiceAccount".to_string(),
        test_v2_create_service_account,
    );
    world.function_mappings.insert(
        "v2.ListServiceAccountApplicationKeys".to_string(),
        test_v2_list_service_account_application_keys,
    );
    world.function_mappings.insert(
        "v2.CreateServiceAccountApplicationKey".to_string(),
        test_v2_create_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.DeleteServiceAccountApplicationKey".to_string(),
        test_v2_delete_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.GetServiceAccountApplicationKey".to_string(),
        test_v2_get_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.UpdateServiceAccountApplicationKey".to_string(),
        test_v2_update_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.ListIncidentServices".to_string(),
        test_v2_list_incident_services,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentService".to_string(),
        test_v2_create_incident_service,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentService".to_string(),
        test_v2_delete_incident_service,
    );
    world.function_mappings.insert(
        "v2.GetIncidentService".to_string(),
        test_v2_get_incident_service,
    );
    world.function_mappings.insert(
        "v2.UpdateIncidentService".to_string(),
        test_v2_update_incident_service,
    );
    world.function_mappings.insert(
        "v2.ListServiceDefinitions".to_string(),
        test_v2_list_service_definitions,
    );
    world.function_mappings.insert(
        "v2.CreateOrUpdateServiceDefinitions".to_string(),
        test_v2_create_or_update_service_definitions,
    );
    world.function_mappings.insert(
        "v2.DeleteServiceDefinition".to_string(),
        test_v2_delete_service_definition,
    );
    world.function_mappings.insert(
        "v2.GetServiceDefinition".to_string(),
        test_v2_get_service_definition,
    );
    world
        .function_mappings
        .insert("v2.AggregateSpans".to_string(), test_v2_aggregate_spans);
    world
        .function_mappings
        .insert("v2.ListSpansGet".to_string(), test_v2_list_spans_get);
    world
        .function_mappings
        .insert("v2.ListSpans".to_string(), test_v2_list_spans);
    world.function_mappings.insert(
        "v2.GetOnDemandConcurrencyCap".to_string(),
        test_v2_get_on_demand_concurrency_cap,
    );
    world.function_mappings.insert(
        "v2.SetOnDemandConcurrencyCap".to_string(),
        test_v2_set_on_demand_concurrency_cap,
    );
    world
        .function_mappings
        .insert("v2.ListTeams".to_string(), test_v2_list_teams);
    world
        .function_mappings
        .insert("v2.CreateTeam".to_string(), test_v2_create_team);
    world
        .function_mappings
        .insert("v2.DeleteTeam".to_string(), test_v2_delete_team);
    world
        .function_mappings
        .insert("v2.GetTeam".to_string(), test_v2_get_team);
    world
        .function_mappings
        .insert("v2.UpdateTeam".to_string(), test_v2_update_team);
    world
        .function_mappings
        .insert("v2.GetTeamLinks".to_string(), test_v2_get_team_links);
    world
        .function_mappings
        .insert("v2.CreateTeamLink".to_string(), test_v2_create_team_link);
    world
        .function_mappings
        .insert("v2.DeleteTeamLink".to_string(), test_v2_delete_team_link);
    world
        .function_mappings
        .insert("v2.GetTeamLink".to_string(), test_v2_get_team_link);
    world
        .function_mappings
        .insert("v2.UpdateTeamLink".to_string(), test_v2_update_team_link);
    world.function_mappings.insert(
        "v2.GetTeamMemberships".to_string(),
        test_v2_get_team_memberships,
    );
    world.function_mappings.insert(
        "v2.CreateTeamMembership".to_string(),
        test_v2_create_team_membership,
    );
    world.function_mappings.insert(
        "v2.DeleteTeamMembership".to_string(),
        test_v2_delete_team_membership,
    );
    world.function_mappings.insert(
        "v2.UpdateTeamMembership".to_string(),
        test_v2_update_team_membership,
    );
    world.function_mappings.insert(
        "v2.GetTeamPermissionSettings".to_string(),
        test_v2_get_team_permission_settings,
    );
    world.function_mappings.insert(
        "v2.UpdateTeamPermissionSetting".to_string(),
        test_v2_update_team_permission_setting,
    );
    world.function_mappings.insert(
        "v2.GetUserMemberships".to_string(),
        test_v2_get_user_memberships,
    );
    world.function_mappings.insert(
        "v2.ListIncidentTeams".to_string(),
        test_v2_list_incident_teams,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentTeam".to_string(),
        test_v2_create_incident_team,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentTeam".to_string(),
        test_v2_delete_incident_team,
    );
    world
        .function_mappings
        .insert("v2.GetIncidentTeam".to_string(), test_v2_get_incident_team);
    world.function_mappings.insert(
        "v2.UpdateIncidentTeam".to_string(),
        test_v2_update_incident_team,
    );
    world
        .function_mappings
        .insert("v2.SendInvitations".to_string(), test_v2_send_invitations);
    world
        .function_mappings
        .insert("v2.GetInvitation".to_string(), test_v2_get_invitation);
    world
        .function_mappings
        .insert("v2.ListUsers".to_string(), test_v2_list_users);
    world
        .function_mappings
        .insert("v2.CreateUser".to_string(), test_v2_create_user);
    world
        .function_mappings
        .insert("v2.DisableUser".to_string(), test_v2_disable_user);
    world
        .function_mappings
        .insert("v2.GetUser".to_string(), test_v2_get_user);
    world
        .function_mappings
        .insert("v2.UpdateUser".to_string(), test_v2_update_user);
    world.function_mappings.insert(
        "v2.ListUserOrganizations".to_string(),
        test_v2_list_user_organizations,
    );
    world.function_mappings.insert(
        "v2.ListUserPermissions".to_string(),
        test_v2_list_user_permissions,
    );
}

fn test_v1_get_ip_ranges(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_ip_ranges
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_ip_ranges_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_api_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_api_keys_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_api_key_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_api_key_with_http_info(key)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let response = match block_on(api.get_api_key_with_http_info(key)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_api_key_with_http_info(key, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_application_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_application_keys_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_application_key_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_application_key_with_http_info(key)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let response = match block_on(api.get_application_key_with_http_info(key)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_key_management
        .as_ref()
        .expect("api instance not found");
    let key = serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_application_key_with_http_info(key, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_submit_service_check(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_checks
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.submit_service_check_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_daily_custom_reports(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_dir = if let Some(param) = _parameters.get("sort_dir") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetDailyCustomReportsOptionalParams {
        page_size,
        page_number,
        sort_dir,
        sort,
    };
    let response = match block_on(api.get_daily_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_specified_daily_custom_reports(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let report_id = serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_specified_daily_custom_reports_with_http_info(report_id))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_monthly_custom_reports(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_dir = if let Some(param) = _parameters.get("sort_dir") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetMonthlyCustomReportsOptionalParams {
        page_size,
        page_number,
        sort_dir,
        sort,
    };
    let response = match block_on(api.get_monthly_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_specified_monthly_custom_reports(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let report_id = serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_specified_monthly_custom_reports_with_http_info(report_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_analyzed_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageAnalyzedLogsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_analyzed_logs_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_attribution(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let fields = serde_json::from_value(_parameters.get("fields").unwrap().clone()).unwrap();
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_direction = if let Some(param) = _parameters.get("sort_direction") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_name = if let Some(param) = _parameters.get("sort_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_descendants = if let Some(param) = _parameters.get("include_descendants") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let offset = if let Some(param) = _parameters.get("offset") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let limit = if let Some(param) = _parameters.get("limit") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageAttributionOptionalParams {
        end_month,
        sort_direction,
        sort_name,
        include_descendants,
        offset,
        limit,
    };
    let response =
        match block_on(api.get_usage_attribution_with_http_info(start_month, fields, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageAuditLogsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_audit_logs_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_lambda(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageLambdaOptionalParams { end_hr };
    let response = match block_on(api.get_usage_lambda_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_billable_summary(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let month = if let Some(param) = _parameters.get("month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageBillableSummaryOptionalParams { month };
    let response = match block_on(api.get_usage_billable_summary_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_ci_app(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageCIAppOptionalParams { end_hr };
    let response = match block_on(api.get_usage_ci_app_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_cloud_security_posture_management(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageCloudSecurityPostureManagementOptionalParams {
            end_hr,
        };
    let response = match block_on(
        api.get_usage_cloud_security_posture_management_with_http_info(start_hr, params),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_cws(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageCWSOptionalParams { end_hr };
    let response = match block_on(api.get_usage_cws_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_dbm(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageDBMOptionalParams { end_hr };
    let response = match block_on(api.get_usage_dbm_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_fargate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageFargateOptionalParams { end_hr };
    let response = match block_on(api.get_usage_fargate_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageHostsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_hosts_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_hourly_usage_attribution(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let usage_type =
        serde_json::from_value(_parameters.get("usage_type").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let next_record_id = if let Some(param) = _parameters.get("next_record_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tag_breakdown_keys = if let Some(param) = _parameters.get("tag_breakdown_keys") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_descendants = if let Some(param) = _parameters.get("include_descendants") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetHourlyUsageAttributionOptionalParams {
        end_hr,
        next_record_id,
        tag_breakdown_keys,
        include_descendants,
    };
    let response = match block_on(
        api.get_hourly_usage_attribution_with_http_info(start_hr, usage_type, params),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_incident_management(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetIncidentManagementOptionalParams { end_hr };
    let response = match block_on(api.get_incident_management_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_indexed_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageIndexedSpansOptionalParams { end_hr };
    let response = match block_on(api.get_usage_indexed_spans_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_ingested_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetIngestedSpansOptionalParams { end_hr };
    let response = match block_on(api.get_ingested_spans_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_internet_of_things(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageInternetOfThingsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_internet_of_things_with_http_info(start_hr, params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageLogsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_logs_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_logs_by_retention(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageLogsByRetentionOptionalParams { end_hr };
    let response = match block_on(api.get_usage_logs_by_retention_with_http_info(start_hr, params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_logs_by_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let index_name = if let Some(param) = _parameters.get("index_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageLogsByIndexOptionalParams {
        end_hr,
        index_name,
    };
    let response = match block_on(api.get_usage_logs_by_index_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_monthly_usage_attribution(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let fields = serde_json::from_value(_parameters.get("fields").unwrap().clone()).unwrap();
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_direction = if let Some(param) = _parameters.get("sort_direction") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_name = if let Some(param) = _parameters.get("sort_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tag_breakdown_keys = if let Some(param) = _parameters.get("tag_breakdown_keys") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let next_record_id = if let Some(param) = _parameters.get("next_record_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_descendants = if let Some(param) = _parameters.get("include_descendants") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetMonthlyUsageAttributionOptionalParams {
        end_month,
        sort_direction,
        sort_name,
        tag_breakdown_keys,
        next_record_id,
        include_descendants,
    };
    let response = match block_on(api.get_monthly_usage_attribution_with_http_info(
        start_month,
        fields,
        params,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_network_flows(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageNetworkFlowsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_network_flows_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_network_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageNetworkHostsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_network_hosts_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_online_archive(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageOnlineArchiveOptionalParams { end_hr };
    let response = match block_on(api.get_usage_online_archive_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_profiling(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageProfilingOptionalParams { end_hr };
    let response = match block_on(api.get_usage_profiling_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_rum_units(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageRumUnitsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_rum_units_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_rum_sessions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let type_ = if let Some(param) = _parameters.get("type") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageRumSessionsOptionalParams { end_hr, type_ };
    let response = match block_on(api.get_usage_rum_sessions_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_sds(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageSDSOptionalParams { end_hr };
    let response = match block_on(api.get_usage_sds_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_snmp(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageSNMPOptionalParams { end_hr };
    let response = match block_on(api.get_usage_snmp_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_summary(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_org_details = if let Some(param) = _parameters.get("include_org_details") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageSummaryOptionalParams {
        end_month,
        include_org_details,
    };
    let response = match block_on(api.get_usage_summary_with_http_info(start_month, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_synthetics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageSyntheticsOptionalParams { end_hr };
    let response = match block_on(api.get_usage_synthetics_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_synthetics_api(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageSyntheticsAPIOptionalParams { end_hr };
    let response = match block_on(api.get_usage_synthetics_api_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_synthetics_browser(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_usage_metering::GetUsageSyntheticsBrowserOptionalParams { end_hr };
    let response = match block_on(api.get_usage_synthetics_browser_with_http_info(start_hr, params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_timeseries(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageTimeseriesOptionalParams { end_hr };
    let response = match block_on(api.get_usage_timeseries_with_http_info(start_hr, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_usage_top_avg_metrics(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let month = if let Some(param) = _parameters.get("month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let day = if let Some(param) = _parameters.get("day") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let names = if let Some(param) = _parameters.get("names") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let limit = if let Some(param) = _parameters.get("limit") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let next_record_id = if let Some(param) = _parameters.get("next_record_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_usage_metering::GetUsageTopAvgMetricsOptionalParams {
        month,
        day,
        names,
        limit,
        next_record_id,
    };
    let response = match block_on(api.get_usage_top_avg_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_dashboards(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_dashboards_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_dashboards(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let filter_shared = if let Some(param) = _parameters.get("filter[shared]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_deleted = if let Some(param) = _parameters.get("filter[deleted]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let count = if let Some(param) = _parameters.get("count") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let start = if let Some(param) = _parameters.get("start") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_dashboards::ListDashboardsOptionalParams {
        filter_shared,
        filter_deleted,
        count,
        start,
    };
    let response = match block_on(api.list_dashboards_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_restore_dashboards(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.restore_dashboards_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_dashboard_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_public_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_public_dashboard_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_public_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_public_dashboard_with_http_info(token)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_public_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let response = match block_on(api.get_public_dashboard_with_http_info(token)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_public_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_public_dashboard_with_http_info(token, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_public_dashboard_invitation(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_public_dashboard_invitation_with_http_info(token, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_public_dashboard_invitations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let page_size = if let Some(param) = _parameters.get("page_size") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page_number") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_dashboards::GetPublicDashboardInvitationsOptionalParams {
        page_size,
        page_number,
    };
    let response =
        match block_on(api.get_public_dashboard_invitations_with_http_info(token, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_send_public_dashboard_invitation(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let token = serde_json::from_value(_parameters.get("token").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.send_public_dashboard_invitation_with_http_info(token, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let dashboard_id =
        serde_json::from_value(_parameters.get("dashboard_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_dashboard_with_http_info(dashboard_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let dashboard_id =
        serde_json::from_value(_parameters.get("dashboard_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_dashboard_with_http_info(dashboard_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_dashboard(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let dashboard_id =
        serde_json::from_value(_parameters.get("dashboard_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_dashboard_with_http_info(dashboard_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_dashboard_lists(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_dashboard_lists_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_dashboard_list_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let list_id = serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_dashboard_list_with_http_info(list_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let list_id = serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_dashboard_list_with_http_info(list_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let list_id = serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_dashboard_list_with_http_info(list_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_submit_distribution_points(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let content_encoding = if let Some(param) = _parameters.get("Content-Encoding") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_metrics::SubmitDistributionPointsOptionalParams { content_encoding };
    let response = match block_on(api.submit_distribution_points_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_active_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let from = serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap();
    let host = if let Some(param) = _parameters.get("host") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tag_filter = if let Some(param) = _parameters.get("tag_filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_metrics::ListActiveMetricsOptionalParams { host, tag_filter };
    let response = match block_on(api.list_active_metrics_with_http_info(from, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_metric_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.get_metric_metadata_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_metric_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_metric_metadata_with_http_info(metric_name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_query_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let from = serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap();
    let to = serde_json::from_value(_parameters.get("to").unwrap().clone()).unwrap();
    let query = serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap();
    let response = match block_on(api.query_metrics_with_http_info(from, to, query)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let q = serde_json::from_value(_parameters.get("q").unwrap().clone()).unwrap();
    let response = match block_on(api.list_metrics_with_http_info(q)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_submit_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let content_encoding = if let Some(param) = _parameters.get("Content-Encoding") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_metrics::SubmitMetricsOptionalParams { content_encoding };
    let response = match block_on(api.submit_metrics_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let current_only = if let Some(param) = _parameters.get("current_only") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let with_creator = if let Some(param) = _parameters.get("with_creator") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_downtimes::ListDowntimesOptionalParams {
        current_only,
        with_creator,
    };
    let response = match block_on(api.list_downtimes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_downtime_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_cancel_downtimes_by_scope(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.cancel_downtimes_by_scope_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_cancel_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let response = match block_on(api.cancel_downtime_with_http_info(downtime_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_downtime_with_http_info(downtime_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_downtime_with_http_info(downtime_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_monitor_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_monitor_downtimes_with_http_info(monitor_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_events
        .as_ref()
        .expect("api instance not found");
    let start = serde_json::from_value(_parameters.get("start").unwrap().clone()).unwrap();
    let end = serde_json::from_value(_parameters.get("end").unwrap().clone()).unwrap();
    let priority = if let Some(param) = _parameters.get("priority") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sources = if let Some(param) = _parameters.get("sources") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tags = if let Some(param) = _parameters.get("tags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let unaggregated = if let Some(param) = _parameters.get("unaggregated") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let exclude_aggregate = if let Some(param) = _parameters.get("exclude_aggregate") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page = if let Some(param) = _parameters.get("page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_events::ListEventsOptionalParams {
        priority,
        sources,
        tags,
        unaggregated,
        exclude_aggregate,
        page,
    };
    let response = match block_on(api.list_events_with_http_info(start, end, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_event(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_events
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_event_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_event(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_events
        .as_ref()
        .expect("api instance not found");
    let event_id = serde_json::from_value(_parameters.get("event_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_event_with_http_info(event_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_graph_snapshot(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_snapshots
        .as_ref()
        .expect("api instance not found");
    let start = serde_json::from_value(_parameters.get("start").unwrap().clone()).unwrap();
    let end = serde_json::from_value(_parameters.get("end").unwrap().clone()).unwrap();
    let metric_query = if let Some(param) = _parameters.get("metric_query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let event_query = if let Some(param) = _parameters.get("event_query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let graph_def = if let Some(param) = _parameters.get("graph_def") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let title = if let Some(param) = _parameters.get("title") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let height = if let Some(param) = _parameters.get("height") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let width = if let Some(param) = _parameters.get("width") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_snapshots::GetGraphSnapshotOptionalParams {
        metric_query,
        event_query,
        graph_def,
        title,
        height,
        width,
    };
    let response = match block_on(api.get_graph_snapshot_with_http_info(start, end, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_mute_host(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_hosts
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.mute_host_with_http_info(host_name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_unmute_host(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_hosts
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let response = match block_on(api.unmute_host_with_http_info(host_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_hosts
        .as_ref()
        .expect("api instance not found");
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_field = if let Some(param) = _parameters.get("sort_field") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_dir = if let Some(param) = _parameters.get("sort_dir") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let start = if let Some(param) = _parameters.get("start") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let count = if let Some(param) = _parameters.get("count") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let from = if let Some(param) = _parameters.get("from") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_muted_hosts_data = if let Some(param) = _parameters.get("include_muted_hosts_data")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_hosts_metadata = if let Some(param) = _parameters.get("include_hosts_metadata") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_hosts::ListHostsOptionalParams {
        filter,
        sort_field,
        sort_dir,
        start,
        count,
        from,
        include_muted_hosts_data,
        include_hosts_metadata,
    };
    let response = match block_on(api.list_hosts_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_host_totals(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_hosts
        .as_ref()
        .expect("api instance not found");
    let from = if let Some(param) = _parameters.get("from") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_hosts::GetHostTotalsOptionalParams { from };
    let response = match block_on(api.get_host_totals_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_aws_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_aws_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let account_id = if let Some(param) = _parameters.get("account_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let role_name = if let Some(param) = _parameters.get("role_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let access_key_id = if let Some(param) = _parameters.get("access_key_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_aws_integration::ListAWSAccountsOptionalParams {
        account_id,
        role_name,
        access_key_id,
    };
    let response = match block_on(api.list_aws_accounts_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_aws_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let account_id = if let Some(param) = _parameters.get("account_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let role_name = if let Some(param) = _parameters.get("role_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let access_key_id = if let Some(param) = _parameters.get("access_key_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_aws_integration::UpdateAWSAccountOptionalParams {
        account_id,
        role_name,
        access_key_id,
    };
    let response = match block_on(api.update_aws_account_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_available_aws_namespaces(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_available_aws_namespaces_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_aws_event_bridge_source(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_aws_event_bridge_source_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_aws_event_bridge_sources(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_aws_event_bridge_sources_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_aws_event_bridge_source(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_aws_event_bridge_source_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_aws_tag_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_aws_tag_filter_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_aws_tag_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_aws_tag_filters_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_aws_tag_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_aws_tag_filter_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_new_aws_external_id(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_new_aws_external_id_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_aws_lambda_arn(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_aws_lambda_arn_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_aws_logs_integrations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_aws_logs_integrations_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_aws_lambda_arn(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_aws_lambda_arn_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_check_aws_logs_lambda_async(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.check_aws_logs_lambda_async_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_aws_logs_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_aws_logs_services_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_enable_aws_log_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.enable_aws_log_services_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_check_aws_logs_services_async(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.check_aws_logs_services_async_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_azure_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_azure_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_azure_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_azure_integration_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_azure_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_azure_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_azure_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_azure_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_azure_host_filters(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_azure_host_filters_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_gcp_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_gcp_integration_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_gcp_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_gcp_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_pager_duty_integration_service(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_pager_duty_integration_service_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_pager_duty_integration_service(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let service_name =
        serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_pager_duty_integration_service_with_http_info(service_name)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_pager_duty_integration_service(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let service_name =
        serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_pager_duty_integration_service_with_http_info(service_name)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_pager_duty_integration_service(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let service_name =
        serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.update_pager_duty_integration_service_with_http_info(service_name, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slack_integration_channels(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let account_name =
        serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap();
    let response = match block_on(api.get_slack_integration_channels_with_http_info(account_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_slack_integration_channel(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let account_name =
        serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.create_slack_integration_channel_with_http_info(account_name, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_remove_slack_integration_channel(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let account_name =
        serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap();
    let channel_name =
        serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap();
    let response = match block_on(
        api.remove_slack_integration_channel_with_http_info(account_name, channel_name),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slack_integration_channel(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let account_name =
        serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap();
    let channel_name =
        serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap();
    let response = match block_on(
        api.get_slack_integration_channel_with_http_info(account_name, channel_name),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_slack_integration_channel(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let account_name =
        serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap();
    let channel_name =
        serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_slack_integration_channel_with_http_info(
        account_name,
        channel_name,
        body,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_webhooks_integration_custom_variable(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.create_webhooks_integration_custom_variable_with_http_info(body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_webhooks_integration_custom_variable(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let custom_variable_name =
        serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap();
    let response = match block_on(
        api.delete_webhooks_integration_custom_variable_with_http_info(custom_variable_name),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_webhooks_integration_custom_variable(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let custom_variable_name =
        serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap();
    let response = match block_on(
        api.get_webhooks_integration_custom_variable_with_http_info(custom_variable_name),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_webhooks_integration_custom_variable(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let custom_variable_name =
        serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.update_webhooks_integration_custom_variable_with_http_info(custom_variable_name, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_webhooks_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_webhooks_integration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_webhooks_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let webhook_name =
        serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_webhooks_integration_with_http_info(webhook_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_webhooks_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let webhook_name =
        serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap();
    let response = match block_on(api.get_webhooks_integration_with_http_info(webhook_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_webhooks_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let webhook_name =
        serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_webhooks_integration_with_http_info(webhook_name, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.list_logs_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_submit_log(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let content_encoding = if let Some(param) = _parameters.get("Content-Encoding") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let ddtags = if let Some(param) = _parameters.get("ddtags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_logs::SubmitLogOptionalParams {
        content_encoding,
        ddtags,
    };
    let response = match block_on(api.submit_log_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_logs_index_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_logs_index_order_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_logs_index_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_index_order_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_log_indexes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_log_indexes_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_logs_index_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let name = serde_json::from_value(_parameters.get("name").unwrap().clone()).unwrap();
    let response = match block_on(api.get_logs_index_with_http_info(name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let name = serde_json::from_value(_parameters.get("name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_index_with_http_info(name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_logs_pipeline_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_logs_pipeline_order_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_logs_pipeline_order(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_pipeline_order_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_logs_pipelines(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_logs_pipelines_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_logs_pipeline(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_logs_pipeline_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_logs_pipeline(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let pipeline_id =
        serde_json::from_value(_parameters.get("pipeline_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_logs_pipeline_with_http_info(pipeline_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_logs_pipeline(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let pipeline_id =
        serde_json::from_value(_parameters.get("pipeline_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_logs_pipeline_with_http_info(pipeline_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_logs_pipeline(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_logs_pipelines
        .as_ref()
        .expect("api instance not found");
    let pipeline_id =
        serde_json::from_value(_parameters.get("pipeline_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_pipeline_with_http_info(pipeline_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_monitors(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let group_states = if let Some(param) = _parameters.get("group_states") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let name = if let Some(param) = _parameters.get("name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tags = if let Some(param) = _parameters.get("tags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let monitor_tags = if let Some(param) = _parameters.get("monitor_tags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let with_downtimes = if let Some(param) = _parameters.get("with_downtimes") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let id_offset = if let Some(param) = _parameters.get("id_offset") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page = if let Some(param) = _parameters.get("page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page_size") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_monitors::ListMonitorsOptionalParams {
        group_states,
        name,
        tags,
        monitor_tags,
        with_downtimes,
        id_offset,
        page,
        page_size,
    };
    let response = match block_on(api.list_monitors_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_monitor(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_monitor_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_check_can_delete_monitor(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let monitor_ids =
        serde_json::from_value(_parameters.get("monitor_ids").unwrap().clone()).unwrap();
    let response = match block_on(api.check_can_delete_monitor_with_http_info(monitor_ids)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_search_monitor_groups(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let query = if let Some(param) = _parameters.get("query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page = if let Some(param) = _parameters.get("page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let per_page = if let Some(param) = _parameters.get("per_page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_monitors::SearchMonitorGroupsOptionalParams {
        query,
        page,
        per_page,
        sort,
    };
    let response = match block_on(api.search_monitor_groups_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_search_monitors(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let query = if let Some(param) = _parameters.get("query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page = if let Some(param) = _parameters.get("page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let per_page = if let Some(param) = _parameters.get("per_page") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_monitors::SearchMonitorsOptionalParams {
        query,
        page,
        per_page,
        sort,
    };
    let response = match block_on(api.search_monitors_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_validate_monitor(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.validate_monitor_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_monitor(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let force = if let Some(param) = _parameters.get("force") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_monitors::DeleteMonitorOptionalParams { force };
    let response = match block_on(api.delete_monitor_with_http_info(monitor_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_monitor(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let group_states = if let Some(param) = _parameters.get("group_states") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let with_downtimes = if let Some(param) = _parameters.get("with_downtimes") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_monitors::GetMonitorOptionalParams {
        group_states,
        with_downtimes,
    };
    let response = match block_on(api.get_monitor_with_http_info(monitor_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_monitor(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_monitor_with_http_info(monitor_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_validate_existing_monitor(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.validate_existing_monitor_with_http_info(monitor_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_notebooks(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let author_handle = if let Some(param) = _parameters.get("author_handle") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let exclude_author_handle = if let Some(param) = _parameters.get("exclude_author_handle") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let start = if let Some(param) = _parameters.get("start") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let count = if let Some(param) = _parameters.get("count") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_field = if let Some(param) = _parameters.get("sort_field") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_dir = if let Some(param) = _parameters.get("sort_dir") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let query = if let Some(param) = _parameters.get("query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_cells = if let Some(param) = _parameters.get("include_cells") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let is_template = if let Some(param) = _parameters.get("is_template") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let type_ = if let Some(param) = _parameters.get("type") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_notebooks::ListNotebooksOptionalParams {
        author_handle,
        exclude_author_handle,
        start,
        count,
        sort_field,
        sort_dir,
        query,
        include_cells,
        is_template,
        type_,
    };
    let response = match block_on(api.list_notebooks_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_notebook(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_notebook_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_notebook(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let notebook_id =
        serde_json::from_value(_parameters.get("notebook_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_notebook_with_http_info(notebook_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_notebook(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let notebook_id =
        serde_json::from_value(_parameters.get("notebook_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_notebook_with_http_info(notebook_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_notebook(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let notebook_id =
        serde_json::from_value(_parameters.get("notebook_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_notebook_with_http_info(notebook_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_orgs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_orgs_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_child_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_child_org_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_org_with_http_info(public_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_org_with_http_info(public_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_downgrade_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let response = match block_on(api.downgrade_org_with_http_info(public_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_upload_id_p_for_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let idp_file = _parameters
        .get("idp_file")
        .unwrap()
        .as_str()
        .unwrap()
        .as_bytes()
        .to_vec();
    let response = match block_on(api.upload_id_p_for_org_with_http_info(public_id, idp_file)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_add_security_monitoring_signal_to_incident(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.add_security_monitoring_signal_to_incident_with_http_info(signal_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_edit_security_monitoring_signal_assignee(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.edit_security_monitoring_signal_assignee_with_http_info(signal_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_edit_security_monitoring_signal_state(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.edit_security_monitoring_signal_state_with_http_info(signal_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_sl_os(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let ids = if let Some(param) = _parameters.get("ids") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let query = if let Some(param) = _parameters.get("query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tags_query = if let Some(param) = _parameters.get("tags_query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let metrics_query = if let Some(param) = _parameters.get("metrics_query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let limit = if let Some(param) = _parameters.get("limit") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let offset = if let Some(param) = _parameters.get("offset") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_service_level_objectives::ListSLOsOptionalParams {
        ids,
        query,
        tags_query,
        metrics_query,
        limit,
        offset,
    };
    let response = match block_on(api.list_sl_os_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_slo_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_slo_timeframe_in_bulk(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_slo_timeframe_in_bulk_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_check_can_delete_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let ids = serde_json::from_value(_parameters.get("ids").unwrap().clone()).unwrap();
    let response = match block_on(api.check_can_delete_slo_with_http_info(ids)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_search_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let query = if let Some(param) = _parameters.get("query") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_facets = if let Some(param) = _parameters.get("include_facets") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_service_level_objectives::SearchSLOOptionalParams {
        query,
        page_size,
        page_number,
        include_facets,
    };
    let response = match block_on(api.search_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let slo_id = serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap();
    let force = if let Some(param) = _parameters.get("force") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_service_level_objectives::DeleteSLOOptionalParams { force };
    let response = match block_on(api.delete_slo_with_http_info(slo_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let slo_id = serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap();
    let with_configured_alert_ids =
        if let Some(param) = _parameters.get("with_configured_alert_ids") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let params = datadogV1::api::api_service_level_objectives::GetSLOOptionalParams {
        with_configured_alert_ids,
    };
    let response = match block_on(api.get_slo_with_http_info(slo_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let slo_id = serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_slo_with_http_info(slo_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slo_corrections(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let slo_id = serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_slo_corrections_with_http_info(slo_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slo_history(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let slo_id = serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap();
    let from_ts = serde_json::from_value(_parameters.get("from_ts").unwrap().clone()).unwrap();
    let to_ts = serde_json::from_value(_parameters.get("to_ts").unwrap().clone()).unwrap();
    let target = if let Some(param) = _parameters.get("target") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let apply_correction = if let Some(param) = _parameters.get("apply_correction") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_service_level_objectives::GetSLOHistoryOptionalParams {
        target,
        apply_correction,
    };
    let response =
        match block_on(api.get_slo_history_with_http_info(slo_id, from_ts, to_ts, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let offset = if let Some(param) = _parameters.get("offset") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let limit = if let Some(param) = _parameters.get("limit") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV1::api::api_service_level_objective_corrections::ListSLOCorrectionOptionalParams {
            offset,
            limit,
        };
    let response = match block_on(api.list_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_slo_correction_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let slo_correction_id =
        serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_slo_correction_with_http_info(slo_correction_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let slo_correction_id =
        serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_slo_correction_with_http_info(slo_correction_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let slo_correction_id =
        serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_slo_correction_with_http_info(slo_correction_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_synthetics_ci_batch(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let batch_id = serde_json::from_value(_parameters.get("batch_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_synthetics_ci_batch_with_http_info(batch_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_locations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_locations_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_private_location(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_private_location_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_private_location(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let location_id =
        serde_json::from_value(_parameters.get("location_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_private_location_with_http_info(location_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_private_location(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let location_id =
        serde_json::from_value(_parameters.get("location_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_private_location_with_http_info(location_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_private_location(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let location_id =
        serde_json::from_value(_parameters.get("location_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_private_location_with_http_info(location_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_synthetics_default_locations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_synthetics_default_locations_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_tests(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page_size") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page_number") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_synthetics::ListTestsOptionalParams {
        page_size,
        page_number,
    };
    let response = match block_on(api.list_tests_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_synthetics_api_test(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_synthetics_api_test_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_api_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_api_test_with_http_info(public_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_api_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_api_test_with_http_info(public_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_synthetics_browser_test(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_synthetics_browser_test_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_browser_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_browser_test_with_http_info(public_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_browser_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_browser_test_with_http_info(public_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_browser_test_latest_results(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let from_ts = if let Some(param) = _parameters.get("from_ts") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let to_ts = if let Some(param) = _parameters.get("to_ts") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let probe_dc = if let Some(param) = _parameters.get("probe_dc") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_synthetics::GetBrowserTestLatestResultsOptionalParams {
        from_ts,
        to_ts,
        probe_dc,
    };
    let response =
        match block_on(api.get_browser_test_latest_results_with_http_info(public_id, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_browser_test_result(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let result_id = serde_json::from_value(_parameters.get("result_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_browser_test_result_with_http_info(public_id, result_id))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_tests(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_tests_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_trigger_tests(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.trigger_tests_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_trigger_ci_tests(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.trigger_ci_tests_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_test_with_http_info(public_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_patch_test(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.patch_test_with_http_info(public_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_api_test_latest_results(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let from_ts = if let Some(param) = _parameters.get("from_ts") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let to_ts = if let Some(param) = _parameters.get("to_ts") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let probe_dc = if let Some(param) = _parameters.get("probe_dc") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_synthetics::GetAPITestLatestResultsOptionalParams {
        from_ts,
        to_ts,
        probe_dc,
    };
    let response = match block_on(api.get_api_test_latest_results_with_http_info(public_id, params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_api_test_result(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let result_id = serde_json::from_value(_parameters.get("result_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_api_test_result_with_http_info(public_id, result_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_test_pause_status(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_test_pause_status_with_http_info(public_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_global_variables(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_global_variables_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_global_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_global_variable_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_global_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let variable_id =
        serde_json::from_value(_parameters.get("variable_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_global_variable_with_http_info(variable_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_global_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let variable_id =
        serde_json::from_value(_parameters.get("variable_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_global_variable_with_http_info(variable_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_edit_global_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let variable_id =
        serde_json::from_value(_parameters.get("variable_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.edit_global_variable_with_http_info(variable_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_tags
        .as_ref()
        .expect("api instance not found");
    let source = if let Some(param) = _parameters.get("source") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_tags::ListHostTagsOptionalParams { source };
    let response = match block_on(api.list_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_delete_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_tags
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let source = if let Some(param) = _parameters.get("source") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_tags::DeleteHostTagsOptionalParams { source };
    let response = match block_on(api.delete_host_tags_with_http_info(host_name, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_tags
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let source = if let Some(param) = _parameters.get("source") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_tags::GetHostTagsOptionalParams { source };
    let response = match block_on(api.get_host_tags_with_http_info(host_name, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_tags
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let source = if let Some(param) = _parameters.get("source") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_tags::CreateHostTagsOptionalParams { source };
    let response = match block_on(api.create_host_tags_with_http_info(host_name, body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_tags
        .as_ref()
        .expect("api instance not found");
    let host_name = serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let source = if let Some(param) = _parameters.get("source") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV1::api::api_tags::UpdateHostTagsOptionalParams { source };
    let response = match block_on(api.update_host_tags_with_http_info(host_name, body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_users(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_users
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_users_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_create_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_users
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_user_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_disable_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_users
        .as_ref()
        .expect("api instance not found");
    let user_handle =
        serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap();
    let response = match block_on(api.disable_user_with_http_info(user_handle)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_get_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_users
        .as_ref()
        .expect("api instance not found");
    let user_handle =
        serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap();
    let response = match block_on(api.get_user_with_http_info(user_handle)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_update_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_users
        .as_ref()
        .expect("api instance not found");
    let user_handle =
        serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_user_with_http_info(user_handle, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_validate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_authentication
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.validate_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_api_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_start = if let Some(param) = _parameters.get("filter[created_at][start]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_end = if let Some(param) = _parameters.get("filter[created_at][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_modified_at_start =
        if let Some(param) = _parameters.get("filter[modified_at][start]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_modified_at_end = if let Some(param) = _parameters.get("filter[modified_at][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_remote_config_read_enabled =
        if let Some(param) = _parameters.get("filter[remote_config_read_enabled]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_category = if let Some(param) = _parameters.get("filter[category]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_key_management::ListAPIKeysOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
        filter_created_at_start,
        filter_created_at_end,
        filter_modified_at_start,
        filter_modified_at_end,
        include,
        filter_remote_config_read_enabled,
        filter_category,
    };
    let response = match block_on(api.list_api_keys_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_api_key_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let api_key_id =
        serde_json::from_value(_parameters.get("api_key_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_api_key_with_http_info(api_key_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let api_key_id =
        serde_json::from_value(_parameters.get("api_key_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_key_management::GetAPIKeyOptionalParams { include };
    let response = match block_on(api.get_api_key_with_http_info(api_key_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let api_key_id =
        serde_json::from_value(_parameters.get("api_key_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_api_key_with_http_info(api_key_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_application_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_start = if let Some(param) = _parameters.get("filter[created_at][start]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_end = if let Some(param) = _parameters.get("filter[created_at][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_key_management::ListApplicationKeysOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
        filter_created_at_start,
        filter_created_at_end,
        include,
    };
    let response = match block_on(api.list_application_keys_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_application_key_with_http_info(app_key_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_key_management::GetApplicationKeyOptionalParams { include };
    let response = match block_on(api.get_application_key_with_http_info(app_key_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_application_key_with_http_info(app_key_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_current_user_application_keys(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_start = if let Some(param) = _parameters.get("filter[created_at][start]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_end = if let Some(param) = _parameters.get("filter[created_at][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_key_management::ListCurrentUserApplicationKeysOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
        filter_created_at_start,
        filter_created_at_end,
        include,
    };
    let response = match block_on(api.list_current_user_application_keys_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_current_user_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_current_user_application_key_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_current_user_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_current_user_application_key_with_http_info(app_key_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_current_user_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_current_user_application_key_with_http_info(app_key_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_current_user_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_key_management
        .as_ref()
        .expect("api instance not found");
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_current_user_application_key_with_http_info(app_key_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_spans_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_spans_metrics_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_spans_metric_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_spans_metric_with_http_info(metric_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_spans_metric_with_http_info(metric_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_spans_metric_with_http_info(metric_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_apm_retention_filters(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_apm_retention_filters_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_apm_retention_filter(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_apm_retention_filter_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_reorder_apm_retention_filters(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.reorder_apm_retention_filters_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_apm_retention_filter(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let filter_id = serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_apm_retention_filter_with_http_info(filter_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_apm_retention_filter(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let filter_id = serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_apm_retention_filter_with_http_info(filter_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_apm_retention_filter(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let filter_id = serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_apm_retention_filter_with_http_info(filter_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_audit
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_audit::ListAuditLogsOptionalParams {
        filter_query,
        filter_from,
        filter_to,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_audit_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_audit
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_audit::SearchAuditLogsOptionalParams { body };
    let response = match block_on(api.search_audit_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_auth_n_mappings(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_auth_n_mappings
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_auth_n_mappings::ListAuthNMappingsOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
    };
    let response = match block_on(api.list_auth_n_mappings_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_auth_n_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_auth_n_mappings
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_auth_n_mapping_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_auth_n_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_auth_n_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_auth_n_mapping_with_http_info(authn_mapping_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_auth_n_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_auth_n_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_auth_n_mapping_with_http_info(authn_mapping_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_auth_n_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_auth_n_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_auth_n_mapping_with_http_info(authn_mapping_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_ci_app_pipeline_event(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_ci_app_pipeline_event_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_aggregate_ci_app_pipeline_events(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.aggregate_ci_app_pipeline_events_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_ci_app_pipeline_events(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_ci_visibility_pipelines::ListCIAppPipelineEventsOptionalParams {
            filter_query,
            filter_from,
            filter_to,
            sort,
            page_cursor,
            page_limit,
        };
    let response = match block_on(api.list_ci_app_pipeline_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_ci_app_pipeline_events(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_ci_visibility_pipelines::SearchCIAppPipelineEventsOptionalParams {
            body,
        };
    let response = match block_on(api.search_ci_app_pipeline_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_aggregate_ci_app_test_events(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_tests
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.aggregate_ci_app_test_events_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_ci_app_test_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_tests
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_ci_visibility_tests::ListCIAppTestEventsOptionalParams {
        filter_query,
        filter_from,
        filter_to,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_ci_app_test_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_ci_app_test_events(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_tests
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_ci_visibility_tests::SearchCIAppTestEventsOptionalParams { body };
    let response = match block_on(api.search_ci_app_test_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_container_images(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_container_images
        .as_ref()
        .expect("api instance not found");
    let filter_tags = if let Some(param) = _parameters.get("filter[tags]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let group_by = if let Some(param) = _parameters.get("group_by") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_container_images::ListContainerImagesOptionalParams {
        filter_tags,
        group_by,
        sort,
        page_size,
        page_cursor,
    };
    let response = match block_on(api.list_container_images_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_containers(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_containers
        .as_ref()
        .expect("api instance not found");
    let filter_tags = if let Some(param) = _parameters.get("filter[tags]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let group_by = if let Some(param) = _parameters.get("group_by") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_containers::ListContainersOptionalParams {
        filter_tags,
        group_by,
        sort,
        page_size,
        page_cursor,
    };
    let response = match block_on(api.list_containers_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_cost_awscur_configs(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_cost_awscur_configs_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_cost_awscur_config(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_cost_awscur_config_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_cost_awscur_config(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let cloud_account_id =
        serde_json::from_value(_parameters.get("cloud_account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_cost_awscur_config_with_http_info(cloud_account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_cost_awscur_config(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let cloud_account_id =
        serde_json::from_value(_parameters.get("cloud_account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_cost_awscur_config_with_http_info(cloud_account_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_aws_related_accounts(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let filter_management_account_id = serde_json::from_value(
        _parameters
            .get("filter[management_account_id]")
            .unwrap()
            .clone(),
    )
    .unwrap();
    let response = match block_on(
        api.list_aws_related_accounts_with_http_info(filter_management_account_id),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_cost_azure_uc_configs(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_cost_azure_uc_configs_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_cost_azure_uc_configs(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_cost_azure_uc_configs_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_cost_azure_uc_config(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let cloud_account_id =
        serde_json::from_value(_parameters.get("cloud_account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_cost_azure_uc_config_with_http_info(cloud_account_id))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_cost_azure_uc_configs(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let cloud_account_id =
        serde_json::from_value(_parameters.get("cloud_account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_cost_azure_uc_configs_with_http_info(cloud_account_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_cloud_cost_activity(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_cloud_cost_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_cloud_cost_activity_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_active_billing_dimensions(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_active_billing_dimensions_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_monthly_cost_attribution(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let end_month = serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap();
    let fields = serde_json::from_value(_parameters.get("fields").unwrap().clone()).unwrap();
    let sort_direction = if let Some(param) = _parameters.get("sort_direction") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_name = if let Some(param) = _parameters.get("sort_name") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tag_breakdown_keys = if let Some(param) = _parameters.get("tag_breakdown_keys") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let next_record_id = if let Some(param) = _parameters.get("next_record_id") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include_descendants = if let Some(param) = _parameters.get("include_descendants") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetMonthlyCostAttributionOptionalParams {
        sort_direction,
        sort_name,
        tag_breakdown_keys,
        next_record_id,
        include_descendants,
    };
    let response = match block_on(api.get_monthly_cost_attribution_with_http_info(
        start_month,
        end_month,
        fields,
        params,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_usage_application_security_monitoring(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_usage_metering::GetUsageApplicationSecurityMonitoringOptionalParams {
            end_hr,
        };
    let response = match block_on(
        api.get_usage_application_security_monitoring_with_http_info(start_hr, params),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_cost_by_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetCostByOrgOptionalParams { end_month };
    let response = match block_on(api.get_cost_by_org_with_http_info(start_month, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_estimated_cost_by_org(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let view = if let Some(param) = _parameters.get("view") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let start_month = if let Some(param) = _parameters.get("start_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let start_date = if let Some(param) = _parameters.get("start_date") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let end_date = if let Some(param) = _parameters.get("end_date") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetEstimatedCostByOrgOptionalParams {
        view,
        start_month,
        end_month,
        start_date,
        end_date,
    };
    let response = match block_on(api.get_estimated_cost_by_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_historical_cost_by_org(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_month =
        serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap();
    let view = if let Some(param) = _parameters.get("view") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let end_month = if let Some(param) = _parameters.get("end_month") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetHistoricalCostByOrgOptionalParams {
        view,
        end_month,
    };
    let response =
        match block_on(api.get_historical_cost_by_org_with_http_info(start_month, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_hourly_usage(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let filter_timestamp_start =
        serde_json::from_value(_parameters.get("filter[timestamp][start]").unwrap().clone())
            .unwrap();
    let filter_product_families =
        serde_json::from_value(_parameters.get("filter[product_families]").unwrap().clone())
            .unwrap();
    let filter_timestamp_end = if let Some(param) = _parameters.get("filter[timestamp][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_include_descendants =
        if let Some(param) = _parameters.get("filter[include_descendants]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_include_breakdown = if let Some(param) = _parameters.get("filter[include_breakdown]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_versions = if let Some(param) = _parameters.get("filter[versions]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_next_record_id = if let Some(param) = _parameters.get("page[next_record_id]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetHourlyUsageOptionalParams {
        filter_timestamp_end,
        filter_include_descendants,
        filter_include_breakdown,
        filter_versions,
        page_limit,
        page_next_record_id,
    };
    let response = match block_on(api.get_hourly_usage_with_http_info(
        filter_timestamp_start,
        filter_product_families,
        params,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_usage_lambda_traced_invocations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_usage_metering::GetUsageLambdaTracedInvocationsOptionalParams {
            end_hr,
        };
    let response =
        match block_on(api.get_usage_lambda_traced_invocations_with_http_info(start_hr, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_usage_observability_pipelines(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let start_hr = serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap();
    let end_hr = if let Some(param) = _parameters.get("end_hr") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_usage_metering::GetUsageObservabilityPipelinesOptionalParams { end_hr };
    let response =
        match block_on(api.get_usage_observability_pipelines_with_http_info(start_hr, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_projected_cost(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let view = if let Some(param) = _parameters.get("view") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_usage_metering::GetProjectedCostOptionalParams { view };
    let response = match block_on(api.get_projected_cost_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_dashboard_list_items(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let dashboard_list_id =
        serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_dashboard_list_items_with_http_info(dashboard_list_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_dashboard_list_items(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let dashboard_list_id =
        serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_dashboard_list_items_with_http_info(dashboard_list_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_dashboard_list_items(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let dashboard_list_id =
        serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.create_dashboard_list_items_with_http_info(dashboard_list_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_dashboard_list_items(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let dashboard_list_id =
        serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_dashboard_list_items_with_http_info(dashboard_list_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_dora_deployment(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_dora_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_dora_deployment_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_dora_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_dora_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_dora_incident_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let current_only = if let Some(param) = _parameters.get("current_only") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_downtimes::ListDowntimesOptionalParams {
        current_only,
        include,
        page_offset,
        page_limit,
    };
    let response = match block_on(api.list_downtimes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_downtime_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_cancel_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let response = match block_on(api.cancel_downtime_with_http_info(downtime_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_downtimes::GetDowntimeOptionalParams { include };
    let response = match block_on(api.get_downtime_with_http_info(downtime_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let downtime_id =
        serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_downtime_with_http_info(downtime_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_monitor_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_downtimes::ListMonitorDowntimesOptionalParams {
        page_offset,
        page_limit,
    };
    let response = match block_on(api.list_monitor_downtimes_with_http_info(monitor_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_events::ListEventsOptionalParams {
        filter_query,
        filter_from,
        filter_to,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_events::SearchEventsOptionalParams { body };
    let response = match block_on(api.search_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incidents(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::ListIncidentsOptionalParams {
        include,
        page_size,
        page_offset,
    };
    let response = match block_on(api.list_incidents_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_incident_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_incidents(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let query = serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::SearchIncidentsOptionalParams {
        include,
        sort,
        page_size,
        page_offset,
    };
    let response = match block_on(api.search_incidents_with_http_info(query, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_incident_with_http_info(incident_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::GetIncidentOptionalParams { include };
    let response = match block_on(api.get_incident_with_http_info(incident_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::UpdateIncidentOptionalParams { include };
    let response = match block_on(api.update_incident_with_http_info(incident_id, body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incident_attachments(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_attachment_type = if let Some(param) = _parameters.get("filter[attachment_type]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::ListIncidentAttachmentsOptionalParams {
        include,
        filter_attachment_type,
    };
    let response = match block_on(api.list_incident_attachments_with_http_info(incident_id, params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident_attachments(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incidents::UpdateIncidentAttachmentsOptionalParams { include };
    let response =
        match block_on(api.update_incident_attachments_with_http_info(incident_id, body, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incident_integrations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_incident_integrations_with_http_info(incident_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_incident_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_incident_integration_with_http_info(incident_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_incident_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let integration_metadata_id =
        serde_json::from_value(_parameters.get("integration_metadata_id").unwrap().clone())
            .unwrap();
    let response = match block_on(
        api.delete_incident_integration_with_http_info(incident_id, integration_metadata_id),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_incident_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let integration_metadata_id =
        serde_json::from_value(_parameters.get("integration_metadata_id").unwrap().clone())
            .unwrap();
    let response = match block_on(
        api.get_incident_integration_with_http_info(incident_id, integration_metadata_id),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident_integration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let integration_metadata_id =
        serde_json::from_value(_parameters.get("integration_metadata_id").unwrap().clone())
            .unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_incident_integration_with_http_info(
        incident_id,
        integration_metadata_id,
        body,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incident_todos(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_incident_todos_with_http_info(incident_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_incident_todo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_incident_todo_with_http_info(incident_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_incident_todo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let todo_id = serde_json::from_value(_parameters.get("todo_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_incident_todo_with_http_info(incident_id, todo_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_incident_todo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let todo_id = serde_json::from_value(_parameters.get("todo_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_incident_todo_with_http_info(incident_id, todo_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident_todo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let incident_id =
        serde_json::from_value(_parameters.get("incident_id").unwrap().clone()).unwrap();
    let todo_id = serde_json::from_value(_parameters.get("todo_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_incident_todo_with_http_info(incident_id, todo_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_gcpsts_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_gcpsts_accounts_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_gcpsts_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_gcpsts_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_gcpsts_account_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_gcpsts_delegate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_gcpsts_delegate_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_make_gcpsts_delegate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_gcp_integration::MakeGCPSTSDelegateOptionalParams { body };
    let response = match block_on(api.make_gcpsts_delegate_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_opsgenie_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_opsgenie_services_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_opsgenie_service_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let integration_service_id =
        serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_opsgenie_service_with_http_info(integration_service_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let integration_service_id =
        serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_opsgenie_service_with_http_info(integration_service_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let integration_service_id =
        serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_opsgenie_service_with_http_info(integration_service_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_cloudflare_accounts(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_cloudflare_accounts_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_cloudflare_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_cloudflare_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_cloudflare_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_cloudflare_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_cloudflare_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_cloudflare_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_cloudflare_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_cloudflare_account_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_confluent_account_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_confluent_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_confluent_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_confluent_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_confluent_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_confluent_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_confluent_account(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_confluent_account_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_confluent_resource_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_confluent_resource(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_confluent_resource_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_confluent_resource(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_confluent_resource_with_http_info(account_id, resource_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_confluent_resource_with_http_info(account_id, resource_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_confluent_resource(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_confluent_resource_with_http_info(account_id, resource_id, body))
        {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_fastly_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_fastly_accounts_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_fastly_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_fastly_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_fastly_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_fastly_account_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_fastly_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_fastly_services_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_fastly_service_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_fastly_service_with_http_info(account_id, service_id))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_fastly_service_with_http_info(account_id, service_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_fastly_service_with_http_info(account_id, service_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_okta_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_okta_integration
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_okta_accounts_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_okta_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_okta_integration
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_okta_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_okta_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_okta_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_okta_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_okta_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_okta_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_okta_account_with_http_info(account_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_okta_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_okta_integration
        .as_ref()
        .expect("api instance not found");
    let account_id =
        serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_okta_account_with_http_info(account_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_ip_allowlist(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_ip_allowlist
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_ip_allowlist_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_ip_allowlist(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_ip_allowlist
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_ip_allowlist_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_submit_log(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let content_encoding = if let Some(param) = _parameters.get("Content-Encoding") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let ddtags = if let Some(param) = _parameters.get("ddtags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_logs::SubmitLogOptionalParams {
        content_encoding,
        ddtags,
    };
    let response = match block_on(api.submit_log_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_aggregate_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.aggregate_logs_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_logs_get(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_indexes = if let Some(param) = _parameters.get("filter[indexes]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_storage_tier = if let Some(param) = _parameters.get("filter[storage_tier]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_logs::ListLogsGetOptionalParams {
        filter_query,
        filter_indexes,
        filter_from,
        filter_to,
        filter_storage_tier,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_logs_get_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_logs::ListLogsOptionalParams { body };
    let response = match block_on(api.list_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_logs_archive_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_logs_archive_order_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_logs_archive_order(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_archive_order_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_logs_archives(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_logs_archives_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_logs_archive(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_logs_archive_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_logs_archive(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_logs_archive_with_http_info(archive_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_logs_archive(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_logs_archive_with_http_info(archive_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_logs_archive(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_archive_with_http_info(archive_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_remove_role_from_archive(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.remove_role_from_archive_with_http_info(archive_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_archive_read_roles(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_archive_read_roles_with_http_info(archive_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_add_read_role_to_archive(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_archives
        .as_ref()
        .expect("api instance not found");
    let archive_id =
        serde_json::from_value(_parameters.get("archive_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.add_read_role_to_archive_with_http_info(archive_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_logs_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_logs_metrics_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_logs_metric_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_logs_metric_with_http_info(metric_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_logs_metric_with_http_info(metric_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_id = serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_logs_metric_with_http_info(metric_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_tag_configurations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let filter_configured = if let Some(param) = _parameters.get("filter[configured]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_tags_configured = if let Some(param) = _parameters.get("filter[tags_configured]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_metric_type = if let Some(param) = _parameters.get("filter[metric_type]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_include_percentiles =
        if let Some(param) = _parameters.get("filter[include_percentiles]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_queried = if let Some(param) = _parameters.get("filter[queried]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_tags = if let Some(param) = _parameters.get("filter[tags]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let window_seconds = if let Some(param) = _parameters.get("window[seconds]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_metrics::ListTagConfigurationsOptionalParams {
        filter_configured,
        filter_tags_configured,
        filter_metric_type,
        filter_include_percentiles,
        filter_queried,
        filter_tags,
        window_seconds,
    };
    let response = match block_on(api.list_tag_configurations_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_bulk_tags_metrics_configuration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_bulk_tags_metrics_configuration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_bulk_tags_metrics_configuration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_bulk_tags_metrics_configuration_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_active_metric_configurations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let window_seconds = if let Some(param) = _parameters.get("window[seconds]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_metrics::ListActiveMetricConfigurationsOptionalParams {
        window_seconds,
    };
    let response =
        match block_on(api.list_active_metric_configurations_with_http_info(metric_name, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_tags_by_metric_name(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.list_tags_by_metric_name_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_estimate_metrics_output_series(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let filter_groups = if let Some(param) = _parameters.get("filter[groups]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_hours_ago = if let Some(param) = _parameters.get("filter[hours_ago]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_num_aggregations = if let Some(param) = _parameters.get("filter[num_aggregations]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_pct = if let Some(param) = _parameters.get("filter[pct]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_timespan_h = if let Some(param) = _parameters.get("filter[timespan_h]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_metrics::EstimateMetricsOutputSeriesOptionalParams {
        filter_groups,
        filter_hours_ago,
        filter_num_aggregations,
        filter_pct,
        filter_timespan_h,
    };
    let response =
        match block_on(api.estimate_metrics_output_series_with_http_info(metric_name, params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_tag_configuration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_tag_configuration_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_tag_configuration_by_name(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.list_tag_configuration_by_name_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_tag_configuration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_tag_configuration_with_http_info(metric_name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_tag_configuration(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_tag_configuration_with_http_info(metric_name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_volumes_by_metric_name(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.list_volumes_by_metric_name_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_query_scalar_data(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.query_scalar_data_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_query_timeseries_data(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.query_timeseries_data_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_submit_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let content_encoding = if let Some(param) = _parameters.get("Content-Encoding") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_metrics::SubmitMetricsOptionalParams { content_encoding };
    let response = match block_on(api.submit_metrics_with_http_info(body, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_monitor_config_policies(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_monitors
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_monitor_config_policies_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_monitor_config_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_monitors
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_monitor_config_policy_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_monitor_config_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_monitors
        .as_ref()
        .expect("api instance not found");
    let policy_id = serde_json::from_value(_parameters.get("policy_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_monitor_config_policy_with_http_info(policy_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_monitor_config_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_monitors
        .as_ref()
        .expect("api instance not found");
    let policy_id = serde_json::from_value(_parameters.get("policy_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_monitor_config_policy_with_http_info(policy_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_monitor_config_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_monitors
        .as_ref()
        .expect("api instance not found");
    let policy_id = serde_json::from_value(_parameters.get("policy_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_monitor_config_policy_with_http_info(policy_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_permissions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_permissions_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_roles(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_id = if let Some(param) = _parameters.get("filter[id]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_roles::ListRolesOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
        filter_id,
    };
    let response = match block_on(api.list_roles_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_role_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_role_with_http_info(role_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_role_with_http_info(role_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_clone_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.clone_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_remove_permission_from_role(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.remove_permission_from_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_role_permissions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_role_permissions_with_http_info(role_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_add_permission_to_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.add_permission_to_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_remove_user_from_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.remove_user_from_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_role_users(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_roles::ListRoleUsersOptionalParams {
        page_size,
        page_number,
        sort,
        filter,
    };
    let response = match block_on(api.list_role_users_with_http_info(role_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_add_user_to_role(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_roles
        .as_ref()
        .expect("api instance not found");
    let role_id = serde_json::from_value(_parameters.get("role_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.add_user_to_role_with_http_info(role_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_findings(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let snapshot_timestamp = if let Some(param) = _parameters.get("snapshot_timestamp") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_tags = if let Some(param) = _parameters.get("filter[tags]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_evaluation_changed_at =
        if let Some(param) = _parameters.get("filter[evaluation_changed_at]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_muted = if let Some(param) = _parameters.get("filter[muted]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_id = if let Some(param) = _parameters.get("filter[rule_id]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_name = if let Some(param) = _parameters.get("filter[rule_name]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_resource_type = if let Some(param) = _parameters.get("filter[resource_type]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_discovery_timestamp =
        if let Some(param) = _parameters.get("filter[discovery_timestamp]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_evaluation = if let Some(param) = _parameters.get("filter[evaluation]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_status = if let Some(param) = _parameters.get("filter[status]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_security_monitoring::ListFindingsOptionalParams {
        page_limit,
        snapshot_timestamp,
        page_cursor,
        filter_tags,
        filter_evaluation_changed_at,
        filter_muted,
        filter_rule_id,
        filter_rule_name,
        filter_resource_type,
        filter_discovery_timestamp,
        filter_evaluation,
        filter_status,
    };
    let response = match block_on(api.list_findings_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_mute_findings(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.mute_findings_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_finding(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let finding_id =
        serde_json::from_value(_parameters.get("finding_id").unwrap().clone()).unwrap();
    let snapshot_timestamp = if let Some(param) = _parameters.get("snapshot_timestamp") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_security_monitoring::GetFindingOptionalParams { snapshot_timestamp };
    let response = match block_on(api.get_finding_with_http_info(finding_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_security_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_security_filters_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_security_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_security_filter_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_security_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let security_filter_id =
        serde_json::from_value(_parameters.get("security_filter_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_security_filter_with_http_info(security_filter_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_security_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let security_filter_id =
        serde_json::from_value(_parameters.get("security_filter_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_security_filter_with_http_info(security_filter_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_security_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let security_filter_id =
        serde_json::from_value(_parameters.get("security_filter_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_security_filter_with_http_info(security_filter_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_security_monitoring_rules(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_security_monitoring::ListSecurityMonitoringRulesOptionalParams {
            page_size,
            page_number,
        };
    let response = match block_on(api.list_security_monitoring_rules_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_security_monitoring_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_security_monitoring_rule_with_http_info(rule_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_security_monitoring_rule_with_http_info(rule_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_security_monitoring_rule_with_http_info(rule_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_security_monitoring_signals(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_security_monitoring::ListSecurityMonitoringSignalsOptionalParams {
            filter_query,
            filter_from,
            filter_to,
            sort,
            page_cursor,
            page_limit,
        };
    let response = match block_on(api.list_security_monitoring_signals_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_security_monitoring_signals(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = if let Some(param) = _parameters.get("body") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_security_monitoring::SearchSecurityMonitoringSignalsOptionalParams {
            body,
        };
    let response = match block_on(api.search_security_monitoring_signals_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_security_monitoring_signal(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_security_monitoring_signal_with_http_info(signal_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_edit_security_monitoring_signal_assignee(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.edit_security_monitoring_signal_assignee_with_http_info(signal_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_edit_security_monitoring_signal_incidents(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.edit_security_monitoring_signal_incidents_with_http_info(signal_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_edit_security_monitoring_signal_state(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let signal_id = serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.edit_security_monitoring_signal_state_with_http_info(signal_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_powerpacks(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_powerpack::ListPowerpacksOptionalParams {
        page_limit,
        page_offset,
    };
    let response = match block_on(api.list_powerpacks_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_powerpack(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_powerpack_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_powerpack(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let powerpack_id =
        serde_json::from_value(_parameters.get("powerpack_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_powerpack_with_http_info(powerpack_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_powerpack(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let powerpack_id =
        serde_json::from_value(_parameters.get("powerpack_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_powerpack_with_http_info(powerpack_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_powerpack(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let powerpack_id =
        serde_json::from_value(_parameters.get("powerpack_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_powerpack_with_http_info(powerpack_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_processes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_processes
        .as_ref()
        .expect("api instance not found");
    let search = if let Some(param) = _parameters.get("search") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let tags = if let Some(param) = _parameters.get("tags") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let from = if let Some(param) = _parameters.get("from") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let to = if let Some(param) = _parameters.get("to") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_processes::ListProcessesOptionalParams {
        search,
        tags,
        from,
        to,
        page_limit,
        page_cursor,
    };
    let response = match block_on(api.list_processes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_restriction_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_restriction_policy_with_http_info(resource_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_restriction_policy(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_restriction_policy_with_http_info(resource_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_restriction_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let resource_id =
        serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_restriction_policy_with_http_info(resource_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_aggregate_rum_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.aggregate_rum_events_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_rum_applications(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_rum_applications_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_rum_application(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_rum_application_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_rum_application(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_rum_application_with_http_info(id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_rum_application(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_rum_application_with_http_info(id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_rum_application(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_rum_application_with_http_info(id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_rum_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_rum::ListRUMEventsOptionalParams {
        filter_query,
        filter_from,
        filter_to,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_rum_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_rum_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.search_rum_events_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_upload_id_p_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_organizations
        .as_ref()
        .expect("api instance not found");
    let idp_file = if let Some(param) = _parameters.get("idp_file") {
        Some(param.as_str().unwrap().as_bytes().to_vec())
    } else {
        None
    };
    let params = datadogV2::api::api_organizations::UploadIdPMetadataOptionalParams { idp_file };
    let response = match block_on(api.upload_id_p_metadata_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_scorecard_outcomes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let fields_outcome = if let Some(param) = _parameters.get("fields[outcome]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let fields_rule = if let Some(param) = _parameters.get("fields[rule]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_outcome_service_name =
        if let Some(param) = _parameters.get("filter[outcome][service_name]") {
            Some(serde_json::from_value(param.clone()).unwrap())
        } else {
            None
        };
    let filter_outcome_state = if let Some(param) = _parameters.get("filter[outcome][state]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_enabled = if let Some(param) = _parameters.get("filter[rule][enabled]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_id = if let Some(param) = _parameters.get("filter[rule][id]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_name = if let Some(param) = _parameters.get("filter[rule][name]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_service_scorecards::ListScorecardOutcomesOptionalParams {
        page_size,
        page_offset,
        include,
        fields_outcome,
        fields_rule,
        filter_outcome_service_name,
        filter_outcome_state,
        filter_rule_enabled,
        filter_rule_id,
        filter_rule_name,
    };
    let response = match block_on(api.list_scorecard_outcomes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_scorecard_outcomes_batch(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_scorecard_outcomes_batch_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_scorecard_rules(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_id = if let Some(param) = _parameters.get("filter[rule][id]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_enabled = if let Some(param) = _parameters.get("filter[rule][enabled]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_custom = if let Some(param) = _parameters.get("filter[rule][custom]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_name = if let Some(param) = _parameters.get("filter[rule][name]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_rule_description = if let Some(param) = _parameters.get("filter[rule][description]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let fields_rule = if let Some(param) = _parameters.get("fields[rule]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let fields_scorecard = if let Some(param) = _parameters.get("fields[scorecard]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_service_scorecards::ListScorecardRulesOptionalParams {
        page_size,
        page_offset,
        include,
        filter_rule_id,
        filter_rule_enabled,
        filter_rule_custom,
        filter_rule_name,
        filter_rule_description,
        fields_rule,
        fields_scorecard,
    };
    let response = match block_on(api.list_scorecard_rules_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_scorecard_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_scorecard_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_scorecard_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_scorecard_rule_with_http_info(rule_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_download_cloud_workload_policy_file(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.download_cloud_workload_policy_file_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_cloud_workload_security_agent_rules(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_cloud_workload_security_agent_rules_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_cloud_workload_security_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.create_cloud_workload_security_agent_rule_with_http_info(body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_cloud_workload_security_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_cloud_workload_security_agent_rule_with_http_info(agent_rule_id))
        {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_cloud_workload_security_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_cloud_workload_security_agent_rule_with_http_info(agent_rule_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_cloud_workload_security_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.update_cloud_workload_security_agent_rule_with_http_info(agent_rule_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_scanning_groups(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_scanning_groups_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_reorder_scanning_groups(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.reorder_scanning_groups_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_scanning_group(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_scanning_group_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_scanning_group(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let group_id = serde_json::from_value(_parameters.get("group_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_scanning_group_with_http_info(group_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_scanning_group(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let group_id = serde_json::from_value(_parameters.get("group_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_scanning_group_with_http_info(group_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_scanning_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_scanning_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_scanning_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_scanning_rule_with_http_info(rule_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_scanning_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_scanning_rule_with_http_info(rule_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_standard_patterns(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_sensitive_data_scanner
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_standard_patterns_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_service_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_service_account_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_service_account_application_keys(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let service_account_id =
        serde_json::from_value(_parameters.get("service_account_id").unwrap().clone()).unwrap();
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_start = if let Some(param) = _parameters.get("filter[created_at][start]")
    {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_created_at_end = if let Some(param) = _parameters.get("filter[created_at][end]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_service_accounts::ListServiceAccountApplicationKeysOptionalParams {
            page_size,
            page_number,
            sort,
            filter,
            filter_created_at_start,
            filter_created_at_end,
        };
    let response = match block_on(
        api.list_service_account_application_keys_with_http_info(service_account_id, params),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_service_account_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let service_account_id =
        serde_json::from_value(_parameters.get("service_account_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.create_service_account_application_key_with_http_info(service_account_id, body),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_service_account_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let service_account_id =
        serde_json::from_value(_parameters.get("service_account_id").unwrap().clone()).unwrap();
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let response = match block_on(
        api.delete_service_account_application_key_with_http_info(service_account_id, app_key_id),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_service_account_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let service_account_id =
        serde_json::from_value(_parameters.get("service_account_id").unwrap().clone()).unwrap();
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let response = match block_on(
        api.get_service_account_application_key_with_http_info(service_account_id, app_key_id),
    ) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_service_account_application_key(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_accounts
        .as_ref()
        .expect("api instance not found");
    let service_account_id =
        serde_json::from_value(_parameters.get("service_account_id").unwrap().clone()).unwrap();
    let app_key_id =
        serde_json::from_value(_parameters.get("app_key_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_service_account_application_key_with_http_info(
        service_account_id,
        app_key_id,
        body,
    )) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incident_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_services
        .as_ref()
        .expect("api instance not found");
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incident_services::ListIncidentServicesOptionalParams {
        include,
        page_size,
        page_offset,
        filter,
    };
    let response = match block_on(api.list_incident_services_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_incident_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_services
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_incident_service_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_incident_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_services
        .as_ref()
        .expect("api instance not found");
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_incident_service_with_http_info(service_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_incident_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_services
        .as_ref()
        .expect("api instance not found");
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params =
        datadogV2::api::api_incident_services::GetIncidentServiceOptionalParams { include };
    let response = match block_on(api.get_incident_service_with_http_info(service_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_services
        .as_ref()
        .expect("api instance not found");
    let service_id =
        serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_incident_service_with_http_info(service_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_service_definitions(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_definition
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let schema_version = if let Some(param) = _parameters.get("schema_version") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_service_definition::ListServiceDefinitionsOptionalParams {
        page_size,
        page_number,
        schema_version,
    };
    let response = match block_on(api.list_service_definitions_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_or_update_service_definitions(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_definition
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_or_update_service_definitions_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_service_definition(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_definition
        .as_ref()
        .expect("api instance not found");
    let service_name =
        serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_service_definition_with_http_info(service_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_service_definition(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_definition
        .as_ref()
        .expect("api instance not found");
    let service_name =
        serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap();
    let schema_version = if let Some(param) = _parameters.get("schema_version") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_service_definition::GetServiceDefinitionOptionalParams {
        schema_version,
    };
    let response = match block_on(api.get_service_definition_with_http_info(service_name, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_aggregate_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.aggregate_spans_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_spans_get(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans
        .as_ref()
        .expect("api instance not found");
    let filter_query = if let Some(param) = _parameters.get("filter[query]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_from = if let Some(param) = _parameters.get("filter[from]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_to = if let Some(param) = _parameters.get("filter[to]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_cursor = if let Some(param) = _parameters.get("page[cursor]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_limit = if let Some(param) = _parameters.get("page[limit]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_spans::ListSpansGetOptionalParams {
        filter_query,
        filter_from,
        filter_to,
        sort,
        page_cursor,
        page_limit,
    };
    let response = match block_on(api.list_spans_get_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_spans
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.list_spans_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_on_demand_concurrency_cap(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_on_demand_concurrency_cap_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_set_on_demand_concurrency_cap(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.set_on_demand_concurrency_cap_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_teams(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_keyword = if let Some(param) = _parameters.get("filter[keyword]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_me = if let Some(param) = _parameters.get("filter[me]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let fields_team = if let Some(param) = _parameters.get("fields[team]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_teams::ListTeamsOptionalParams {
        page_number,
        page_size,
        sort,
        include,
        filter_keyword,
        filter_me,
        fields_team,
    };
    let response = match block_on(api.list_teams_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_team_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_team_with_http_info(team_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_team_with_http_info(team_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_team_with_http_info(team_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_team_links(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_team_links_with_http_info(team_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_team_link(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_team_link_with_http_info(team_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_team_link(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let link_id = serde_json::from_value(_parameters.get("link_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_team_link_with_http_info(team_id, link_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_team_link(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let link_id = serde_json::from_value(_parameters.get("link_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_team_link_with_http_info(team_id, link_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_team_link(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let link_id = serde_json::from_value(_parameters.get("link_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_team_link_with_http_info(team_id, link_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_team_memberships(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_keyword = if let Some(param) = _parameters.get("filter[keyword]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_teams::GetTeamMembershipsOptionalParams {
        page_size,
        page_number,
        sort,
        filter_keyword,
    };
    let response = match block_on(api.get_team_memberships_with_http_info(team_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_team_membership(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_team_membership_with_http_info(team_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_team_membership(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_team_membership_with_http_info(team_id, user_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_team_membership(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_team_membership_with_http_info(team_id, user_id, body))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_team_permission_settings(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_team_permission_settings_with_http_info(team_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_team_permission_setting(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let action = serde_json::from_value(_parameters.get("action").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_team_permission_setting_with_http_info(team_id, action, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {}", error),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_user_memberships(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let user_uuid = serde_json::from_value(_parameters.get("user_uuid").unwrap().clone()).unwrap();
    let response = match block_on(api.get_user_memberships_with_http_info(user_uuid)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_incident_teams(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_teams
        .as_ref()
        .expect("api instance not found");
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_offset = if let Some(param) = _parameters.get("page[offset]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incident_teams::ListIncidentTeamsOptionalParams {
        include,
        page_size,
        page_offset,
        filter,
    };
    let response = match block_on(api.list_incident_teams_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_incident_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_teams
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_incident_team_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_incident_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_incident_team_with_http_info(team_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_incident_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let include = if let Some(param) = _parameters.get("include") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_incident_teams::GetIncidentTeamOptionalParams { include };
    let response = match block_on(api.get_incident_team_with_http_info(team_id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_incident_team(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incident_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_incident_team_with_http_info(team_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_send_invitations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.send_invitations_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_invitation(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_invitation_uuid =
        serde_json::from_value(_parameters.get("user_invitation_uuid").unwrap().clone()).unwrap();
    let response = match block_on(api.get_invitation_with_http_info(user_invitation_uuid)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_users(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let page_size = if let Some(param) = _parameters.get("page[size]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let page_number = if let Some(param) = _parameters.get("page[number]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort = if let Some(param) = _parameters.get("sort") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let sort_dir = if let Some(param) = _parameters.get("sort_dir") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter = if let Some(param) = _parameters.get("filter") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let filter_status = if let Some(param) = _parameters.get("filter[status]") {
        Some(serde_json::from_value(param.clone()).unwrap())
    } else {
        None
    };
    let params = datadogV2::api::api_users::ListUsersOptionalParams {
        page_size,
        page_number,
        sort,
        sort_dir,
        filter,
        filter_status,
    };
    let response = match block_on(api.list_users_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_user_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_disable_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let response = match block_on(api.disable_user_with_http_info(user_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_user_with_http_info(user_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_user_with_http_info(user_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_user_organizations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_user_organizations_with_http_info(user_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_user_permissions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let user_id = serde_json::from_value(_parameters.get("user_id").unwrap().clone()).unwrap();
    let response = match block_on(api.list_user_permissions_with_http_info(user_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
