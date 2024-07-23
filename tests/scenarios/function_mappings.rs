use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;
use datadog_api_client::datadogV1;
use datadog_api_client::datadogV2;

#[derive(Debug, Default)]
pub struct ApiInstances {
    pub v1_api_ip_ranges: Option<datadogV1::api_ip_ranges::IPRangesAPI>,
    pub v1_api_key_management: Option<datadogV1::api_key_management::KeyManagementAPI>,
    pub v1_api_service_checks: Option<datadogV1::api_service_checks::ServiceChecksAPI>,
    pub v1_api_usage_metering: Option<datadogV1::api_usage_metering::UsageMeteringAPI>,
    pub v1_api_dashboards: Option<datadogV1::api_dashboards::DashboardsAPI>,
    pub v1_api_dashboard_lists: Option<datadogV1::api_dashboard_lists::DashboardListsAPI>,
    pub v1_api_metrics: Option<datadogV1::api_metrics::MetricsAPI>,
    pub v1_api_downtimes: Option<datadogV1::api_downtimes::DowntimesAPI>,
    pub v1_api_events: Option<datadogV1::api_events::EventsAPI>,
    pub v1_api_snapshots: Option<datadogV1::api_snapshots::SnapshotsAPI>,
    pub v1_api_hosts: Option<datadogV1::api_hosts::HostsAPI>,
    pub v1_api_aws_integration: Option<datadogV1::api_aws_integration::AWSIntegrationAPI>,
    pub v1_api_aws_logs_integration:
        Option<datadogV1::api_aws_logs_integration::AWSLogsIntegrationAPI>,
    pub v1_api_azure_integration: Option<datadogV1::api_azure_integration::AzureIntegrationAPI>,
    pub v1_api_gcp_integration: Option<datadogV1::api_gcp_integration::GCPIntegrationAPI>,
    pub v1_api_pager_duty_integration:
        Option<datadogV1::api_pager_duty_integration::PagerDutyIntegrationAPI>,
    pub v1_api_slack_integration: Option<datadogV1::api_slack_integration::SlackIntegrationAPI>,
    pub v1_api_webhooks_integration:
        Option<datadogV1::api_webhooks_integration::WebhooksIntegrationAPI>,
    pub v1_api_logs: Option<datadogV1::api_logs::LogsAPI>,
    pub v1_api_logs_indexes: Option<datadogV1::api_logs_indexes::LogsIndexesAPI>,
    pub v1_api_logs_pipelines: Option<datadogV1::api_logs_pipelines::LogsPipelinesAPI>,
    pub v1_api_monitors: Option<datadogV1::api_monitors::MonitorsAPI>,
    pub v1_api_notebooks: Option<datadogV1::api_notebooks::NotebooksAPI>,
    pub v1_api_organizations: Option<datadogV1::api_organizations::OrganizationsAPI>,
    pub v1_api_security_monitoring:
        Option<datadogV1::api_security_monitoring::SecurityMonitoringAPI>,
    pub v1_api_service_level_objectives:
        Option<datadogV1::api_service_level_objectives::ServiceLevelObjectivesAPI>,
    pub v1_api_service_level_objective_corrections: Option<
        datadogV1::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI,
    >,
    pub v1_api_synthetics: Option<datadogV1::api_synthetics::SyntheticsAPI>,
    pub v1_api_tags: Option<datadogV1::api_tags::TagsAPI>,
    pub v1_api_users: Option<datadogV1::api_users::UsersAPI>,
    pub v1_api_authentication: Option<datadogV1::api_authentication::AuthenticationAPI>,
    pub v2_api_key_management: Option<datadogV2::api_key_management::KeyManagementAPI>,
    pub v2_api_api_management: Option<datadogV2::api_api_management::APIManagementAPI>,
    pub v2_api_spans_metrics: Option<datadogV2::api_spans_metrics::SpansMetricsAPI>,
    pub v2_api_apm_retention_filters:
        Option<datadogV2::api_apm_retention_filters::APMRetentionFiltersAPI>,
    pub v2_api_audit: Option<datadogV2::api_audit::AuditAPI>,
    pub v2_api_authn_mappings: Option<datadogV2::api_authn_mappings::AuthNMappingsAPI>,
    pub v2_api_case_management: Option<datadogV2::api_case_management::CaseManagementAPI>,
    pub v2_api_ci_visibility_pipelines:
        Option<datadogV2::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI>,
    pub v2_api_ci_visibility_tests:
        Option<datadogV2::api_ci_visibility_tests::CIVisibilityTestsAPI>,
    pub v2_api_container_images: Option<datadogV2::api_container_images::ContainerImagesAPI>,
    pub v2_api_containers: Option<datadogV2::api_containers::ContainersAPI>,
    pub v2_api_cloud_cost_management:
        Option<datadogV2::api_cloud_cost_management::CloudCostManagementAPI>,
    pub v2_api_usage_metering: Option<datadogV2::api_usage_metering::UsageMeteringAPI>,
    pub v2_api_dashboard_lists: Option<datadogV2::api_dashboard_lists::DashboardListsAPI>,
    pub v2_api_dora_metrics: Option<datadogV2::api_dora_metrics::DORAMetricsAPI>,
    pub v2_api_downtimes: Option<datadogV2::api_downtimes::DowntimesAPI>,
    pub v2_api_events: Option<datadogV2::api_events::EventsAPI>,
    pub v2_api_incidents: Option<datadogV2::api_incidents::IncidentsAPI>,
    pub v2_api_gcp_integration: Option<datadogV2::api_gcp_integration::GCPIntegrationAPI>,
    pub v2_api_opsgenie_integration:
        Option<datadogV2::api_opsgenie_integration::OpsgenieIntegrationAPI>,
    pub v2_api_cloudflare_integration:
        Option<datadogV2::api_cloudflare_integration::CloudflareIntegrationAPI>,
    pub v2_api_confluent_cloud: Option<datadogV2::api_confluent_cloud::ConfluentCloudAPI>,
    pub v2_api_fastly_integration: Option<datadogV2::api_fastly_integration::FastlyIntegrationAPI>,
    pub v2_api_okta_integration: Option<datadogV2::api_okta_integration::OktaIntegrationAPI>,
    pub v2_api_ip_allowlist: Option<datadogV2::api_ip_allowlist::IPAllowlistAPI>,
    pub v2_api_logs: Option<datadogV2::api_logs::LogsAPI>,
    pub v2_api_logs_archives: Option<datadogV2::api_logs_archives::LogsArchivesAPI>,
    pub v2_api_logs_custom_destinations:
        Option<datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI>,
    pub v2_api_logs_metrics: Option<datadogV2::api_logs_metrics::LogsMetricsAPI>,
    pub v2_api_metrics: Option<datadogV2::api_metrics::MetricsAPI>,
    pub v2_api_monitors: Option<datadogV2::api_monitors::MonitorsAPI>,
    pub v2_api_network_device_monitoring:
        Option<datadogV2::api_network_device_monitoring::NetworkDeviceMonitoringAPI>,
    pub v2_api_organizations: Option<datadogV2::api_organizations::OrganizationsAPI>,
    pub v2_api_roles: Option<datadogV2::api_roles::RolesAPI>,
    pub v2_api_security_monitoring:
        Option<datadogV2::api_security_monitoring::SecurityMonitoringAPI>,
    pub v2_api_powerpack: Option<datadogV2::api_powerpack::PowerpackAPI>,
    pub v2_api_processes: Option<datadogV2::api_processes::ProcessesAPI>,
    pub v2_api_csm_threats: Option<datadogV2::api_csm_threats::CSMThreatsAPI>,
    pub v2_api_restriction_policies:
        Option<datadogV2::api_restriction_policies::RestrictionPoliciesAPI>,
    pub v2_api_rum: Option<datadogV2::api_rum::RUMAPI>,
    pub v2_api_service_scorecards: Option<datadogV2::api_service_scorecards::ServiceScorecardsAPI>,
    pub v2_api_sensitive_data_scanner:
        Option<datadogV2::api_sensitive_data_scanner::SensitiveDataScannerAPI>,
    pub v2_api_service_accounts: Option<datadogV2::api_service_accounts::ServiceAccountsAPI>,
    pub v2_api_incident_services: Option<datadogV2::api_incident_services::IncidentServicesAPI>,
    pub v2_api_service_definition: Option<datadogV2::api_service_definition::ServiceDefinitionAPI>,
    pub v2_api_service_level_objectives:
        Option<datadogV2::api_service_level_objectives::ServiceLevelObjectivesAPI>,
    pub v2_api_spans: Option<datadogV2::api_spans::SpansAPI>,
    pub v2_api_synthetics: Option<datadogV2::api_synthetics::SyntheticsAPI>,
    pub v2_api_teams: Option<datadogV2::api_teams::TeamsAPI>,
    pub v2_api_incident_teams: Option<datadogV2::api_incident_teams::IncidentTeamsAPI>,
    pub v2_api_users: Option<datadogV2::api_users::UsersAPI>,
}

pub fn initialize_api_instance(world: &mut DatadogWorld, api: String) {
    match api.as_str() {
        "IPRanges" => {
            world.api_instances.v1_api_ip_ranges = Some(
                datadogV1::api_ip_ranges::IPRangesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "KeyManagement" => {
            world.api_instances.v1_api_key_management = Some(
                datadogV1::api_key_management::KeyManagementAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_key_management = Some(
                datadogV2::api_key_management::KeyManagementAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "ServiceChecks" => {
            world.api_instances.v1_api_service_checks = Some(
                datadogV1::api_service_checks::ServiceChecksAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "UsageMetering" => {
            world.api_instances.v1_api_usage_metering = Some(
                datadogV1::api_usage_metering::UsageMeteringAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_usage_metering = Some(
                datadogV2::api_usage_metering::UsageMeteringAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Dashboards" => {
            world.api_instances.v1_api_dashboards = Some(
                datadogV1::api_dashboards::DashboardsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "DashboardLists" => {
            world.api_instances.v1_api_dashboard_lists = Some(
                datadogV1::api_dashboard_lists::DashboardListsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_dashboard_lists = Some(
                datadogV2::api_dashboard_lists::DashboardListsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Metrics" => {
            world.api_instances.v1_api_metrics =
                Some(datadogV1::api_metrics::MetricsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
            world.api_instances.v2_api_metrics =
                Some(datadogV2::api_metrics::MetricsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Downtimes" => {
            world.api_instances.v1_api_downtimes = Some(
                datadogV1::api_downtimes::DowntimesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_downtimes = Some(
                datadogV2::api_downtimes::DowntimesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Events" => {
            world.api_instances.v1_api_events =
                Some(datadogV1::api_events::EventsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
            world.api_instances.v2_api_events =
                Some(datadogV2::api_events::EventsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Snapshots" => {
            world.api_instances.v1_api_snapshots = Some(
                datadogV1::api_snapshots::SnapshotsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Hosts" => {
            world.api_instances.v1_api_hosts =
                Some(datadogV1::api_hosts::HostsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "AWSIntegration" => {
            world.api_instances.v1_api_aws_integration = Some(
                datadogV1::api_aws_integration::AWSIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "AWSLogsIntegration" => {
            world.api_instances.v1_api_aws_logs_integration = Some(
                datadogV1::api_aws_logs_integration::AWSLogsIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "AzureIntegration" => {
            world.api_instances.v1_api_azure_integration = Some(
                datadogV1::api_azure_integration::AzureIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "GCPIntegration" => {
            world.api_instances.v1_api_gcp_integration = Some(
                datadogV1::api_gcp_integration::GCPIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_gcp_integration = Some(
                datadogV2::api_gcp_integration::GCPIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "PagerDutyIntegration" => {
            world.api_instances.v1_api_pager_duty_integration = Some(datadogV1::api_pager_duty_integration::PagerDutyIntegrationAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "SlackIntegration" => {
            world.api_instances.v1_api_slack_integration = Some(
                datadogV1::api_slack_integration::SlackIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "WebhooksIntegration" => {
            world.api_instances.v1_api_webhooks_integration = Some(
                datadogV1::api_webhooks_integration::WebhooksIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Logs" => {
            world.api_instances.v1_api_logs =
                Some(datadogV1::api_logs::LogsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
            world.api_instances.v2_api_logs =
                Some(datadogV2::api_logs::LogsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "LogsIndexes" => {
            world.api_instances.v1_api_logs_indexes = Some(
                datadogV1::api_logs_indexes::LogsIndexesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "LogsPipelines" => {
            world.api_instances.v1_api_logs_pipelines = Some(
                datadogV1::api_logs_pipelines::LogsPipelinesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Monitors" => {
            world.api_instances.v1_api_monitors = Some(
                datadogV1::api_monitors::MonitorsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_monitors = Some(
                datadogV2::api_monitors::MonitorsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Notebooks" => {
            world.api_instances.v1_api_notebooks = Some(
                datadogV1::api_notebooks::NotebooksAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Organizations" => {
            world.api_instances.v1_api_organizations = Some(
                datadogV1::api_organizations::OrganizationsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_organizations = Some(
                datadogV2::api_organizations::OrganizationsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "SecurityMonitoring" => {
            world.api_instances.v1_api_security_monitoring = Some(
                datadogV1::api_security_monitoring::SecurityMonitoringAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_security_monitoring = Some(
                datadogV2::api_security_monitoring::SecurityMonitoringAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "ServiceLevelObjectives" => {
            world.api_instances.v1_api_service_level_objectives = Some(datadogV1::api_service_level_objectives::ServiceLevelObjectivesAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
            world.api_instances.v2_api_service_level_objectives = Some(datadogV2::api_service_level_objectives::ServiceLevelObjectivesAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "ServiceLevelObjectiveCorrections" => {
            world.api_instances.v1_api_service_level_objective_corrections = Some(datadogV1::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "Synthetics" => {
            world.api_instances.v1_api_synthetics = Some(
                datadogV1::api_synthetics::SyntheticsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
            world.api_instances.v2_api_synthetics = Some(
                datadogV2::api_synthetics::SyntheticsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Tags" => {
            world.api_instances.v1_api_tags =
                Some(datadogV1::api_tags::TagsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Users" => {
            world.api_instances.v1_api_users =
                Some(datadogV1::api_users::UsersAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
            world.api_instances.v2_api_users =
                Some(datadogV2::api_users::UsersAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Authentication" => {
            world.api_instances.v1_api_authentication = Some(
                datadogV1::api_authentication::AuthenticationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "APIManagement" => {
            world.api_instances.v2_api_api_management = Some(
                datadogV2::api_api_management::APIManagementAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "SpansMetrics" => {
            world.api_instances.v2_api_spans_metrics = Some(
                datadogV2::api_spans_metrics::SpansMetricsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "APMRetentionFilters" => {
            world.api_instances.v2_api_apm_retention_filters = Some(datadogV2::api_apm_retention_filters::APMRetentionFiltersAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "Audit" => {
            world.api_instances.v2_api_audit =
                Some(datadogV2::api_audit::AuditAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "AuthNMappings" => {
            world.api_instances.v2_api_authn_mappings = Some(
                datadogV2::api_authn_mappings::AuthNMappingsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "CaseManagement" => {
            world.api_instances.v2_api_case_management = Some(
                datadogV2::api_case_management::CaseManagementAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "CIVisibilityPipelines" => {
            world.api_instances.v2_api_ci_visibility_pipelines = Some(datadogV2::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "CIVisibilityTests" => {
            world.api_instances.v2_api_ci_visibility_tests = Some(
                datadogV2::api_ci_visibility_tests::CIVisibilityTestsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "ContainerImages" => {
            world.api_instances.v2_api_container_images = Some(
                datadogV2::api_container_images::ContainerImagesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Containers" => {
            world.api_instances.v2_api_containers = Some(
                datadogV2::api_containers::ContainersAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "CloudCostManagement" => {
            world.api_instances.v2_api_cloud_cost_management = Some(datadogV2::api_cloud_cost_management::CloudCostManagementAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "DORAMetrics" => {
            world.api_instances.v2_api_dora_metrics = Some(
                datadogV2::api_dora_metrics::DORAMetricsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Incidents" => {
            world.api_instances.v2_api_incidents = Some(
                datadogV2::api_incidents::IncidentsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "OpsgenieIntegration" => {
            world.api_instances.v2_api_opsgenie_integration = Some(
                datadogV2::api_opsgenie_integration::OpsgenieIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "CloudflareIntegration" => {
            world.api_instances.v2_api_cloudflare_integration = Some(datadogV2::api_cloudflare_integration::CloudflareIntegrationAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "ConfluentCloud" => {
            world.api_instances.v2_api_confluent_cloud = Some(
                datadogV2::api_confluent_cloud::ConfluentCloudAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "FastlyIntegration" => {
            world.api_instances.v2_api_fastly_integration = Some(
                datadogV2::api_fastly_integration::FastlyIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "OktaIntegration" => {
            world.api_instances.v2_api_okta_integration = Some(
                datadogV2::api_okta_integration::OktaIntegrationAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "IPAllowlist" => {
            world.api_instances.v2_api_ip_allowlist = Some(
                datadogV2::api_ip_allowlist::IPAllowlistAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "LogsArchives" => {
            world.api_instances.v2_api_logs_archives = Some(
                datadogV2::api_logs_archives::LogsArchivesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "LogsCustomDestinations" => {
            world.api_instances.v2_api_logs_custom_destinations = Some(datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "LogsMetrics" => {
            world.api_instances.v2_api_logs_metrics = Some(
                datadogV2::api_logs_metrics::LogsMetricsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "NetworkDeviceMonitoring" => {
            world.api_instances.v2_api_network_device_monitoring = Some(datadogV2::api_network_device_monitoring::NetworkDeviceMonitoringAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "Roles" => {
            world.api_instances.v2_api_roles =
                Some(datadogV2::api_roles::RolesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Powerpack" => {
            world.api_instances.v2_api_powerpack = Some(
                datadogV2::api_powerpack::PowerpackAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Processes" => {
            world.api_instances.v2_api_processes = Some(
                datadogV2::api_processes::ProcessesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "CSMThreats" => {
            world.api_instances.v2_api_csm_threats = Some(
                datadogV2::api_csm_threats::CSMThreatsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "RestrictionPolicies" => {
            world.api_instances.v2_api_restriction_policies = Some(
                datadogV2::api_restriction_policies::RestrictionPoliciesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "RUM" => {
            world.api_instances.v2_api_rum =
                Some(datadogV2::api_rum::RUMAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "ServiceScorecards" => {
            world.api_instances.v2_api_service_scorecards = Some(
                datadogV2::api_service_scorecards::ServiceScorecardsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "SensitiveDataScanner" => {
            world.api_instances.v2_api_sensitive_data_scanner = Some(datadogV2::api_sensitive_data_scanner::SensitiveDataScannerAPI::with_client_and_config(
                world.config.clone(),
                world.http_client.as_ref().unwrap().clone()
            ));
        }
        "ServiceAccounts" => {
            world.api_instances.v2_api_service_accounts = Some(
                datadogV2::api_service_accounts::ServiceAccountsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "IncidentServices" => {
            world.api_instances.v2_api_incident_services = Some(
                datadogV2::api_incident_services::IncidentServicesAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "ServiceDefinition" => {
            world.api_instances.v2_api_service_definition = Some(
                datadogV2::api_service_definition::ServiceDefinitionAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        "Spans" => {
            world.api_instances.v2_api_spans =
                Some(datadogV2::api_spans::SpansAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "Teams" => {
            world.api_instances.v2_api_teams =
                Some(datadogV2::api_teams::TeamsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ));
        }
        "IncidentTeams" => {
            world.api_instances.v2_api_incident_teams = Some(
                datadogV2::api_incident_teams::IncidentTeamsAPI::with_client_and_config(
                    world.config.clone(),
                    world.http_client.as_ref().unwrap().clone(),
                ),
            );
        }
        _ => panic!("{api} API instance not found"),
    }
}

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("v1.GetIPRanges".into(), test_v1_get_ip_ranges);
    world
        .function_mappings
        .insert("v1.ListAPIKeys".into(), test_v1_list_api_keys);
    world
        .function_mappings
        .insert("v1.CreateAPIKey".into(), test_v1_create_api_key);
    world
        .function_mappings
        .insert("v1.DeleteAPIKey".into(), test_v1_delete_api_key);
    world
        .function_mappings
        .insert("v1.GetAPIKey".into(), test_v1_get_api_key);
    world
        .function_mappings
        .insert("v1.UpdateAPIKey".into(), test_v1_update_api_key);
    world.function_mappings.insert(
        "v1.ListApplicationKeys".into(),
        test_v1_list_application_keys,
    );
    world.function_mappings.insert(
        "v1.CreateApplicationKey".into(),
        test_v1_create_application_key,
    );
    world.function_mappings.insert(
        "v1.DeleteApplicationKey".into(),
        test_v1_delete_application_key,
    );
    world
        .function_mappings
        .insert("v1.GetApplicationKey".into(), test_v1_get_application_key);
    world.function_mappings.insert(
        "v1.UpdateApplicationKey".into(),
        test_v1_update_application_key,
    );
    world
        .function_mappings
        .insert("v1.SubmitServiceCheck".into(), test_v1_submit_service_check);
    world.function_mappings.insert(
        "v1.GetDailyCustomReports".into(),
        test_v1_get_daily_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetSpecifiedDailyCustomReports".into(),
        test_v1_get_specified_daily_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetMonthlyCustomReports".into(),
        test_v1_get_monthly_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetSpecifiedMonthlyCustomReports".into(),
        test_v1_get_specified_monthly_custom_reports,
    );
    world.function_mappings.insert(
        "v1.GetUsageAnalyzedLogs".into(),
        test_v1_get_usage_analyzed_logs,
    );
    world
        .function_mappings
        .insert("v1.GetUsageAuditLogs".into(), test_v1_get_usage_audit_logs);
    world
        .function_mappings
        .insert("v1.GetUsageLambda".into(), test_v1_get_usage_lambda);
    world.function_mappings.insert(
        "v1.GetUsageBillableSummary".into(),
        test_v1_get_usage_billable_summary,
    );
    world
        .function_mappings
        .insert("v1.GetUsageCIApp".into(), test_v1_get_usage_ci_app);
    world.function_mappings.insert(
        "v1.GetUsageCloudSecurityPostureManagement".into(),
        test_v1_get_usage_cloud_security_posture_management,
    );
    world
        .function_mappings
        .insert("v1.GetUsageCWS".into(), test_v1_get_usage_cws);
    world
        .function_mappings
        .insert("v1.GetUsageDBM".into(), test_v1_get_usage_dbm);
    world
        .function_mappings
        .insert("v1.GetUsageFargate".into(), test_v1_get_usage_fargate);
    world
        .function_mappings
        .insert("v1.GetUsageHosts".into(), test_v1_get_usage_hosts);
    world.function_mappings.insert(
        "v1.GetHourlyUsageAttribution".into(),
        test_v1_get_hourly_usage_attribution,
    );
    world.function_mappings.insert(
        "v1.GetIncidentManagement".into(),
        test_v1_get_incident_management,
    );
    world.function_mappings.insert(
        "v1.GetUsageIndexedSpans".into(),
        test_v1_get_usage_indexed_spans,
    );
    world
        .function_mappings
        .insert("v1.GetIngestedSpans".into(), test_v1_get_ingested_spans);
    world.function_mappings.insert(
        "v1.GetUsageInternetOfThings".into(),
        test_v1_get_usage_internet_of_things,
    );
    world
        .function_mappings
        .insert("v1.GetUsageLogs".into(), test_v1_get_usage_logs);
    world.function_mappings.insert(
        "v1.GetUsageLogsByRetention".into(),
        test_v1_get_usage_logs_by_retention,
    );
    world.function_mappings.insert(
        "v1.GetUsageLogsByIndex".into(),
        test_v1_get_usage_logs_by_index,
    );
    world.function_mappings.insert(
        "v1.GetMonthlyUsageAttribution".into(),
        test_v1_get_monthly_usage_attribution,
    );
    world.function_mappings.insert(
        "v1.GetUsageNetworkFlows".into(),
        test_v1_get_usage_network_flows,
    );
    world.function_mappings.insert(
        "v1.GetUsageNetworkHosts".into(),
        test_v1_get_usage_network_hosts,
    );
    world.function_mappings.insert(
        "v1.GetUsageOnlineArchive".into(),
        test_v1_get_usage_online_archive,
    );
    world
        .function_mappings
        .insert("v1.GetUsageProfiling".into(), test_v1_get_usage_profiling);
    world
        .function_mappings
        .insert("v1.GetUsageRumUnits".into(), test_v1_get_usage_rum_units);
    world.function_mappings.insert(
        "v1.GetUsageRumSessions".into(),
        test_v1_get_usage_rum_sessions,
    );
    world
        .function_mappings
        .insert("v1.GetUsageSDS".into(), test_v1_get_usage_sds);
    world
        .function_mappings
        .insert("v1.GetUsageSNMP".into(), test_v1_get_usage_snmp);
    world
        .function_mappings
        .insert("v1.GetUsageSummary".into(), test_v1_get_usage_summary);
    world
        .function_mappings
        .insert("v1.GetUsageSynthetics".into(), test_v1_get_usage_synthetics);
    world.function_mappings.insert(
        "v1.GetUsageSyntheticsAPI".into(),
        test_v1_get_usage_synthetics_api,
    );
    world.function_mappings.insert(
        "v1.GetUsageSyntheticsBrowser".into(),
        test_v1_get_usage_synthetics_browser,
    );
    world
        .function_mappings
        .insert("v1.GetUsageTimeseries".into(), test_v1_get_usage_timeseries);
    world.function_mappings.insert(
        "v1.GetUsageTopAvgMetrics".into(),
        test_v1_get_usage_top_avg_metrics,
    );
    world
        .function_mappings
        .insert("v1.DeleteDashboards".into(), test_v1_delete_dashboards);
    world
        .function_mappings
        .insert("v1.ListDashboards".into(), test_v1_list_dashboards);
    world.function_mappings.insert(
        "v1.ListDashboardsWithPagination".into(),
        test_v1_list_dashboards_with_pagination,
    );
    world
        .function_mappings
        .insert("v1.RestoreDashboards".into(), test_v1_restore_dashboards);
    world
        .function_mappings
        .insert("v1.CreateDashboard".into(), test_v1_create_dashboard);
    world.function_mappings.insert(
        "v1.CreatePublicDashboard".into(),
        test_v1_create_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.DeletePublicDashboard".into(),
        test_v1_delete_public_dashboard,
    );
    world
        .function_mappings
        .insert("v1.GetPublicDashboard".into(), test_v1_get_public_dashboard);
    world.function_mappings.insert(
        "v1.UpdatePublicDashboard".into(),
        test_v1_update_public_dashboard,
    );
    world.function_mappings.insert(
        "v1.DeletePublicDashboardInvitation".into(),
        test_v1_delete_public_dashboard_invitation,
    );
    world.function_mappings.insert(
        "v1.GetPublicDashboardInvitations".into(),
        test_v1_get_public_dashboard_invitations,
    );
    world.function_mappings.insert(
        "v1.SendPublicDashboardInvitation".into(),
        test_v1_send_public_dashboard_invitation,
    );
    world
        .function_mappings
        .insert("v1.DeleteDashboard".into(), test_v1_delete_dashboard);
    world
        .function_mappings
        .insert("v1.GetDashboard".into(), test_v1_get_dashboard);
    world
        .function_mappings
        .insert("v1.UpdateDashboard".into(), test_v1_update_dashboard);
    world
        .function_mappings
        .insert("v1.ListDashboardLists".into(), test_v1_list_dashboard_lists);
    world.function_mappings.insert(
        "v1.CreateDashboardList".into(),
        test_v1_create_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.DeleteDashboardList".into(),
        test_v1_delete_dashboard_list,
    );
    world
        .function_mappings
        .insert("v1.GetDashboardList".into(), test_v1_get_dashboard_list);
    world.function_mappings.insert(
        "v1.UpdateDashboardList".into(),
        test_v1_update_dashboard_list,
    );
    world.function_mappings.insert(
        "v1.SubmitDistributionPoints".into(),
        test_v1_submit_distribution_points,
    );
    world
        .function_mappings
        .insert("v1.ListActiveMetrics".into(), test_v1_list_active_metrics);
    world
        .function_mappings
        .insert("v1.GetMetricMetadata".into(), test_v1_get_metric_metadata);
    world.function_mappings.insert(
        "v1.UpdateMetricMetadata".into(),
        test_v1_update_metric_metadata,
    );
    world
        .function_mappings
        .insert("v1.QueryMetrics".into(), test_v1_query_metrics);
    world
        .function_mappings
        .insert("v1.ListMetrics".into(), test_v1_list_metrics);
    world
        .function_mappings
        .insert("v1.SubmitMetrics".into(), test_v1_submit_metrics);
    world
        .function_mappings
        .insert("v1.ListDowntimes".into(), test_v1_list_downtimes);
    world
        .function_mappings
        .insert("v1.CreateDowntime".into(), test_v1_create_downtime);
    world.function_mappings.insert(
        "v1.CancelDowntimesByScope".into(),
        test_v1_cancel_downtimes_by_scope,
    );
    world
        .function_mappings
        .insert("v1.CancelDowntime".into(), test_v1_cancel_downtime);
    world
        .function_mappings
        .insert("v1.GetDowntime".into(), test_v1_get_downtime);
    world
        .function_mappings
        .insert("v1.UpdateDowntime".into(), test_v1_update_downtime);
    world.function_mappings.insert(
        "v1.ListMonitorDowntimes".into(),
        test_v1_list_monitor_downtimes,
    );
    world
        .function_mappings
        .insert("v1.ListEvents".into(), test_v1_list_events);
    world
        .function_mappings
        .insert("v1.CreateEvent".into(), test_v1_create_event);
    world
        .function_mappings
        .insert("v1.GetEvent".into(), test_v1_get_event);
    world
        .function_mappings
        .insert("v1.GetGraphSnapshot".into(), test_v1_get_graph_snapshot);
    world
        .function_mappings
        .insert("v1.MuteHost".into(), test_v1_mute_host);
    world
        .function_mappings
        .insert("v1.UnmuteHost".into(), test_v1_unmute_host);
    world
        .function_mappings
        .insert("v1.ListHosts".into(), test_v1_list_hosts);
    world
        .function_mappings
        .insert("v1.GetHostTotals".into(), test_v1_get_host_totals);
    world
        .function_mappings
        .insert("v1.DeleteAWSAccount".into(), test_v1_delete_aws_account);
    world
        .function_mappings
        .insert("v1.ListAWSAccounts".into(), test_v1_list_aws_accounts);
    world
        .function_mappings
        .insert("v1.CreateAWSAccount".into(), test_v1_create_aws_account);
    world
        .function_mappings
        .insert("v1.UpdateAWSAccount".into(), test_v1_update_aws_account);
    world.function_mappings.insert(
        "v1.ListAvailableAWSNamespaces".into(),
        test_v1_list_available_aws_namespaces,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSEventBridgeSource".into(),
        test_v1_delete_aws_event_bridge_source,
    );
    world.function_mappings.insert(
        "v1.ListAWSEventBridgeSources".into(),
        test_v1_list_aws_event_bridge_sources,
    );
    world.function_mappings.insert(
        "v1.CreateAWSEventBridgeSource".into(),
        test_v1_create_aws_event_bridge_source,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSTagFilter".into(),
        test_v1_delete_aws_tag_filter,
    );
    world
        .function_mappings
        .insert("v1.ListAWSTagFilters".into(), test_v1_list_aws_tag_filters);
    world.function_mappings.insert(
        "v1.CreateAWSTagFilter".into(),
        test_v1_create_aws_tag_filter,
    );
    world.function_mappings.insert(
        "v1.CreateNewAWSExternalID".into(),
        test_v1_create_new_aws_external_id,
    );
    world.function_mappings.insert(
        "v1.DeleteAWSLambdaARN".into(),
        test_v1_delete_aws_lambda_arn,
    );
    world.function_mappings.insert(
        "v1.ListAWSLogsIntegrations".into(),
        test_v1_list_aws_logs_integrations,
    );
    world.function_mappings.insert(
        "v1.CreateAWSLambdaARN".into(),
        test_v1_create_aws_lambda_arn,
    );
    world.function_mappings.insert(
        "v1.CheckAWSLogsLambdaAsync".into(),
        test_v1_check_aws_logs_lambda_async,
    );
    world.function_mappings.insert(
        "v1.ListAWSLogsServices".into(),
        test_v1_list_aws_logs_services,
    );
    world.function_mappings.insert(
        "v1.EnableAWSLogServices".into(),
        test_v1_enable_aws_log_services,
    );
    world.function_mappings.insert(
        "v1.CheckAWSLogsServicesAsync".into(),
        test_v1_check_aws_logs_services_async,
    );
    world.function_mappings.insert(
        "v1.DeleteAzureIntegration".into(),
        test_v1_delete_azure_integration,
    );
    world.function_mappings.insert(
        "v1.ListAzureIntegration".into(),
        test_v1_list_azure_integration,
    );
    world.function_mappings.insert(
        "v1.CreateAzureIntegration".into(),
        test_v1_create_azure_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateAzureIntegration".into(),
        test_v1_update_azure_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateAzureHostFilters".into(),
        test_v1_update_azure_host_filters,
    );
    world.function_mappings.insert(
        "v1.DeleteGCPIntegration".into(),
        test_v1_delete_gcp_integration,
    );
    world
        .function_mappings
        .insert("v1.ListGCPIntegration".into(), test_v1_list_gcp_integration);
    world.function_mappings.insert(
        "v1.CreateGCPIntegration".into(),
        test_v1_create_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateGCPIntegration".into(),
        test_v1_update_gcp_integration,
    );
    world.function_mappings.insert(
        "v1.CreatePagerDutyIntegrationService".into(),
        test_v1_create_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.DeletePagerDutyIntegrationService".into(),
        test_v1_delete_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.GetPagerDutyIntegrationService".into(),
        test_v1_get_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.UpdatePagerDutyIntegrationService".into(),
        test_v1_update_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "v1.GetSlackIntegrationChannels".into(),
        test_v1_get_slack_integration_channels,
    );
    world.function_mappings.insert(
        "v1.CreateSlackIntegrationChannel".into(),
        test_v1_create_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.RemoveSlackIntegrationChannel".into(),
        test_v1_remove_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.GetSlackIntegrationChannel".into(),
        test_v1_get_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.UpdateSlackIntegrationChannel".into(),
        test_v1_update_slack_integration_channel,
    );
    world.function_mappings.insert(
        "v1.CreateWebhooksIntegrationCustomVariable".into(),
        test_v1_create_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.DeleteWebhooksIntegrationCustomVariable".into(),
        test_v1_delete_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.GetWebhooksIntegrationCustomVariable".into(),
        test_v1_get_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.UpdateWebhooksIntegrationCustomVariable".into(),
        test_v1_update_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "v1.CreateWebhooksIntegration".into(),
        test_v1_create_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.DeleteWebhooksIntegration".into(),
        test_v1_delete_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.GetWebhooksIntegration".into(),
        test_v1_get_webhooks_integration,
    );
    world.function_mappings.insert(
        "v1.UpdateWebhooksIntegration".into(),
        test_v1_update_webhooks_integration,
    );
    world
        .function_mappings
        .insert("v1.ListLogs".into(), test_v1_list_logs);
    world
        .function_mappings
        .insert("v1.SubmitLog".into(), test_v1_submit_log);
    world
        .function_mappings
        .insert("v1.GetLogsIndexOrder".into(), test_v1_get_logs_index_order);
    world.function_mappings.insert(
        "v1.UpdateLogsIndexOrder".into(),
        test_v1_update_logs_index_order,
    );
    world
        .function_mappings
        .insert("v1.ListLogIndexes".into(), test_v1_list_log_indexes);
    world
        .function_mappings
        .insert("v1.CreateLogsIndex".into(), test_v1_create_logs_index);
    world
        .function_mappings
        .insert("v1.GetLogsIndex".into(), test_v1_get_logs_index);
    world
        .function_mappings
        .insert("v1.UpdateLogsIndex".into(), test_v1_update_logs_index);
    world.function_mappings.insert(
        "v1.GetLogsPipelineOrder".into(),
        test_v1_get_logs_pipeline_order,
    );
    world.function_mappings.insert(
        "v1.UpdateLogsPipelineOrder".into(),
        test_v1_update_logs_pipeline_order,
    );
    world
        .function_mappings
        .insert("v1.ListLogsPipelines".into(), test_v1_list_logs_pipelines);
    world
        .function_mappings
        .insert("v1.CreateLogsPipeline".into(), test_v1_create_logs_pipeline);
    world
        .function_mappings
        .insert("v1.DeleteLogsPipeline".into(), test_v1_delete_logs_pipeline);
    world
        .function_mappings
        .insert("v1.GetLogsPipeline".into(), test_v1_get_logs_pipeline);
    world
        .function_mappings
        .insert("v1.UpdateLogsPipeline".into(), test_v1_update_logs_pipeline);
    world
        .function_mappings
        .insert("v1.ListMonitors".into(), test_v1_list_monitors);
    world.function_mappings.insert(
        "v1.ListMonitorsWithPagination".into(),
        test_v1_list_monitors_with_pagination,
    );
    world
        .function_mappings
        .insert("v1.CreateMonitor".into(), test_v1_create_monitor);
    world.function_mappings.insert(
        "v1.CheckCanDeleteMonitor".into(),
        test_v1_check_can_delete_monitor,
    );
    world.function_mappings.insert(
        "v1.SearchMonitorGroups".into(),
        test_v1_search_monitor_groups,
    );
    world
        .function_mappings
        .insert("v1.SearchMonitors".into(), test_v1_search_monitors);
    world
        .function_mappings
        .insert("v1.ValidateMonitor".into(), test_v1_validate_monitor);
    world
        .function_mappings
        .insert("v1.DeleteMonitor".into(), test_v1_delete_monitor);
    world
        .function_mappings
        .insert("v1.GetMonitor".into(), test_v1_get_monitor);
    world
        .function_mappings
        .insert("v1.UpdateMonitor".into(), test_v1_update_monitor);
    world.function_mappings.insert(
        "v1.ValidateExistingMonitor".into(),
        test_v1_validate_existing_monitor,
    );
    world
        .function_mappings
        .insert("v1.ListNotebooks".into(), test_v1_list_notebooks);
    world.function_mappings.insert(
        "v1.ListNotebooksWithPagination".into(),
        test_v1_list_notebooks_with_pagination,
    );
    world
        .function_mappings
        .insert("v1.CreateNotebook".into(), test_v1_create_notebook);
    world
        .function_mappings
        .insert("v1.DeleteNotebook".into(), test_v1_delete_notebook);
    world
        .function_mappings
        .insert("v1.GetNotebook".into(), test_v1_get_notebook);
    world
        .function_mappings
        .insert("v1.UpdateNotebook".into(), test_v1_update_notebook);
    world
        .function_mappings
        .insert("v1.ListOrgs".into(), test_v1_list_orgs);
    world
        .function_mappings
        .insert("v1.CreateChildOrg".into(), test_v1_create_child_org);
    world
        .function_mappings
        .insert("v1.GetOrg".into(), test_v1_get_org);
    world
        .function_mappings
        .insert("v1.UpdateOrg".into(), test_v1_update_org);
    world
        .function_mappings
        .insert("v1.DowngradeOrg".into(), test_v1_downgrade_org);
    world
        .function_mappings
        .insert("v1.UploadIdPForOrg".into(), test_v1_upload_idp_for_org);
    world.function_mappings.insert(
        "v1.AddSecurityMonitoringSignalToIncident".into(),
        test_v1_add_security_monitoring_signal_to_incident,
    );
    world.function_mappings.insert(
        "v1.EditSecurityMonitoringSignalAssignee".into(),
        test_v1_edit_security_monitoring_signal_assignee,
    );
    world.function_mappings.insert(
        "v1.EditSecurityMonitoringSignalState".into(),
        test_v1_edit_security_monitoring_signal_state,
    );
    world
        .function_mappings
        .insert("v1.ListSLOs".into(), test_v1_list_slos);
    world.function_mappings.insert(
        "v1.ListSLOsWithPagination".into(),
        test_v1_list_slos_with_pagination,
    );
    world
        .function_mappings
        .insert("v1.CreateSLO".into(), test_v1_create_slo);
    world.function_mappings.insert(
        "v1.DeleteSLOTimeframeInBulk".into(),
        test_v1_delete_slo_timeframe_in_bulk,
    );
    world
        .function_mappings
        .insert("v1.CheckCanDeleteSLO".into(), test_v1_check_can_delete_slo);
    world
        .function_mappings
        .insert("v1.SearchSLO".into(), test_v1_search_slo);
    world
        .function_mappings
        .insert("v1.DeleteSLO".into(), test_v1_delete_slo);
    world
        .function_mappings
        .insert("v1.GetSLO".into(), test_v1_get_slo);
    world
        .function_mappings
        .insert("v1.UpdateSLO".into(), test_v1_update_slo);
    world
        .function_mappings
        .insert("v1.GetSLOCorrections".into(), test_v1_get_slo_corrections);
    world
        .function_mappings
        .insert("v1.GetSLOHistory".into(), test_v1_get_slo_history);
    world
        .function_mappings
        .insert("v1.ListSLOCorrection".into(), test_v1_list_slo_correction);
    world.function_mappings.insert(
        "v1.ListSLOCorrectionWithPagination".into(),
        test_v1_list_slo_correction_with_pagination,
    );
    world.function_mappings.insert(
        "v1.CreateSLOCorrection".into(),
        test_v1_create_slo_correction,
    );
    world.function_mappings.insert(
        "v1.DeleteSLOCorrection".into(),
        test_v1_delete_slo_correction,
    );
    world
        .function_mappings
        .insert("v1.GetSLOCorrection".into(), test_v1_get_slo_correction);
    world.function_mappings.insert(
        "v1.UpdateSLOCorrection".into(),
        test_v1_update_slo_correction,
    );
    world.function_mappings.insert(
        "v1.GetSyntheticsCIBatch".into(),
        test_v1_get_synthetics_ci_batch,
    );
    world
        .function_mappings
        .insert("v1.ListLocations".into(), test_v1_list_locations);
    world.function_mappings.insert(
        "v1.CreatePrivateLocation".into(),
        test_v1_create_private_location,
    );
    world.function_mappings.insert(
        "v1.DeletePrivateLocation".into(),
        test_v1_delete_private_location,
    );
    world
        .function_mappings
        .insert("v1.GetPrivateLocation".into(), test_v1_get_private_location);
    world.function_mappings.insert(
        "v1.UpdatePrivateLocation".into(),
        test_v1_update_private_location,
    );
    world.function_mappings.insert(
        "v1.GetSyntheticsDefaultLocations".into(),
        test_v1_get_synthetics_default_locations,
    );
    world
        .function_mappings
        .insert("v1.ListTests".into(), test_v1_list_tests);
    world.function_mappings.insert(
        "v1.ListTestsWithPagination".into(),
        test_v1_list_tests_with_pagination,
    );
    world.function_mappings.insert(
        "v1.CreateSyntheticsAPITest".into(),
        test_v1_create_synthetics_api_test,
    );
    world
        .function_mappings
        .insert("v1.GetAPITest".into(), test_v1_get_api_test);
    world
        .function_mappings
        .insert("v1.UpdateAPITest".into(), test_v1_update_api_test);
    world.function_mappings.insert(
        "v1.CreateSyntheticsBrowserTest".into(),
        test_v1_create_synthetics_browser_test,
    );
    world
        .function_mappings
        .insert("v1.GetBrowserTest".into(), test_v1_get_browser_test);
    world
        .function_mappings
        .insert("v1.UpdateBrowserTest".into(), test_v1_update_browser_test);
    world.function_mappings.insert(
        "v1.GetBrowserTestLatestResults".into(),
        test_v1_get_browser_test_latest_results,
    );
    world.function_mappings.insert(
        "v1.GetBrowserTestResult".into(),
        test_v1_get_browser_test_result,
    );
    world
        .function_mappings
        .insert("v1.DeleteTests".into(), test_v1_delete_tests);
    world
        .function_mappings
        .insert("v1.TriggerTests".into(), test_v1_trigger_tests);
    world
        .function_mappings
        .insert("v1.TriggerCITests".into(), test_v1_trigger_ci_tests);
    world
        .function_mappings
        .insert("v1.GetTest".into(), test_v1_get_test);
    world
        .function_mappings
        .insert("v1.PatchTest".into(), test_v1_patch_test);
    world.function_mappings.insert(
        "v1.GetAPITestLatestResults".into(),
        test_v1_get_api_test_latest_results,
    );
    world
        .function_mappings
        .insert("v1.GetAPITestResult".into(), test_v1_get_api_test_result);
    world.function_mappings.insert(
        "v1.UpdateTestPauseStatus".into(),
        test_v1_update_test_pause_status,
    );
    world.function_mappings.insert(
        "v1.ListGlobalVariables".into(),
        test_v1_list_global_variables,
    );
    world.function_mappings.insert(
        "v1.CreateGlobalVariable".into(),
        test_v1_create_global_variable,
    );
    world.function_mappings.insert(
        "v1.DeleteGlobalVariable".into(),
        test_v1_delete_global_variable,
    );
    world
        .function_mappings
        .insert("v1.GetGlobalVariable".into(), test_v1_get_global_variable);
    world
        .function_mappings
        .insert("v1.EditGlobalVariable".into(), test_v1_edit_global_variable);
    world
        .function_mappings
        .insert("v1.ListHostTags".into(), test_v1_list_host_tags);
    world
        .function_mappings
        .insert("v1.DeleteHostTags".into(), test_v1_delete_host_tags);
    world
        .function_mappings
        .insert("v1.GetHostTags".into(), test_v1_get_host_tags);
    world
        .function_mappings
        .insert("v1.CreateHostTags".into(), test_v1_create_host_tags);
    world
        .function_mappings
        .insert("v1.UpdateHostTags".into(), test_v1_update_host_tags);
    world
        .function_mappings
        .insert("v1.ListUsers".into(), test_v1_list_users);
    world
        .function_mappings
        .insert("v1.CreateUser".into(), test_v1_create_user);
    world
        .function_mappings
        .insert("v1.DisableUser".into(), test_v1_disable_user);
    world
        .function_mappings
        .insert("v1.GetUser".into(), test_v1_get_user);
    world
        .function_mappings
        .insert("v1.UpdateUser".into(), test_v1_update_user);
    world
        .function_mappings
        .insert("v1.Validate".into(), test_v1_validate);
    world
        .function_mappings
        .insert("v2.ListAPIKeys".into(), test_v2_list_api_keys);
    world
        .function_mappings
        .insert("v2.CreateAPIKey".into(), test_v2_create_api_key);
    world
        .function_mappings
        .insert("v2.DeleteAPIKey".into(), test_v2_delete_api_key);
    world
        .function_mappings
        .insert("v2.GetAPIKey".into(), test_v2_get_api_key);
    world
        .function_mappings
        .insert("v2.UpdateAPIKey".into(), test_v2_update_api_key);
    world.function_mappings.insert(
        "v2.ListApplicationKeys".into(),
        test_v2_list_application_keys,
    );
    world.function_mappings.insert(
        "v2.DeleteApplicationKey".into(),
        test_v2_delete_application_key,
    );
    world
        .function_mappings
        .insert("v2.GetApplicationKey".into(), test_v2_get_application_key);
    world.function_mappings.insert(
        "v2.UpdateApplicationKey".into(),
        test_v2_update_application_key,
    );
    world.function_mappings.insert(
        "v2.ListCurrentUserApplicationKeys".into(),
        test_v2_list_current_user_application_keys,
    );
    world.function_mappings.insert(
        "v2.CreateCurrentUserApplicationKey".into(),
        test_v2_create_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.DeleteCurrentUserApplicationKey".into(),
        test_v2_delete_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.GetCurrentUserApplicationKey".into(),
        test_v2_get_current_user_application_key,
    );
    world.function_mappings.insert(
        "v2.UpdateCurrentUserApplicationKey".into(),
        test_v2_update_current_user_application_key,
    );
    world
        .function_mappings
        .insert("v2.ListAPIs".into(), test_v2_list_apis);
    world
        .function_mappings
        .insert("v2.DeleteOpenAPI".into(), test_v2_delete_open_api);
    world
        .function_mappings
        .insert("v2.GetOpenAPI".into(), test_v2_get_open_api);
    world
        .function_mappings
        .insert("v2.UpdateOpenAPI".into(), test_v2_update_open_api);
    world
        .function_mappings
        .insert("v2.CreateOpenAPI".into(), test_v2_create_open_api);
    world
        .function_mappings
        .insert("v2.ListSpansMetrics".into(), test_v2_list_spans_metrics);
    world
        .function_mappings
        .insert("v2.CreateSpansMetric".into(), test_v2_create_spans_metric);
    world
        .function_mappings
        .insert("v2.DeleteSpansMetric".into(), test_v2_delete_spans_metric);
    world
        .function_mappings
        .insert("v2.GetSpansMetric".into(), test_v2_get_spans_metric);
    world
        .function_mappings
        .insert("v2.UpdateSpansMetric".into(), test_v2_update_spans_metric);
    world.function_mappings.insert(
        "v2.ListApmRetentionFilters".into(),
        test_v2_list_apm_retention_filters,
    );
    world.function_mappings.insert(
        "v2.CreateApmRetentionFilter".into(),
        test_v2_create_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.ReorderApmRetentionFilters".into(),
        test_v2_reorder_apm_retention_filters,
    );
    world.function_mappings.insert(
        "v2.DeleteApmRetentionFilter".into(),
        test_v2_delete_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.GetApmRetentionFilter".into(),
        test_v2_get_apm_retention_filter,
    );
    world.function_mappings.insert(
        "v2.UpdateApmRetentionFilter".into(),
        test_v2_update_apm_retention_filter,
    );
    world
        .function_mappings
        .insert("v2.ListAuditLogs".into(), test_v2_list_audit_logs);
    world.function_mappings.insert(
        "v2.ListAuditLogsWithPagination".into(),
        test_v2_list_audit_logs_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.SearchAuditLogs".into(), test_v2_search_audit_logs);
    world.function_mappings.insert(
        "v2.SearchAuditLogsWithPagination".into(),
        test_v2_search_audit_logs_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListAuthNMappings".into(), test_v2_list_authn_mappings);
    world
        .function_mappings
        .insert("v2.CreateAuthNMapping".into(), test_v2_create_authn_mapping);
    world
        .function_mappings
        .insert("v2.DeleteAuthNMapping".into(), test_v2_delete_authn_mapping);
    world
        .function_mappings
        .insert("v2.GetAuthNMapping".into(), test_v2_get_authn_mapping);
    world
        .function_mappings
        .insert("v2.UpdateAuthNMapping".into(), test_v2_update_authn_mapping);
    world
        .function_mappings
        .insert("v2.SearchCases".into(), test_v2_search_cases);
    world.function_mappings.insert(
        "v2.SearchCasesWithPagination".into(),
        test_v2_search_cases_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreateCase".into(), test_v2_create_case);
    world
        .function_mappings
        .insert("v2.GetProjects".into(), test_v2_get_projects);
    world
        .function_mappings
        .insert("v2.CreateProject".into(), test_v2_create_project);
    world
        .function_mappings
        .insert("v2.DeleteProject".into(), test_v2_delete_project);
    world
        .function_mappings
        .insert("v2.GetProject".into(), test_v2_get_project);
    world
        .function_mappings
        .insert("v2.GetCase".into(), test_v2_get_case);
    world
        .function_mappings
        .insert("v2.ArchiveCase".into(), test_v2_archive_case);
    world
        .function_mappings
        .insert("v2.AssignCase".into(), test_v2_assign_case);
    world
        .function_mappings
        .insert("v2.UpdatePriority".into(), test_v2_update_priority);
    world
        .function_mappings
        .insert("v2.UpdateStatus".into(), test_v2_update_status);
    world
        .function_mappings
        .insert("v2.UnarchiveCase".into(), test_v2_unarchive_case);
    world
        .function_mappings
        .insert("v2.UnassignCase".into(), test_v2_unassign_case);
    world.function_mappings.insert(
        "v2.CreateCIAppPipelineEvent".into(),
        test_v2_create_ci_app_pipeline_event,
    );
    world.function_mappings.insert(
        "v2.AggregateCIAppPipelineEvents".into(),
        test_v2_aggregate_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppPipelineEvents".into(),
        test_v2_list_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppPipelineEventsWithPagination".into(),
        test_v2_list_ci_app_pipeline_events_with_pagination,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppPipelineEvents".into(),
        test_v2_search_ci_app_pipeline_events,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppPipelineEventsWithPagination".into(),
        test_v2_search_ci_app_pipeline_events_with_pagination,
    );
    world.function_mappings.insert(
        "v2.AggregateCIAppTestEvents".into(),
        test_v2_aggregate_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppTestEvents".into(),
        test_v2_list_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.ListCIAppTestEventsWithPagination".into(),
        test_v2_list_ci_app_test_events_with_pagination,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppTestEvents".into(),
        test_v2_search_ci_app_test_events,
    );
    world.function_mappings.insert(
        "v2.SearchCIAppTestEventsWithPagination".into(),
        test_v2_search_ci_app_test_events_with_pagination,
    );
    world.function_mappings.insert(
        "v2.ListContainerImages".into(),
        test_v2_list_container_images,
    );
    world.function_mappings.insert(
        "v2.ListContainerImagesWithPagination".into(),
        test_v2_list_container_images_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListContainers".into(), test_v2_list_containers);
    world.function_mappings.insert(
        "v2.ListContainersWithPagination".into(),
        test_v2_list_containers_with_pagination,
    );
    world.function_mappings.insert(
        "v2.ListCostAWSCURConfigs".into(),
        test_v2_list_cost_awscur_configs,
    );
    world.function_mappings.insert(
        "v2.CreateCostAWSCURConfig".into(),
        test_v2_create_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.DeleteCostAWSCURConfig".into(),
        test_v2_delete_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.UpdateCostAWSCURConfig".into(),
        test_v2_update_cost_awscur_config,
    );
    world.function_mappings.insert(
        "v2.ListAWSRelatedAccounts".into(),
        test_v2_list_aws_related_accounts,
    );
    world.function_mappings.insert(
        "v2.ListCostAzureUCConfigs".into(),
        test_v2_list_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.CreateCostAzureUCConfigs".into(),
        test_v2_create_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.DeleteCostAzureUCConfig".into(),
        test_v2_delete_cost_azure_uc_config,
    );
    world.function_mappings.insert(
        "v2.UpdateCostAzureUCConfigs".into(),
        test_v2_update_cost_azure_uc_configs,
    );
    world.function_mappings.insert(
        "v2.GetCloudCostActivity".into(),
        test_v2_get_cloud_cost_activity,
    );
    world.function_mappings.insert(
        "v2.GetActiveBillingDimensions".into(),
        test_v2_get_active_billing_dimensions,
    );
    world.function_mappings.insert(
        "v2.GetMonthlyCostAttribution".into(),
        test_v2_get_monthly_cost_attribution,
    );
    world.function_mappings.insert(
        "v2.GetUsageApplicationSecurityMonitoring".into(),
        test_v2_get_usage_application_security_monitoring,
    );
    world
        .function_mappings
        .insert("v2.GetCostByOrg".into(), test_v2_get_cost_by_org);
    world.function_mappings.insert(
        "v2.GetEstimatedCostByOrg".into(),
        test_v2_get_estimated_cost_by_org,
    );
    world.function_mappings.insert(
        "v2.GetHistoricalCostByOrg".into(),
        test_v2_get_historical_cost_by_org,
    );
    world
        .function_mappings
        .insert("v2.GetHourlyUsage".into(), test_v2_get_hourly_usage);
    world.function_mappings.insert(
        "v2.GetUsageLambdaTracedInvocations".into(),
        test_v2_get_usage_lambda_traced_invocations,
    );
    world.function_mappings.insert(
        "v2.GetUsageObservabilityPipelines".into(),
        test_v2_get_usage_observability_pipelines,
    );
    world
        .function_mappings
        .insert("v2.GetProjectedCost".into(), test_v2_get_projected_cost);
    world.function_mappings.insert(
        "v2.DeleteDashboardListItems".into(),
        test_v2_delete_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.GetDashboardListItems".into(),
        test_v2_get_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.CreateDashboardListItems".into(),
        test_v2_create_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.UpdateDashboardListItems".into(),
        test_v2_update_dashboard_list_items,
    );
    world.function_mappings.insert(
        "v2.CreateDORADeployment".into(),
        test_v2_create_dora_deployment,
    );
    world
        .function_mappings
        .insert("v2.CreateDORAIncident".into(), test_v2_create_dora_incident);
    world
        .function_mappings
        .insert("v2.ListDowntimes".into(), test_v2_list_downtimes);
    world.function_mappings.insert(
        "v2.ListDowntimesWithPagination".into(),
        test_v2_list_downtimes_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreateDowntime".into(), test_v2_create_downtime);
    world
        .function_mappings
        .insert("v2.CancelDowntime".into(), test_v2_cancel_downtime);
    world
        .function_mappings
        .insert("v2.GetDowntime".into(), test_v2_get_downtime);
    world
        .function_mappings
        .insert("v2.UpdateDowntime".into(), test_v2_update_downtime);
    world.function_mappings.insert(
        "v2.ListMonitorDowntimes".into(),
        test_v2_list_monitor_downtimes,
    );
    world.function_mappings.insert(
        "v2.ListMonitorDowntimesWithPagination".into(),
        test_v2_list_monitor_downtimes_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListEvents".into(), test_v2_list_events);
    world.function_mappings.insert(
        "v2.ListEventsWithPagination".into(),
        test_v2_list_events_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.SearchEvents".into(), test_v2_search_events);
    world.function_mappings.insert(
        "v2.SearchEventsWithPagination".into(),
        test_v2_search_events_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListIncidents".into(), test_v2_list_incidents);
    world.function_mappings.insert(
        "v2.ListIncidentsWithPagination".into(),
        test_v2_list_incidents_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreateIncident".into(), test_v2_create_incident);
    world
        .function_mappings
        .insert("v2.SearchIncidents".into(), test_v2_search_incidents);
    world.function_mappings.insert(
        "v2.SearchIncidentsWithPagination".into(),
        test_v2_search_incidents_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.DeleteIncident".into(), test_v2_delete_incident);
    world
        .function_mappings
        .insert("v2.GetIncident".into(), test_v2_get_incident);
    world
        .function_mappings
        .insert("v2.UpdateIncident".into(), test_v2_update_incident);
    world.function_mappings.insert(
        "v2.ListIncidentAttachments".into(),
        test_v2_list_incident_attachments,
    );
    world.function_mappings.insert(
        "v2.UpdateIncidentAttachments".into(),
        test_v2_update_incident_attachments,
    );
    world.function_mappings.insert(
        "v2.ListIncidentIntegrations".into(),
        test_v2_list_incident_integrations,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentIntegration".into(),
        test_v2_create_incident_integration,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentIntegration".into(),
        test_v2_delete_incident_integration,
    );
    world.function_mappings.insert(
        "v2.GetIncidentIntegration".into(),
        test_v2_get_incident_integration,
    );
    world.function_mappings.insert(
        "v2.UpdateIncidentIntegration".into(),
        test_v2_update_incident_integration,
    );
    world
        .function_mappings
        .insert("v2.ListIncidentTodos".into(), test_v2_list_incident_todos);
    world
        .function_mappings
        .insert("v2.CreateIncidentTodo".into(), test_v2_create_incident_todo);
    world
        .function_mappings
        .insert("v2.DeleteIncidentTodo".into(), test_v2_delete_incident_todo);
    world
        .function_mappings
        .insert("v2.GetIncidentTodo".into(), test_v2_get_incident_todo);
    world
        .function_mappings
        .insert("v2.UpdateIncidentTodo".into(), test_v2_update_incident_todo);
    world
        .function_mappings
        .insert("v2.ListGCPSTSAccounts".into(), test_v2_list_gcpsts_accounts);
    world.function_mappings.insert(
        "v2.CreateGCPSTSAccount".into(),
        test_v2_create_gcpsts_account,
    );
    world.function_mappings.insert(
        "v2.DeleteGCPSTSAccount".into(),
        test_v2_delete_gcpsts_account,
    );
    world.function_mappings.insert(
        "v2.UpdateGCPSTSAccount".into(),
        test_v2_update_gcpsts_account,
    );
    world
        .function_mappings
        .insert("v2.GetGCPSTSDelegate".into(), test_v2_get_gcpsts_delegate);
    world
        .function_mappings
        .insert("v2.MakeGCPSTSDelegate".into(), test_v2_make_gcpsts_delegate);
    world.function_mappings.insert(
        "v2.ListOpsgenieServices".into(),
        test_v2_list_opsgenie_services,
    );
    world.function_mappings.insert(
        "v2.CreateOpsgenieService".into(),
        test_v2_create_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.DeleteOpsgenieService".into(),
        test_v2_delete_opsgenie_service,
    );
    world
        .function_mappings
        .insert("v2.GetOpsgenieService".into(), test_v2_get_opsgenie_service);
    world.function_mappings.insert(
        "v2.UpdateOpsgenieService".into(),
        test_v2_update_opsgenie_service,
    );
    world.function_mappings.insert(
        "v2.ListCloudflareAccounts".into(),
        test_v2_list_cloudflare_accounts,
    );
    world.function_mappings.insert(
        "v2.CreateCloudflareAccount".into(),
        test_v2_create_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.DeleteCloudflareAccount".into(),
        test_v2_delete_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.GetCloudflareAccount".into(),
        test_v2_get_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.UpdateCloudflareAccount".into(),
        test_v2_update_cloudflare_account,
    );
    world.function_mappings.insert(
        "v2.ListConfluentAccount".into(),
        test_v2_list_confluent_account,
    );
    world.function_mappings.insert(
        "v2.CreateConfluentAccount".into(),
        test_v2_create_confluent_account,
    );
    world.function_mappings.insert(
        "v2.DeleteConfluentAccount".into(),
        test_v2_delete_confluent_account,
    );
    world.function_mappings.insert(
        "v2.GetConfluentAccount".into(),
        test_v2_get_confluent_account,
    );
    world.function_mappings.insert(
        "v2.UpdateConfluentAccount".into(),
        test_v2_update_confluent_account,
    );
    world.function_mappings.insert(
        "v2.ListConfluentResource".into(),
        test_v2_list_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.CreateConfluentResource".into(),
        test_v2_create_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.DeleteConfluentResource".into(),
        test_v2_delete_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.GetConfluentResource".into(),
        test_v2_get_confluent_resource,
    );
    world.function_mappings.insert(
        "v2.UpdateConfluentResource".into(),
        test_v2_update_confluent_resource,
    );
    world
        .function_mappings
        .insert("v2.ListFastlyAccounts".into(), test_v2_list_fastly_accounts);
    world.function_mappings.insert(
        "v2.CreateFastlyAccount".into(),
        test_v2_create_fastly_account,
    );
    world.function_mappings.insert(
        "v2.DeleteFastlyAccount".into(),
        test_v2_delete_fastly_account,
    );
    world
        .function_mappings
        .insert("v2.GetFastlyAccount".into(), test_v2_get_fastly_account);
    world.function_mappings.insert(
        "v2.UpdateFastlyAccount".into(),
        test_v2_update_fastly_account,
    );
    world
        .function_mappings
        .insert("v2.ListFastlyServices".into(), test_v2_list_fastly_services);
    world.function_mappings.insert(
        "v2.CreateFastlyService".into(),
        test_v2_create_fastly_service,
    );
    world.function_mappings.insert(
        "v2.DeleteFastlyService".into(),
        test_v2_delete_fastly_service,
    );
    world
        .function_mappings
        .insert("v2.GetFastlyService".into(), test_v2_get_fastly_service);
    world.function_mappings.insert(
        "v2.UpdateFastlyService".into(),
        test_v2_update_fastly_service,
    );
    world
        .function_mappings
        .insert("v2.ListOktaAccounts".into(), test_v2_list_okta_accounts);
    world
        .function_mappings
        .insert("v2.CreateOktaAccount".into(), test_v2_create_okta_account);
    world
        .function_mappings
        .insert("v2.DeleteOktaAccount".into(), test_v2_delete_okta_account);
    world
        .function_mappings
        .insert("v2.GetOktaAccount".into(), test_v2_get_okta_account);
    world
        .function_mappings
        .insert("v2.UpdateOktaAccount".into(), test_v2_update_okta_account);
    world
        .function_mappings
        .insert("v2.GetIPAllowlist".into(), test_v2_get_ip_allowlist);
    world
        .function_mappings
        .insert("v2.UpdateIPAllowlist".into(), test_v2_update_ip_allowlist);
    world
        .function_mappings
        .insert("v2.SubmitLog".into(), test_v2_submit_log);
    world
        .function_mappings
        .insert("v2.AggregateLogs".into(), test_v2_aggregate_logs);
    world
        .function_mappings
        .insert("v2.ListLogsGet".into(), test_v2_list_logs_get);
    world.function_mappings.insert(
        "v2.ListLogsGetWithPagination".into(),
        test_v2_list_logs_get_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListLogs".into(), test_v2_list_logs);
    world.function_mappings.insert(
        "v2.ListLogsWithPagination".into(),
        test_v2_list_logs_with_pagination,
    );
    world.function_mappings.insert(
        "v2.GetLogsArchiveOrder".into(),
        test_v2_get_logs_archive_order,
    );
    world.function_mappings.insert(
        "v2.UpdateLogsArchiveOrder".into(),
        test_v2_update_logs_archive_order,
    );
    world
        .function_mappings
        .insert("v2.ListLogsArchives".into(), test_v2_list_logs_archives);
    world
        .function_mappings
        .insert("v2.CreateLogsArchive".into(), test_v2_create_logs_archive);
    world
        .function_mappings
        .insert("v2.DeleteLogsArchive".into(), test_v2_delete_logs_archive);
    world
        .function_mappings
        .insert("v2.GetLogsArchive".into(), test_v2_get_logs_archive);
    world
        .function_mappings
        .insert("v2.UpdateLogsArchive".into(), test_v2_update_logs_archive);
    world.function_mappings.insert(
        "v2.RemoveRoleFromArchive".into(),
        test_v2_remove_role_from_archive,
    );
    world.function_mappings.insert(
        "v2.ListArchiveReadRoles".into(),
        test_v2_list_archive_read_roles,
    );
    world.function_mappings.insert(
        "v2.AddReadRoleToArchive".into(),
        test_v2_add_read_role_to_archive,
    );
    world.function_mappings.insert(
        "v2.ListLogsCustomDestinations".into(),
        test_v2_list_logs_custom_destinations,
    );
    world.function_mappings.insert(
        "v2.CreateLogsCustomDestination".into(),
        test_v2_create_logs_custom_destination,
    );
    world.function_mappings.insert(
        "v2.DeleteLogsCustomDestination".into(),
        test_v2_delete_logs_custom_destination,
    );
    world.function_mappings.insert(
        "v2.GetLogsCustomDestination".into(),
        test_v2_get_logs_custom_destination,
    );
    world.function_mappings.insert(
        "v2.UpdateLogsCustomDestination".into(),
        test_v2_update_logs_custom_destination,
    );
    world
        .function_mappings
        .insert("v2.ListLogsMetrics".into(), test_v2_list_logs_metrics);
    world
        .function_mappings
        .insert("v2.CreateLogsMetric".into(), test_v2_create_logs_metric);
    world
        .function_mappings
        .insert("v2.DeleteLogsMetric".into(), test_v2_delete_logs_metric);
    world
        .function_mappings
        .insert("v2.GetLogsMetric".into(), test_v2_get_logs_metric);
    world
        .function_mappings
        .insert("v2.UpdateLogsMetric".into(), test_v2_update_logs_metric);
    world.function_mappings.insert(
        "v2.ListTagConfigurations".into(),
        test_v2_list_tag_configurations,
    );
    world.function_mappings.insert(
        "v2.DeleteBulkTagsMetricsConfiguration".into(),
        test_v2_delete_bulk_tags_metrics_configuration,
    );
    world.function_mappings.insert(
        "v2.CreateBulkTagsMetricsConfiguration".into(),
        test_v2_create_bulk_tags_metrics_configuration,
    );
    world.function_mappings.insert(
        "v2.ListActiveMetricConfigurations".into(),
        test_v2_list_active_metric_configurations,
    );
    world.function_mappings.insert(
        "v2.ListTagsByMetricName".into(),
        test_v2_list_tags_by_metric_name,
    );
    world
        .function_mappings
        .insert("v2.ListMetricAssets".into(), test_v2_list_metric_assets);
    world.function_mappings.insert(
        "v2.EstimateMetricsOutputSeries".into(),
        test_v2_estimate_metrics_output_series,
    );
    world.function_mappings.insert(
        "v2.DeleteTagConfiguration".into(),
        test_v2_delete_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.ListTagConfigurationByName".into(),
        test_v2_list_tag_configuration_by_name,
    );
    world.function_mappings.insert(
        "v2.UpdateTagConfiguration".into(),
        test_v2_update_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.CreateTagConfiguration".into(),
        test_v2_create_tag_configuration,
    );
    world.function_mappings.insert(
        "v2.ListVolumesByMetricName".into(),
        test_v2_list_volumes_by_metric_name,
    );
    world
        .function_mappings
        .insert("v2.QueryScalarData".into(), test_v2_query_scalar_data);
    world.function_mappings.insert(
        "v2.QueryTimeseriesData".into(),
        test_v2_query_timeseries_data,
    );
    world
        .function_mappings
        .insert("v2.SubmitMetrics".into(), test_v2_submit_metrics);
    world.function_mappings.insert(
        "v2.ListMonitorConfigPolicies".into(),
        test_v2_list_monitor_config_policies,
    );
    world.function_mappings.insert(
        "v2.CreateMonitorConfigPolicy".into(),
        test_v2_create_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.DeleteMonitorConfigPolicy".into(),
        test_v2_delete_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.GetMonitorConfigPolicy".into(),
        test_v2_get_monitor_config_policy,
    );
    world.function_mappings.insert(
        "v2.UpdateMonitorConfigPolicy".into(),
        test_v2_update_monitor_config_policy,
    );
    world
        .function_mappings
        .insert("v2.ListDevices".into(), test_v2_list_devices);
    world
        .function_mappings
        .insert("v2.GetDevice".into(), test_v2_get_device);
    world
        .function_mappings
        .insert("v2.GetInterfaces".into(), test_v2_get_interfaces);
    world
        .function_mappings
        .insert("v2.ListOrgConfigs".into(), test_v2_list_org_configs);
    world
        .function_mappings
        .insert("v2.GetOrgConfig".into(), test_v2_get_org_config);
    world
        .function_mappings
        .insert("v2.UpdateOrgConfig".into(), test_v2_update_org_config);
    world
        .function_mappings
        .insert("v2.UploadIdPMetadata".into(), test_v2_upload_idp_metadata);
    world
        .function_mappings
        .insert("v2.ListPermissions".into(), test_v2_list_permissions);
    world
        .function_mappings
        .insert("v2.ListRoles".into(), test_v2_list_roles);
    world
        .function_mappings
        .insert("v2.CreateRole".into(), test_v2_create_role);
    world
        .function_mappings
        .insert("v2.DeleteRole".into(), test_v2_delete_role);
    world
        .function_mappings
        .insert("v2.GetRole".into(), test_v2_get_role);
    world
        .function_mappings
        .insert("v2.UpdateRole".into(), test_v2_update_role);
    world
        .function_mappings
        .insert("v2.CloneRole".into(), test_v2_clone_role);
    world.function_mappings.insert(
        "v2.RemovePermissionFromRole".into(),
        test_v2_remove_permission_from_role,
    );
    world.function_mappings.insert(
        "v2.ListRolePermissions".into(),
        test_v2_list_role_permissions,
    );
    world.function_mappings.insert(
        "v2.AddPermissionToRole".into(),
        test_v2_add_permission_to_role,
    );
    world.function_mappings.insert(
        "v2.RemoveUserFromRole".into(),
        test_v2_remove_user_from_role,
    );
    world
        .function_mappings
        .insert("v2.ListRoleUsers".into(), test_v2_list_role_users);
    world
        .function_mappings
        .insert("v2.AddUserToRole".into(), test_v2_add_user_to_role);
    world
        .function_mappings
        .insert("v2.ListFindings".into(), test_v2_list_findings);
    world.function_mappings.insert(
        "v2.ListFindingsWithPagination".into(),
        test_v2_list_findings_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.MuteFindings".into(), test_v2_mute_findings);
    world
        .function_mappings
        .insert("v2.GetFinding".into(), test_v2_get_finding);
    world.function_mappings.insert(
        "v2.ListSecurityFilters".into(),
        test_v2_list_security_filters,
    );
    world.function_mappings.insert(
        "v2.CreateSecurityFilter".into(),
        test_v2_create_security_filter,
    );
    world.function_mappings.insert(
        "v2.DeleteSecurityFilter".into(),
        test_v2_delete_security_filter,
    );
    world
        .function_mappings
        .insert("v2.GetSecurityFilter".into(), test_v2_get_security_filter);
    world.function_mappings.insert(
        "v2.UpdateSecurityFilter".into(),
        test_v2_update_security_filter,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringSuppressions".into(),
        test_v2_list_security_monitoring_suppressions,
    );
    world.function_mappings.insert(
        "v2.CreateSecurityMonitoringSuppression".into(),
        test_v2_create_security_monitoring_suppression,
    );
    world.function_mappings.insert(
        "v2.DeleteSecurityMonitoringSuppression".into(),
        test_v2_delete_security_monitoring_suppression,
    );
    world.function_mappings.insert(
        "v2.GetSecurityMonitoringSuppression".into(),
        test_v2_get_security_monitoring_suppression,
    );
    world.function_mappings.insert(
        "v2.UpdateSecurityMonitoringSuppression".into(),
        test_v2_update_security_monitoring_suppression,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringRules".into(),
        test_v2_list_security_monitoring_rules,
    );
    world.function_mappings.insert(
        "v2.CreateSecurityMonitoringRule".into(),
        test_v2_create_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.ConvertSecurityMonitoringRuleFromJSONToTerraform".into(),
        test_v2_convert_security_monitoring_rule_from_json_to_terraform,
    );
    world.function_mappings.insert(
        "v2.TestSecurityMonitoringRule".into(),
        test_v2_test_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.ValidateSecurityMonitoringRule".into(),
        test_v2_validate_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteSecurityMonitoringRule".into(),
        test_v2_delete_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.GetSecurityMonitoringRule".into(),
        test_v2_get_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateSecurityMonitoringRule".into(),
        test_v2_update_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.ConvertExistingSecurityMonitoringRule".into(),
        test_v2_convert_existing_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.TestExistingSecurityMonitoringRule".into(),
        test_v2_test_existing_security_monitoring_rule,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringSignals".into(),
        test_v2_list_security_monitoring_signals,
    );
    world.function_mappings.insert(
        "v2.ListSecurityMonitoringSignalsWithPagination".into(),
        test_v2_list_security_monitoring_signals_with_pagination,
    );
    world.function_mappings.insert(
        "v2.SearchSecurityMonitoringSignals".into(),
        test_v2_search_security_monitoring_signals,
    );
    world.function_mappings.insert(
        "v2.SearchSecurityMonitoringSignalsWithPagination".into(),
        test_v2_search_security_monitoring_signals_with_pagination,
    );
    world.function_mappings.insert(
        "v2.GetSecurityMonitoringSignal".into(),
        test_v2_get_security_monitoring_signal,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalAssignee".into(),
        test_v2_edit_security_monitoring_signal_assignee,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalIncidents".into(),
        test_v2_edit_security_monitoring_signal_incidents,
    );
    world.function_mappings.insert(
        "v2.EditSecurityMonitoringSignalState".into(),
        test_v2_edit_security_monitoring_signal_state,
    );
    world
        .function_mappings
        .insert("v2.ListPowerpacks".into(), test_v2_list_powerpacks);
    world.function_mappings.insert(
        "v2.ListPowerpacksWithPagination".into(),
        test_v2_list_powerpacks_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreatePowerpack".into(), test_v2_create_powerpack);
    world
        .function_mappings
        .insert("v2.DeletePowerpack".into(), test_v2_delete_powerpack);
    world
        .function_mappings
        .insert("v2.GetPowerpack".into(), test_v2_get_powerpack);
    world
        .function_mappings
        .insert("v2.UpdatePowerpack".into(), test_v2_update_powerpack);
    world
        .function_mappings
        .insert("v2.ListProcesses".into(), test_v2_list_processes);
    world.function_mappings.insert(
        "v2.ListProcessesWithPagination".into(),
        test_v2_list_processes_with_pagination,
    );
    world.function_mappings.insert(
        "v2.ListCSMThreatsAgentRules".into(),
        test_v2_list_csm_threats_agent_rules,
    );
    world.function_mappings.insert(
        "v2.CreateCSMThreatsAgentRule".into(),
        test_v2_create_csm_threats_agent_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteCSMThreatsAgentRule".into(),
        test_v2_delete_csm_threats_agent_rule,
    );
    world.function_mappings.insert(
        "v2.GetCSMThreatsAgentRule".into(),
        test_v2_get_csm_threats_agent_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateCSMThreatsAgentRule".into(),
        test_v2_update_csm_threats_agent_rule,
    );
    world.function_mappings.insert(
        "v2.DownloadCSMThreatsPolicy".into(),
        test_v2_download_csm_threats_policy,
    );
    world.function_mappings.insert(
        "v2.DownloadCloudWorkloadPolicyFile".into(),
        test_v2_download_cloud_workload_policy_file,
    );
    world.function_mappings.insert(
        "v2.ListCloudWorkloadSecurityAgentRules".into(),
        test_v2_list_cloud_workload_security_agent_rules,
    );
    world.function_mappings.insert(
        "v2.CreateCloudWorkloadSecurityAgentRule".into(),
        test_v2_create_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteCloudWorkloadSecurityAgentRule".into(),
        test_v2_delete_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.GetCloudWorkloadSecurityAgentRule".into(),
        test_v2_get_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.UpdateCloudWorkloadSecurityAgentRule".into(),
        test_v2_update_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteRestrictionPolicy".into(),
        test_v2_delete_restriction_policy,
    );
    world.function_mappings.insert(
        "v2.GetRestrictionPolicy".into(),
        test_v2_get_restriction_policy,
    );
    world.function_mappings.insert(
        "v2.UpdateRestrictionPolicy".into(),
        test_v2_update_restriction_policy,
    );
    world
        .function_mappings
        .insert("v2.AggregateRUMEvents".into(), test_v2_aggregate_rum_events);
    world
        .function_mappings
        .insert("v2.GetRUMApplications".into(), test_v2_get_rum_applications);
    world.function_mappings.insert(
        "v2.CreateRUMApplication".into(),
        test_v2_create_rum_application,
    );
    world.function_mappings.insert(
        "v2.DeleteRUMApplication".into(),
        test_v2_delete_rum_application,
    );
    world
        .function_mappings
        .insert("v2.GetRUMApplication".into(), test_v2_get_rum_application);
    world.function_mappings.insert(
        "v2.UpdateRUMApplication".into(),
        test_v2_update_rum_application,
    );
    world
        .function_mappings
        .insert("v2.ListRUMEvents".into(), test_v2_list_rum_events);
    world.function_mappings.insert(
        "v2.ListRUMEventsWithPagination".into(),
        test_v2_list_rum_events_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.SearchRUMEvents".into(), test_v2_search_rum_events);
    world.function_mappings.insert(
        "v2.SearchRUMEventsWithPagination".into(),
        test_v2_search_rum_events_with_pagination,
    );
    world.function_mappings.insert(
        "v2.ListScorecardOutcomes".into(),
        test_v2_list_scorecard_outcomes,
    );
    world.function_mappings.insert(
        "v2.ListScorecardOutcomesWithPagination".into(),
        test_v2_list_scorecard_outcomes_with_pagination,
    );
    world.function_mappings.insert(
        "v2.CreateScorecardOutcomesBatch".into(),
        test_v2_create_scorecard_outcomes_batch,
    );
    world
        .function_mappings
        .insert("v2.ListScorecardRules".into(), test_v2_list_scorecard_rules);
    world.function_mappings.insert(
        "v2.ListScorecardRulesWithPagination".into(),
        test_v2_list_scorecard_rules_with_pagination,
    );
    world.function_mappings.insert(
        "v2.CreateScorecardRule".into(),
        test_v2_create_scorecard_rule,
    );
    world.function_mappings.insert(
        "v2.DeleteScorecardRule".into(),
        test_v2_delete_scorecard_rule,
    );
    world
        .function_mappings
        .insert("v2.ListScanningGroups".into(), test_v2_list_scanning_groups);
    world.function_mappings.insert(
        "v2.ReorderScanningGroups".into(),
        test_v2_reorder_scanning_groups,
    );
    world.function_mappings.insert(
        "v2.CreateScanningGroup".into(),
        test_v2_create_scanning_group,
    );
    world.function_mappings.insert(
        "v2.DeleteScanningGroup".into(),
        test_v2_delete_scanning_group,
    );
    world.function_mappings.insert(
        "v2.UpdateScanningGroup".into(),
        test_v2_update_scanning_group,
    );
    world
        .function_mappings
        .insert("v2.CreateScanningRule".into(), test_v2_create_scanning_rule);
    world
        .function_mappings
        .insert("v2.DeleteScanningRule".into(), test_v2_delete_scanning_rule);
    world
        .function_mappings
        .insert("v2.UpdateScanningRule".into(), test_v2_update_scanning_rule);
    world.function_mappings.insert(
        "v2.ListStandardPatterns".into(),
        test_v2_list_standard_patterns,
    );
    world.function_mappings.insert(
        "v2.CreateServiceAccount".into(),
        test_v2_create_service_account,
    );
    world.function_mappings.insert(
        "v2.ListServiceAccountApplicationKeys".into(),
        test_v2_list_service_account_application_keys,
    );
    world.function_mappings.insert(
        "v2.CreateServiceAccountApplicationKey".into(),
        test_v2_create_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.DeleteServiceAccountApplicationKey".into(),
        test_v2_delete_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.GetServiceAccountApplicationKey".into(),
        test_v2_get_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.UpdateServiceAccountApplicationKey".into(),
        test_v2_update_service_account_application_key,
    );
    world.function_mappings.insert(
        "v2.ListIncidentServices".into(),
        test_v2_list_incident_services,
    );
    world.function_mappings.insert(
        "v2.CreateIncidentService".into(),
        test_v2_create_incident_service,
    );
    world.function_mappings.insert(
        "v2.DeleteIncidentService".into(),
        test_v2_delete_incident_service,
    );
    world
        .function_mappings
        .insert("v2.GetIncidentService".into(), test_v2_get_incident_service);
    world.function_mappings.insert(
        "v2.UpdateIncidentService".into(),
        test_v2_update_incident_service,
    );
    world.function_mappings.insert(
        "v2.ListServiceDefinitions".into(),
        test_v2_list_service_definitions,
    );
    world.function_mappings.insert(
        "v2.ListServiceDefinitionsWithPagination".into(),
        test_v2_list_service_definitions_with_pagination,
    );
    world.function_mappings.insert(
        "v2.CreateOrUpdateServiceDefinitions".into(),
        test_v2_create_or_update_service_definitions,
    );
    world.function_mappings.insert(
        "v2.DeleteServiceDefinition".into(),
        test_v2_delete_service_definition,
    );
    world.function_mappings.insert(
        "v2.GetServiceDefinition".into(),
        test_v2_get_service_definition,
    );
    world.function_mappings.insert(
        "v2.CreateSLOReportJob".into(),
        test_v2_create_slo_report_job,
    );
    world
        .function_mappings
        .insert("v2.GetSLOReport".into(), test_v2_get_slo_report);
    world.function_mappings.insert(
        "v2.GetSLOReportJobStatus".into(),
        test_v2_get_slo_report_job_status,
    );
    world
        .function_mappings
        .insert("v2.AggregateSpans".into(), test_v2_aggregate_spans);
    world
        .function_mappings
        .insert("v2.ListSpansGet".into(), test_v2_list_spans_get);
    world.function_mappings.insert(
        "v2.ListSpansGetWithPagination".into(),
        test_v2_list_spans_get_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.ListSpans".into(), test_v2_list_spans);
    world.function_mappings.insert(
        "v2.ListSpansWithPagination".into(),
        test_v2_list_spans_with_pagination,
    );
    world.function_mappings.insert(
        "v2.GetOnDemandConcurrencyCap".into(),
        test_v2_get_on_demand_concurrency_cap,
    );
    world.function_mappings.insert(
        "v2.SetOnDemandConcurrencyCap".into(),
        test_v2_set_on_demand_concurrency_cap,
    );
    world
        .function_mappings
        .insert("v2.ListTeams".into(), test_v2_list_teams);
    world.function_mappings.insert(
        "v2.ListTeamsWithPagination".into(),
        test_v2_list_teams_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreateTeam".into(), test_v2_create_team);
    world
        .function_mappings
        .insert("v2.DeleteTeam".into(), test_v2_delete_team);
    world
        .function_mappings
        .insert("v2.GetTeam".into(), test_v2_get_team);
    world
        .function_mappings
        .insert("v2.UpdateTeam".into(), test_v2_update_team);
    world
        .function_mappings
        .insert("v2.GetTeamLinks".into(), test_v2_get_team_links);
    world
        .function_mappings
        .insert("v2.CreateTeamLink".into(), test_v2_create_team_link);
    world
        .function_mappings
        .insert("v2.DeleteTeamLink".into(), test_v2_delete_team_link);
    world
        .function_mappings
        .insert("v2.GetTeamLink".into(), test_v2_get_team_link);
    world
        .function_mappings
        .insert("v2.UpdateTeamLink".into(), test_v2_update_team_link);
    world
        .function_mappings
        .insert("v2.GetTeamMemberships".into(), test_v2_get_team_memberships);
    world.function_mappings.insert(
        "v2.GetTeamMembershipsWithPagination".into(),
        test_v2_get_team_memberships_with_pagination,
    );
    world.function_mappings.insert(
        "v2.CreateTeamMembership".into(),
        test_v2_create_team_membership,
    );
    world.function_mappings.insert(
        "v2.DeleteTeamMembership".into(),
        test_v2_delete_team_membership,
    );
    world.function_mappings.insert(
        "v2.UpdateTeamMembership".into(),
        test_v2_update_team_membership,
    );
    world.function_mappings.insert(
        "v2.GetTeamPermissionSettings".into(),
        test_v2_get_team_permission_settings,
    );
    world.function_mappings.insert(
        "v2.UpdateTeamPermissionSetting".into(),
        test_v2_update_team_permission_setting,
    );
    world
        .function_mappings
        .insert("v2.GetUserMemberships".into(), test_v2_get_user_memberships);
    world
        .function_mappings
        .insert("v2.ListIncidentTeams".into(), test_v2_list_incident_teams);
    world
        .function_mappings
        .insert("v2.CreateIncidentTeam".into(), test_v2_create_incident_team);
    world
        .function_mappings
        .insert("v2.DeleteIncidentTeam".into(), test_v2_delete_incident_team);
    world
        .function_mappings
        .insert("v2.GetIncidentTeam".into(), test_v2_get_incident_team);
    world
        .function_mappings
        .insert("v2.UpdateIncidentTeam".into(), test_v2_update_incident_team);
    world
        .function_mappings
        .insert("v2.SendInvitations".into(), test_v2_send_invitations);
    world
        .function_mappings
        .insert("v2.GetInvitation".into(), test_v2_get_invitation);
    world
        .function_mappings
        .insert("v2.ListUsers".into(), test_v2_list_users);
    world.function_mappings.insert(
        "v2.ListUsersWithPagination".into(),
        test_v2_list_users_with_pagination,
    );
    world
        .function_mappings
        .insert("v2.CreateUser".into(), test_v2_create_user);
    world
        .function_mappings
        .insert("v2.DisableUser".into(), test_v2_disable_user);
    world
        .function_mappings
        .insert("v2.GetUser".into(), test_v2_get_user);
    world
        .function_mappings
        .insert("v2.UpdateUser".into(), test_v2_update_user);
    world.function_mappings.insert(
        "v2.ListUserOrganizations".into(),
        test_v2_list_user_organizations,
    );
    world.function_mappings.insert(
        "v2.ListUserPermissions".into(),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetDailyCustomReportsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort_dir = sort_dir;
    params.sort = sort;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetMonthlyCustomReportsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort_dir = sort_dir;
    params.sort = sort;
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageAnalyzedLogsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageAuditLogsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageLambdaOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let month = _parameters
        .get("month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_connected_accounts = _parameters
        .get("include_connected_accounts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetUsageBillableSummaryOptionalParams::default();
    params.month = month;
    params.include_connected_accounts = include_connected_accounts;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageCIAppOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageCloudSecurityPostureManagementOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageCWSOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageDBMOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageFargateOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageHostsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let next_record_id = _parameters
        .get("next_record_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tag_breakdown_keys = _parameters
        .get("tag_breakdown_keys")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_descendants = _parameters
        .get("include_descendants")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetHourlyUsageAttributionOptionalParams::default();
    params.end_hr = end_hr;
    params.next_record_id = next_record_id;
    params.tag_breakdown_keys = tag_breakdown_keys;
    params.include_descendants = include_descendants;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetIncidentManagementOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageIndexedSpansOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetIngestedSpansOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetUsageInternetOfThingsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageLogsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetUsageLogsByRetentionOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let index_name = _parameters
        .get("index_name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageLogsByIndexOptionalParams::default();
    params.end_hr = end_hr;
    params.index_name = index_name;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_month = _parameters
        .get("end_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_direction = _parameters
        .get("sort_direction")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_name = _parameters
        .get("sort_name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tag_breakdown_keys = _parameters
        .get("tag_breakdown_keys")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let next_record_id = _parameters
        .get("next_record_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_descendants = _parameters
        .get("include_descendants")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetMonthlyUsageAttributionOptionalParams::default();
    params.end_month = end_month;
    params.sort_direction = sort_direction;
    params.sort_name = sort_name;
    params.tag_breakdown_keys = tag_breakdown_keys;
    params.next_record_id = next_record_id;
    params.include_descendants = include_descendants;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageNetworkFlowsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageNetworkHostsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageOnlineArchiveOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageProfilingOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageRumUnitsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let type_ = _parameters
        .get("type")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageRumSessionsOptionalParams::default();
    params.end_hr = end_hr;
    params.type_ = type_;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageSDSOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageSNMPOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_month = _parameters
        .get("end_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_org_details = _parameters
        .get("include_org_details")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_connected_accounts = _parameters
        .get("include_connected_accounts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageSummaryOptionalParams::default();
    params.end_month = end_month;
    params.include_org_details = include_org_details;
    params.include_connected_accounts = include_connected_accounts;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageSyntheticsOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageSyntheticsAPIOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_usage_metering::GetUsageSyntheticsBrowserOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageTimeseriesOptionalParams::default();
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let month = _parameters
        .get("month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let day = _parameters
        .get("day")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let names = _parameters
        .get("names")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let limit = _parameters
        .get("limit")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let next_record_id = _parameters
        .get("next_record_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_usage_metering::GetUsageTopAvgMetricsOptionalParams::default();
    params.month = month;
    params.day = day;
    params.names = names;
    params.limit = limit;
    params.next_record_id = next_record_id;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_shared = _parameters
        .get("filter[shared]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_deleted = _parameters
        .get("filter[deleted]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let count = _parameters
        .get("count")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start = _parameters
        .get("start")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_dashboards::ListDashboardsOptionalParams::default();
    params.filter_shared = filter_shared;
    params.filter_deleted = filter_deleted;
    params.count = count;
    params.start = start;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_dashboards_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_dashboards
        .as_ref()
        .expect("api instance not found");
    let filter_shared = _parameters
        .get("filter[shared]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_deleted = _parameters
        .get("filter[deleted]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let count = _parameters
        .get("count")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start = _parameters
        .get("start")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_dashboards::ListDashboardsOptionalParams::default();
    params.filter_shared = filter_shared;
    params.filter_deleted = filter_deleted;
    params.count = count;
    params.start = start;
    let response = api.list_dashboards_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page_size")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page_number")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_dashboards::GetPublicDashboardInvitationsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let content_encoding = _parameters
        .get("Content-Encoding")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_metrics::SubmitDistributionPointsOptionalParams::default();
    params.content_encoding = content_encoding;
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
                _ => panic!("error parsing response: {error}"),
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
    let host = _parameters
        .get("host")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tag_filter = _parameters
        .get("tag_filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_metrics::ListActiveMetricsOptionalParams::default();
    params.host = host;
    params.tag_filter = tag_filter;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let content_encoding = _parameters
        .get("Content-Encoding")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_metrics::SubmitMetricsOptionalParams::default();
    params.content_encoding = content_encoding;
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
                _ => panic!("error parsing response: {error}"),
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
    let current_only = _parameters
        .get("current_only")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let with_creator = _parameters
        .get("with_creator")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_downtimes::ListDowntimesOptionalParams::default();
    params.current_only = current_only;
    params.with_creator = with_creator;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let priority = _parameters
        .get("priority")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sources = _parameters
        .get("sources")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags = _parameters
        .get("tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let unaggregated = _parameters
        .get("unaggregated")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let exclude_aggregate = _parameters
        .get("exclude_aggregate")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page = _parameters
        .get("page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_events::ListEventsOptionalParams::default();
    params.priority = priority;
    params.sources = sources;
    params.tags = tags;
    params.unaggregated = unaggregated;
    params.exclude_aggregate = exclude_aggregate;
    params.page = page;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let metric_query = _parameters
        .get("metric_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let event_query = _parameters
        .get("event_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let graph_def = _parameters
        .get("graph_def")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let title = _parameters
        .get("title")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let height = _parameters
        .get("height")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let width = _parameters
        .get("width")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_snapshots::GetGraphSnapshotOptionalParams::default();
    params.metric_query = metric_query;
    params.event_query = event_query;
    params.graph_def = graph_def;
    params.title = title;
    params.height = height;
    params.width = width;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_field = _parameters
        .get("sort_field")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start = _parameters
        .get("start")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let count = _parameters
        .get("count")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let from = _parameters
        .get("from")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_muted_hosts_data = _parameters
        .get("include_muted_hosts_data")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_hosts_metadata = _parameters
        .get("include_hosts_metadata")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_hosts::ListHostsOptionalParams::default();
    params.filter = filter;
    params.sort_field = sort_field;
    params.sort_dir = sort_dir;
    params.start = start;
    params.count = count;
    params.from = from;
    params.include_muted_hosts_data = include_muted_hosts_data;
    params.include_hosts_metadata = include_hosts_metadata;
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
                _ => panic!("error parsing response: {error}"),
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
    let from = _parameters
        .get("from")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_hosts::GetHostTotalsOptionalParams::default();
    params.from = from;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let account_id = _parameters
        .get("account_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let role_name = _parameters
        .get("role_name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let access_key_id = _parameters
        .get("access_key_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_aws_integration::ListAWSAccountsOptionalParams::default();
    params.account_id = account_id;
    params.role_name = role_name;
    params.access_key_id = access_key_id;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let account_id = _parameters
        .get("account_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let role_name = _parameters
        .get("role_name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let access_key_id = _parameters
        .get("access_key_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_aws_integration::UpdateAWSAccountOptionalParams::default();
    params.account_id = account_id;
    params.role_name = role_name;
    params.access_key_id = access_key_id;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let content_encoding = _parameters
        .get("Content-Encoding")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let ddtags = _parameters
        .get("ddtags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_logs::SubmitLogOptionalParams::default();
    params.content_encoding = content_encoding;
    params.ddtags = ddtags;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let group_states = _parameters
        .get("group_states")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let name = _parameters
        .get("name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags = _parameters
        .get("tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let monitor_tags = _parameters
        .get("monitor_tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let with_downtimes = _parameters
        .get("with_downtimes")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let id_offset = _parameters
        .get("id_offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page = _parameters
        .get("page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page_size")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::ListMonitorsOptionalParams::default();
    params.group_states = group_states;
    params.name = name;
    params.tags = tags;
    params.monitor_tags = monitor_tags;
    params.with_downtimes = with_downtimes;
    params.id_offset = id_offset;
    params.page = page;
    params.page_size = page_size;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_monitors_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_monitors
        .as_ref()
        .expect("api instance not found");
    let group_states = _parameters
        .get("group_states")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let name = _parameters
        .get("name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags = _parameters
        .get("tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let monitor_tags = _parameters
        .get("monitor_tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let with_downtimes = _parameters
        .get("with_downtimes")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let id_offset = _parameters
        .get("id_offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page = _parameters
        .get("page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page_size")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::ListMonitorsOptionalParams::default();
    params.group_states = group_states;
    params.name = name;
    params.tags = tags;
    params.monitor_tags = monitor_tags;
    params.with_downtimes = with_downtimes;
    params.id_offset = id_offset;
    params.page = page;
    params.page_size = page_size;
    let response = api.list_monitors_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page = _parameters
        .get("page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let per_page = _parameters
        .get("per_page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::SearchMonitorGroupsOptionalParams::default();
    params.query = query;
    params.page = page;
    params.per_page = per_page;
    params.sort = sort;
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
                _ => panic!("error parsing response: {error}"),
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
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page = _parameters
        .get("page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let per_page = _parameters
        .get("per_page")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::SearchMonitorsOptionalParams::default();
    params.query = query;
    params.page = page;
    params.per_page = per_page;
    params.sort = sort;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let force = _parameters
        .get("force")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::DeleteMonitorOptionalParams::default();
    params.force = force;
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
                _ => panic!("error parsing response: {error}"),
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
    let group_states = _parameters
        .get("group_states")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let with_downtimes = _parameters
        .get("with_downtimes")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_monitors::GetMonitorOptionalParams::default();
    params.group_states = group_states;
    params.with_downtimes = with_downtimes;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let author_handle = _parameters
        .get("author_handle")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let exclude_author_handle = _parameters
        .get("exclude_author_handle")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start = _parameters
        .get("start")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let count = _parameters
        .get("count")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_field = _parameters
        .get("sort_field")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_cells = _parameters
        .get("include_cells")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let is_template = _parameters
        .get("is_template")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let type_ = _parameters
        .get("type")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_notebooks::ListNotebooksOptionalParams::default();
    params.author_handle = author_handle;
    params.exclude_author_handle = exclude_author_handle;
    params.start = start;
    params.count = count;
    params.sort_field = sort_field;
    params.sort_dir = sort_dir;
    params.query = query;
    params.include_cells = include_cells;
    params.is_template = is_template;
    params.type_ = type_;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_notebooks_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_notebooks
        .as_ref()
        .expect("api instance not found");
    let author_handle = _parameters
        .get("author_handle")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let exclude_author_handle = _parameters
        .get("exclude_author_handle")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start = _parameters
        .get("start")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let count = _parameters
        .get("count")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_field = _parameters
        .get("sort_field")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_cells = _parameters
        .get("include_cells")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let is_template = _parameters
        .get("is_template")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let type_ = _parameters
        .get("type")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_notebooks::ListNotebooksOptionalParams::default();
    params.author_handle = author_handle;
    params.exclude_author_handle = exclude_author_handle;
    params.start = start;
    params.count = count;
    params.sort_field = sort_field;
    params.sort_dir = sort_dir;
    params.query = query;
    params.include_cells = include_cells;
    params.is_template = is_template;
    params.type_ = type_;
    let response = api.list_notebooks_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_upload_idp_for_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_organizations
        .as_ref()
        .expect("api instance not found");
    let public_id = serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap();
    let idp_file = std::fs::read(format!(
        "tests/scenarios/features/v{}/{}",
        world.api_version,
        _parameters.get("idp_file").unwrap().as_str().unwrap()
    ))
    .unwrap();
    let response = match block_on(api.upload_idp_for_org_with_http_info(public_id, idp_file)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v1_list_slos(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let ids = _parameters
        .get("ids")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags_query = _parameters
        .get("tags_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let metrics_query = _parameters
        .get("metrics_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let limit = _parameters
        .get("limit")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let offset = _parameters
        .get("offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objectives::ListSLOsOptionalParams::default();
    params.ids = ids;
    params.query = query;
    params.tags_query = tags_query;
    params.metrics_query = metrics_query;
    params.limit = limit;
    params.offset = offset;
    let response = match block_on(api.list_slos_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_slos_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let ids = _parameters
        .get("ids")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags_query = _parameters
        .get("tags_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let metrics_query = _parameters
        .get("metrics_query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let limit = _parameters
        .get("limit")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let offset = _parameters
        .get("offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objectives::ListSLOsOptionalParams::default();
    params.ids = ids;
    params.query = query;
    params.tags_query = tags_query;
    params.metrics_query = metrics_query;
    params.limit = limit;
    params.offset = offset;
    let response = api.list_slos_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_facets = _parameters
        .get("include_facets")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objectives::SearchSLOOptionalParams::default();
    params.query = query;
    params.page_size = page_size;
    params.page_number = page_number;
    params.include_facets = include_facets;
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
                _ => panic!("error parsing response: {error}"),
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
    let force = _parameters
        .get("force")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objectives::DeleteSLOOptionalParams::default();
    params.force = force;
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
                _ => panic!("error parsing response: {error}"),
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
    let with_configured_alert_ids = _parameters
        .get("with_configured_alert_ids")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objectives::GetSLOOptionalParams::default();
    params.with_configured_alert_ids = with_configured_alert_ids;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let target = _parameters
        .get("target")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let apply_correction = _parameters
        .get("apply_correction")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_service_level_objectives::GetSLOHistoryOptionalParams::default();
    params.target = target;
    params.apply_correction = apply_correction;
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
                    _ => panic!("error parsing response: {error}"),
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
    let offset = _parameters
        .get("offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let limit = _parameters
        .get("limit")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objective_corrections::ListSLOCorrectionOptionalParams::default();
    params.offset = offset;
    params.limit = limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_slo_correction_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let offset = _parameters
        .get("offset")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let limit = _parameters
        .get("limit")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_service_level_objective_corrections::ListSLOCorrectionOptionalParams::default();
    params.offset = offset;
    params.limit = limit;
    let response = api.list_slo_correction_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page_size")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page_number")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_synthetics::ListTestsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v1_list_tests_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v1_api_synthetics
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page_size")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page_number")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_synthetics::ListTestsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    let response = api.list_tests_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let from_ts = _parameters
        .get("from_ts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let to_ts = _parameters
        .get("to_ts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let probe_dc = _parameters
        .get("probe_dc")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV1::api_synthetics::GetBrowserTestLatestResultsOptionalParams::default();
    params.from_ts = from_ts;
    params.to_ts = to_ts;
    params.probe_dc = probe_dc;
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let from_ts = _parameters
        .get("from_ts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let to_ts = _parameters
        .get("to_ts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let probe_dc = _parameters
        .get("probe_dc")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_synthetics::GetAPITestLatestResultsOptionalParams::default();
    params.from_ts = from_ts;
    params.to_ts = to_ts;
    params.probe_dc = probe_dc;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let source = _parameters
        .get("source")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_tags::ListHostTagsOptionalParams::default();
    params.source = source;
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
                _ => panic!("error parsing response: {error}"),
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
    let source = _parameters
        .get("source")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_tags::DeleteHostTagsOptionalParams::default();
    params.source = source;
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
                _ => panic!("error parsing response: {error}"),
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
    let source = _parameters
        .get("source")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_tags::GetHostTagsOptionalParams::default();
    params.source = source;
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
                _ => panic!("error parsing response: {error}"),
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
    let source = _parameters
        .get("source")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_tags::CreateHostTagsOptionalParams::default();
    params.source = source;
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
                _ => panic!("error parsing response: {error}"),
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
    let source = _parameters
        .get("source")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV1::api_tags::UpdateHostTagsOptionalParams::default();
    params.source = source;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_start = _parameters
        .get("filter[created_at][start]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_end = _parameters
        .get("filter[created_at][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_modified_at_start = _parameters
        .get("filter[modified_at][start]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_modified_at_end = _parameters
        .get("filter[modified_at][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_remote_config_read_enabled = _parameters
        .get("filter[remote_config_read_enabled]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_category = _parameters
        .get("filter[category]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_key_management::ListAPIKeysOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.filter_created_at_start = filter_created_at_start;
    params.filter_created_at_end = filter_created_at_end;
    params.filter_modified_at_start = filter_modified_at_start;
    params.filter_modified_at_end = filter_modified_at_end;
    params.include = include;
    params.filter_remote_config_read_enabled = filter_remote_config_read_enabled;
    params.filter_category = filter_category;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_key_management::GetAPIKeyOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_start = _parameters
        .get("filter[created_at][start]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_end = _parameters
        .get("filter[created_at][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_key_management::ListApplicationKeysOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.filter_created_at_start = filter_created_at_start;
    params.filter_created_at_end = filter_created_at_end;
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_key_management::GetApplicationKeyOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_start = _parameters
        .get("filter[created_at][start]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_end = _parameters
        .get("filter[created_at][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_key_management::ListCurrentUserApplicationKeysOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.filter_created_at_start = filter_created_at_start;
    params.filter_created_at_end = filter_created_at_end;
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_apis(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_api_management
        .as_ref()
        .expect("api instance not found");
    let query = _parameters
        .get("query")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_api_management::ListAPIsOptionalParams::default();
    params.query = query;
    params.page_limit = page_limit;
    params.page_offset = page_offset;
    let response = match block_on(api.list_apis_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_open_api(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_api_management
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_open_api_with_http_info(id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_open_api(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_api_management
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_open_api_with_http_info(id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_open_api(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_api_management
        .as_ref()
        .expect("api instance not found");
    let id = serde_json::from_value(_parameters.get("id").unwrap().clone()).unwrap();
    let openapi_spec_file = _parameters.get("openapi_spec_file").and_then(|param| {
        std::fs::read(format!(
            "tests/scenarios/features/v{}/{}",
            world.api_version,
            param.as_str().unwrap()
        ))
        .ok()
    });
    let mut params = datadogV2::api_api_management::UpdateOpenAPIOptionalParams::default();
    params.openapi_spec_file = openapi_spec_file;
    let response = match block_on(api.update_open_api_with_http_info(id, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_open_api(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_api_management
        .as_ref()
        .expect("api instance not found");
    let openapi_spec_file = _parameters.get("openapi_spec_file").and_then(|param| {
        std::fs::read(format!(
            "tests/scenarios/features/v{}/{}",
            world.api_version,
            param.as_str().unwrap()
        ))
        .ok()
    });
    let mut params = datadogV2::api_api_management::CreateOpenAPIOptionalParams::default();
    params.openapi_spec_file = openapi_spec_file;
    let response = match block_on(api.create_open_api_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_audit::ListAuditLogsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_audit_logs_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_audit
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_audit::ListAuditLogsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_audit_logs_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_search_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_audit
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_audit::SearchAuditLogsOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_audit_logs_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_audit
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_audit::SearchAuditLogsOptionalParams::default();
    params.body = body;
    let response = api.search_audit_logs_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_authn_mappings(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_authn_mappings
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let resource_type = _parameters
        .get("resource_type")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_authn_mappings::ListAuthNMappingsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.resource_type = resource_type;
    let response = match block_on(api.list_authn_mappings_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_authn_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_authn_mappings
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_authn_mapping_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_authn_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_authn_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_authn_mapping_with_http_info(authn_mapping_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_authn_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_authn_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_authn_mapping_with_http_info(authn_mapping_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_authn_mapping(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_authn_mappings
        .as_ref()
        .expect("api instance not found");
    let authn_mapping_id =
        serde_json::from_value(_parameters.get("authn_mapping_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_authn_mapping_with_http_info(authn_mapping_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_search_cases(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_field = _parameters
        .get("sort[field]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_asc = _parameters
        .get("sort[asc]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_case_management::SearchCasesOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort_field = sort_field;
    params.filter = filter;
    params.sort_asc = sort_asc;
    let response = match block_on(api.search_cases_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_cases_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_field = _parameters
        .get("sort[field]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_asc = _parameters
        .get("sort[asc]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_case_management::SearchCasesOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort_field = sort_field;
    params.filter = filter;
    params.sort_asc = sort_asc;
    let response = api.search_cases_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_create_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_case_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_projects(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.get_projects_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_project(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_project_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_project(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let project_id =
        serde_json::from_value(_parameters.get("project_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_project_with_http_info(project_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_project(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let project_id =
        serde_json::from_value(_parameters.get("project_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_project_with_http_info(project_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_case_with_http_info(case_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_archive_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.archive_case_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_assign_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.assign_case_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_priority(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_priority_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_status(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_status_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_unarchive_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.unarchive_case_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_unassign_case(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_case_management
        .as_ref()
        .expect("api instance not found");
    let case_id = serde_json::from_value(_parameters.get("case_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.unassign_case_with_http_info(case_id, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_pipelines::ListCIAppPipelineEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_ci_app_pipeline_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_pipelines::ListCIAppPipelineEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_ci_app_pipeline_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_pipelines::SearchCIAppPipelineEventsOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_ci_app_pipeline_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_pipelines
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_pipelines::SearchCIAppPipelineEventsOptionalParams::default();
    params.body = body;
    let response = api.search_ci_app_pipeline_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_tests::ListCIAppTestEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_ci_app_test_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_tests
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_tests::ListCIAppTestEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_ci_app_test_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_tests::SearchCIAppTestEventsOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_ci_app_test_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_ci_visibility_tests
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_ci_visibility_tests::SearchCIAppTestEventsOptionalParams::default();
    params.body = body;
    let response = api.search_ci_app_test_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_container_images(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_container_images
        .as_ref()
        .expect("api instance not found");
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let group_by = _parameters
        .get("group_by")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_container_images::ListContainerImagesOptionalParams::default();
    params.filter_tags = filter_tags;
    params.group_by = group_by;
    params.sort = sort;
    params.page_size = page_size;
    params.page_cursor = page_cursor;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_container_images_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_container_images
        .as_ref()
        .expect("api instance not found");
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let group_by = _parameters
        .get("group_by")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_container_images::ListContainerImagesOptionalParams::default();
    params.filter_tags = filter_tags;
    params.group_by = group_by;
    params.sort = sort;
    params.page_size = page_size;
    params.page_cursor = page_cursor;
    let response = api.list_container_images_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_containers(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_containers
        .as_ref()
        .expect("api instance not found");
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let group_by = _parameters
        .get("group_by")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_containers::ListContainersOptionalParams::default();
    params.filter_tags = filter_tags;
    params.group_by = group_by;
    params.sort = sort;
    params.page_size = page_size;
    params.page_cursor = page_cursor;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_containers_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_containers
        .as_ref()
        .expect("api instance not found");
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let group_by = _parameters
        .get("group_by")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_containers::ListContainersOptionalParams::default();
    params.filter_tags = filter_tags;
    params.group_by = group_by;
    params.sort = sort;
    params.page_size = page_size;
    params.page_cursor = page_cursor;
    let response = api.list_containers_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let sort_direction = _parameters
        .get("sort_direction")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_name = _parameters
        .get("sort_name")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tag_breakdown_keys = _parameters
        .get("tag_breakdown_keys")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let next_record_id = _parameters
        .get("next_record_id")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_descendants = _parameters
        .get("include_descendants")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_usage_metering::GetMonthlyCostAttributionOptionalParams::default();
    params.sort_direction = sort_direction;
    params.sort_name = sort_name;
    params.tag_breakdown_keys = tag_breakdown_keys;
    params.next_record_id = next_record_id;
    params.include_descendants = include_descendants;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_usage_metering::GetUsageApplicationSecurityMonitoringOptionalParams::default(
        );
    params.end_hr = end_hr;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_month = _parameters
        .get("end_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_usage_metering::GetCostByOrgOptionalParams::default();
    params.end_month = end_month;
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
                _ => panic!("error parsing response: {error}"),
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
    let view = _parameters
        .get("view")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start_month = _parameters
        .get("start_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let end_month = _parameters
        .get("end_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let start_date = _parameters
        .get("start_date")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let end_date = _parameters
        .get("end_date")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_connected_accounts = _parameters
        .get("include_connected_accounts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_usage_metering::GetEstimatedCostByOrgOptionalParams::default();
    params.view = view;
    params.start_month = start_month;
    params.end_month = end_month;
    params.start_date = start_date;
    params.end_date = end_date;
    params.include_connected_accounts = include_connected_accounts;
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
                _ => panic!("error parsing response: {error}"),
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
    let view = _parameters
        .get("view")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let end_month = _parameters
        .get("end_month")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_connected_accounts = _parameters
        .get("include_connected_accounts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_usage_metering::GetHistoricalCostByOrgOptionalParams::default();
    params.view = view;
    params.end_month = end_month;
    params.include_connected_accounts = include_connected_accounts;
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
                    _ => panic!("error parsing response: {error}"),
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
    let filter_timestamp_end = _parameters
        .get("filter[timestamp][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_include_descendants = _parameters
        .get("filter[include_descendants]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_include_connected_accounts = _parameters
        .get("filter[include_connected_accounts]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_include_breakdown = _parameters
        .get("filter[include_breakdown]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_versions = _parameters
        .get("filter[versions]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_next_record_id = _parameters
        .get("page[next_record_id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_usage_metering::GetHourlyUsageOptionalParams::default();
    params.filter_timestamp_end = filter_timestamp_end;
    params.filter_include_descendants = filter_include_descendants;
    params.filter_include_connected_accounts = filter_include_connected_accounts;
    params.filter_include_breakdown = filter_include_breakdown;
    params.filter_versions = filter_versions;
    params.page_limit = page_limit;
    params.page_next_record_id = page_next_record_id;
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
                _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_usage_metering::GetUsageLambdaTracedInvocationsOptionalParams::default();
    params.end_hr = end_hr;
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
                    _ => panic!("error parsing response: {error}"),
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
    let end_hr = _parameters
        .get("end_hr")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_usage_metering::GetUsageObservabilityPipelinesOptionalParams::default();
    params.end_hr = end_hr;
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
                    _ => panic!("error parsing response: {error}"),
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
    let view = _parameters
        .get("view")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include_connected_accounts = _parameters
        .get("include_connected_accounts")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_usage_metering::GetProjectedCostOptionalParams::default();
    params.view = view;
    params.include_connected_accounts = include_connected_accounts;
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let current_only = _parameters
        .get("current_only")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_downtimes::ListDowntimesOptionalParams::default();
    params.current_only = current_only;
    params.include = include;
    params.page_offset = page_offset;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_downtimes_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let current_only = _parameters
        .get("current_only")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_downtimes::ListDowntimesOptionalParams::default();
    params.current_only = current_only;
    params.include = include;
    params.page_offset = page_offset;
    params.page_limit = page_limit;
    let response = api.list_downtimes_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_downtimes::GetDowntimeOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_downtimes::ListMonitorDowntimesOptionalParams::default();
    params.page_offset = page_offset;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_monitor_downtimes_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_downtimes
        .as_ref()
        .expect("api instance not found");
    let monitor_id =
        serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap();
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_downtimes::ListMonitorDowntimesOptionalParams::default();
    params.page_offset = page_offset;
    params.page_limit = page_limit;
    let response = api.list_monitor_downtimes_with_pagination(monitor_id, params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_events::ListEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_events::ListEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_search_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_events::SearchEventsOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_events
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_events::SearchEventsOptionalParams::default();
    params.body = body;
    let response = api.search_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_incidents(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::ListIncidentsOptionalParams::default();
    params.include = include;
    params.page_size = page_size;
    params.page_offset = page_offset;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_incidents_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::ListIncidentsOptionalParams::default();
    params.include = include;
    params.page_size = page_size;
    params.page_offset = page_offset;
    let response = api.list_incidents_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::SearchIncidentsOptionalParams::default();
    params.include = include;
    params.sort = sort;
    params.page_size = page_size;
    params.page_offset = page_offset;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_incidents_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_incidents
        .as_ref()
        .expect("api instance not found");
    let query = serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap();
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::SearchIncidentsOptionalParams::default();
    params.include = include;
    params.sort = sort;
    params.page_size = page_size;
    params.page_offset = page_offset;
    let response = api.search_incidents_with_pagination(query, params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::GetIncidentOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::UpdateIncidentOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_attachment_type = _parameters
        .get("filter[attachment_type]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::ListIncidentAttachmentsOptionalParams::default();
    params.include = include;
    params.filter_attachment_type = filter_attachment_type;
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incidents::UpdateIncidentAttachmentsOptionalParams::default();
    params.include = include;
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_gcp_integration::MakeGCPSTSDelegateOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let content_encoding = _parameters
        .get("Content-Encoding")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let ddtags = _parameters
        .get("ddtags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_logs::SubmitLogOptionalParams::default();
    params.content_encoding = content_encoding;
    params.ddtags = ddtags;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_indexes = _parameters
        .get("filter[indexes]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_storage_tier = _parameters
        .get("filter[storage_tier]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_logs::ListLogsGetOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_indexes = filter_indexes;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.filter_storage_tier = filter_storage_tier;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_logs_get_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_indexes = _parameters
        .get("filter[indexes]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_storage_tier = _parameters
        .get("filter[storage_tier]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_logs::ListLogsGetOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_indexes = filter_indexes;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.filter_storage_tier = filter_storage_tier;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_logs_get_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_logs::ListLogsOptionalParams::default();
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_logs_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_logs::ListLogsOptionalParams::default();
    params.body = body;
    let response = api.list_logs_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_logs_custom_destinations(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_custom_destinations
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_logs_custom_destinations_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_logs_custom_destination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_custom_destinations
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_logs_custom_destination_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_logs_custom_destination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_custom_destinations
        .as_ref()
        .expect("api instance not found");
    let custom_destination_id =
        serde_json::from_value(_parameters.get("custom_destination_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_logs_custom_destination_with_http_info(custom_destination_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_logs_custom_destination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_custom_destinations
        .as_ref()
        .expect("api instance not found");
    let custom_destination_id =
        serde_json::from_value(_parameters.get("custom_destination_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_logs_custom_destination_with_http_info(custom_destination_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_logs_custom_destination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_logs_custom_destinations
        .as_ref()
        .expect("api instance not found");
    let custom_destination_id =
        serde_json::from_value(_parameters.get("custom_destination_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.update_logs_custom_destination_with_http_info(custom_destination_id, body),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_configured = _parameters
        .get("filter[configured]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_tags_configured = _parameters
        .get("filter[tags_configured]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_metric_type = _parameters
        .get("filter[metric_type]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_include_percentiles = _parameters
        .get("filter[include_percentiles]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_queried = _parameters
        .get("filter[queried]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let window_seconds = _parameters
        .get("window[seconds]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_metrics::ListTagConfigurationsOptionalParams::default();
    params.filter_configured = filter_configured;
    params.filter_tags_configured = filter_tags_configured;
    params.filter_metric_type = filter_metric_type;
    params.filter_include_percentiles = filter_include_percentiles;
    params.filter_queried = filter_queried;
    params.filter_tags = filter_tags;
    params.window_seconds = window_seconds;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let window_seconds = _parameters
        .get("window[seconds]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_metrics::ListActiveMetricConfigurationsOptionalParams::default();
    params.window_seconds = window_seconds;
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_metric_assets(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_metrics
        .as_ref()
        .expect("api instance not found");
    let metric_name =
        serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap();
    let response = match block_on(api.list_metric_assets_with_http_info(metric_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
    let filter_groups = _parameters
        .get("filter[groups]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_hours_ago = _parameters
        .get("filter[hours_ago]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_num_aggregations = _parameters
        .get("filter[num_aggregations]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_pct = _parameters
        .get("filter[pct]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_timespan_h = _parameters
        .get("filter[timespan_h]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_metrics::EstimateMetricsOutputSeriesOptionalParams::default();
    params.filter_groups = filter_groups;
    params.filter_hours_ago = filter_hours_ago;
    params.filter_num_aggregations = filter_num_aggregations;
    params.filter_pct = filter_pct;
    params.filter_timespan_h = filter_timespan_h;
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let content_encoding = _parameters
        .get("Content-Encoding")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_metrics::SubmitMetricsOptionalParams::default();
    params.content_encoding = content_encoding;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_devices(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_network_device_monitoring
        .as_ref()
        .expect("api instance not found");
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_tag = _parameters
        .get("filter[tag]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_network_device_monitoring::ListDevicesOptionalParams::default();
    params.page_number = page_number;
    params.page_size = page_size;
    params.sort = sort;
    params.filter_tag = filter_tag;
    let response = match block_on(api.list_devices_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_device(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_network_device_monitoring
        .as_ref()
        .expect("api instance not found");
    let device_id = serde_json::from_value(_parameters.get("device_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_device_with_http_info(device_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_interfaces(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_network_device_monitoring
        .as_ref()
        .expect("api instance not found");
    let device_id = serde_json::from_value(_parameters.get("device_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_interfaces_with_http_info(device_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_org_configs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_organizations
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_org_configs_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_org_config(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_organizations
        .as_ref()
        .expect("api instance not found");
    let org_config_name =
        serde_json::from_value(_parameters.get("org_config_name").unwrap().clone()).unwrap();
    let response = match block_on(api.get_org_config_with_http_info(org_config_name)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_org_config(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_organizations
        .as_ref()
        .expect("api instance not found");
    let org_config_name =
        serde_json::from_value(_parameters.get("org_config_name").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.update_org_config_with_http_info(org_config_name, body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_upload_idp_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_organizations
        .as_ref()
        .expect("api instance not found");
    let idp_file = _parameters.get("idp_file").and_then(|param| {
        std::fs::read(format!(
            "tests/scenarios/features/v{}/{}",
            world.api_version,
            param.as_str().unwrap()
        ))
        .ok()
    });
    let mut params = datadogV2::api_organizations::UploadIdPMetadataOptionalParams::default();
    params.idp_file = idp_file;
    let response = match block_on(api.upload_idp_metadata_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_id = _parameters
        .get("filter[id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_roles::ListRolesOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.filter_id = filter_id;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_roles::ListRoleUsersOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let snapshot_timestamp = _parameters
        .get("snapshot_timestamp")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_evaluation_changed_at = _parameters
        .get("filter[evaluation_changed_at]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_muted = _parameters
        .get("filter[muted]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule_id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule_name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_resource_type = _parameters
        .get("filter[resource_type]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_discovery_timestamp = _parameters
        .get("filter[discovery_timestamp]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_evaluation = _parameters
        .get("filter[evaluation]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_status = _parameters
        .get("filter[status]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_security_monitoring::ListFindingsOptionalParams::default();
    params.page_limit = page_limit;
    params.snapshot_timestamp = snapshot_timestamp;
    params.page_cursor = page_cursor;
    params.filter_tags = filter_tags;
    params.filter_evaluation_changed_at = filter_evaluation_changed_at;
    params.filter_muted = filter_muted;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_name = filter_rule_name;
    params.filter_resource_type = filter_resource_type;
    params.filter_discovery_timestamp = filter_discovery_timestamp;
    params.filter_evaluation = filter_evaluation;
    params.filter_status = filter_status;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_findings_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let snapshot_timestamp = _parameters
        .get("snapshot_timestamp")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_tags = _parameters
        .get("filter[tags]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_evaluation_changed_at = _parameters
        .get("filter[evaluation_changed_at]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_muted = _parameters
        .get("filter[muted]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule_id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule_name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_resource_type = _parameters
        .get("filter[resource_type]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_discovery_timestamp = _parameters
        .get("filter[discovery_timestamp]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_evaluation = _parameters
        .get("filter[evaluation]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_status = _parameters
        .get("filter[status]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_security_monitoring::ListFindingsOptionalParams::default();
    params.page_limit = page_limit;
    params.snapshot_timestamp = snapshot_timestamp;
    params.page_cursor = page_cursor;
    params.filter_tags = filter_tags;
    params.filter_evaluation_changed_at = filter_evaluation_changed_at;
    params.filter_muted = filter_muted;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_name = filter_rule_name;
    params.filter_resource_type = filter_resource_type;
    params.filter_discovery_timestamp = filter_discovery_timestamp;
    params.filter_evaluation = filter_evaluation;
    params.filter_status = filter_status;
    let response = api.list_findings_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
    let snapshot_timestamp = _parameters
        .get("snapshot_timestamp")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_security_monitoring::GetFindingOptionalParams::default();
    params.snapshot_timestamp = snapshot_timestamp;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_list_security_monitoring_suppressions(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_security_monitoring_suppressions_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_security_monitoring_suppression(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_security_monitoring_suppression_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_security_monitoring_suppression(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let suppression_id =
        serde_json::from_value(_parameters.get("suppression_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.delete_security_monitoring_suppression_with_http_info(suppression_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_security_monitoring_suppression(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let suppression_id =
        serde_json::from_value(_parameters.get("suppression_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.get_security_monitoring_suppression_with_http_info(suppression_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_security_monitoring_suppression(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let suppression_id =
        serde_json::from_value(_parameters.get("suppression_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.update_security_monitoring_suppression_with_http_info(suppression_id, body),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_security_monitoring::ListSecurityMonitoringRulesOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_convert_security_monitoring_rule_from_json_to_terraform(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(
        api.convert_security_monitoring_rule_from_json_to_terraform_with_http_info(body),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_test_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.test_security_monitoring_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_validate_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.validate_security_monitoring_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_convert_existing_security_monitoring_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let rule_id = serde_json::from_value(_parameters.get("rule_id").unwrap().clone()).unwrap();
    let response =
        match block_on(api.convert_existing_security_monitoring_rule_with_http_info(rule_id)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_test_existing_security_monitoring_rule(
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
    let response =
        match block_on(api.test_existing_security_monitoring_rule_with_http_info(rule_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_security_monitoring::ListSecurityMonitoringSignalsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_security_monitoring_signals_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_security_monitoring::ListSecurityMonitoringSignalsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_security_monitoring_signals_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_security_monitoring::SearchSecurityMonitoringSignalsOptionalParams::default(
        );
    params.body = body;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_security_monitoring_signals_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let body = _parameters
        .get("body")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_security_monitoring::SearchSecurityMonitoringSignalsOptionalParams::default(
        );
    params.body = body;
    let response = api.search_security_monitoring_signals_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_powerpack::ListPowerpacksOptionalParams::default();
    params.page_limit = page_limit;
    params.page_offset = page_offset;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_powerpacks_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_powerpack
        .as_ref()
        .expect("api instance not found");
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_powerpack::ListPowerpacksOptionalParams::default();
    params.page_limit = page_limit;
    params.page_offset = page_offset;
    let response = api.list_powerpacks_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let search = _parameters
        .get("search")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags = _parameters
        .get("tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let from = _parameters
        .get("from")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let to = _parameters
        .get("to")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_processes::ListProcessesOptionalParams::default();
    params.search = search;
    params.tags = tags;
    params.from = from;
    params.to = to;
    params.page_limit = page_limit;
    params.page_cursor = page_cursor;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_processes_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_processes
        .as_ref()
        .expect("api instance not found");
    let search = _parameters
        .get("search")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let tags = _parameters
        .get("tags")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let from = _parameters
        .get("from")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let to = _parameters
        .get("to")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_processes::ListProcessesOptionalParams::default();
    params.search = search;
    params.tags = tags;
    params.from = from;
    params.to = to;
    params.page_limit = page_limit;
    params.page_cursor = page_cursor;
    let response = api.list_processes_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_csm_threats_agent_rules(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.list_csm_threats_agent_rules_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_csm_threats_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_csm_threats_agent_rule_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_delete_csm_threats_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let response = match block_on(api.delete_csm_threats_agent_rule_with_http_info(agent_rule_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_csm_threats_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_csm_threats_agent_rule_with_http_info(agent_rule_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_csm_threats_agent_rule(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let agent_rule_id =
        serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response =
        match block_on(api.update_csm_threats_agent_rule_with_http_info(agent_rule_id, body)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => {
                        world.response.code = e.status.as_u16();
                        if let Some(entity) = e.entity {
                            world.response.object = serde_json::to_value(entity).unwrap();
                        }
                    }
                    _ => panic!("error parsing response: {error}"),
                };
            }
        };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_download_csm_threats_policy(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_csm_threats
        .as_ref()
        .expect("api instance not found");
    let response = match block_on(api.download_csm_threats_policy_with_http_info()) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                    _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                    _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                    _ => panic!("error parsing response: {error}"),
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
        .v2_api_csm_threats
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_rum::ListRUMEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_rum_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_rum::ListRUMEventsOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_rum_events_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_search_rum_events_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_rum
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = api.search_rum_events_with_pagination(body);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
}

fn test_v2_list_scorecard_outcomes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_outcome = _parameters
        .get("fields[outcome]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_rule = _parameters
        .get("fields[rule]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_outcome_service_name = _parameters
        .get("filter[outcome][service_name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_outcome_state = _parameters
        .get("filter[outcome][state]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_enabled = _parameters
        .get("filter[rule][enabled]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule][id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule][name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_scorecards::ListScorecardOutcomesOptionalParams::default();
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.include = include;
    params.fields_outcome = fields_outcome;
    params.fields_rule = fields_rule;
    params.filter_outcome_service_name = filter_outcome_service_name;
    params.filter_outcome_state = filter_outcome_state;
    params.filter_rule_enabled = filter_rule_enabled;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_name = filter_rule_name;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_scorecard_outcomes_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_outcome = _parameters
        .get("fields[outcome]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_rule = _parameters
        .get("fields[rule]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_outcome_service_name = _parameters
        .get("filter[outcome][service_name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_outcome_state = _parameters
        .get("filter[outcome][state]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_enabled = _parameters
        .get("filter[rule][enabled]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule][id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule][name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_scorecards::ListScorecardOutcomesOptionalParams::default();
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.include = include;
    params.fields_outcome = fields_outcome;
    params.fields_rule = fields_rule;
    params.filter_outcome_service_name = filter_outcome_service_name;
    params.filter_outcome_state = filter_outcome_state;
    params.filter_rule_enabled = filter_rule_enabled;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_name = filter_rule_name;
    let response = api.list_scorecard_outcomes_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule][id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_enabled = _parameters
        .get("filter[rule][enabled]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_custom = _parameters
        .get("filter[rule][custom]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule][name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_description = _parameters
        .get("filter[rule][description]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_rule = _parameters
        .get("fields[rule]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_scorecard = _parameters
        .get("fields[scorecard]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_service_scorecards::ListScorecardRulesOptionalParams::default();
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.include = include;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_enabled = filter_rule_enabled;
    params.filter_rule_custom = filter_rule_custom;
    params.filter_rule_name = filter_rule_name;
    params.filter_rule_description = filter_rule_description;
    params.fields_rule = fields_rule;
    params.fields_scorecard = fields_scorecard;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_scorecard_rules_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_scorecards
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_id = _parameters
        .get("filter[rule][id]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_enabled = _parameters
        .get("filter[rule][enabled]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_custom = _parameters
        .get("filter[rule][custom]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_name = _parameters
        .get("filter[rule][name]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_rule_description = _parameters
        .get("filter[rule][description]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_rule = _parameters
        .get("fields[rule]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_scorecard = _parameters
        .get("fields[scorecard]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_service_scorecards::ListScorecardRulesOptionalParams::default();
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.include = include;
    params.filter_rule_id = filter_rule_id;
    params.filter_rule_enabled = filter_rule_enabled;
    params.filter_rule_custom = filter_rule_custom;
    params.filter_rule_name = filter_rule_name;
    params.filter_rule_description = filter_rule_description;
    params.fields_rule = fields_rule;
    params.fields_scorecard = fields_scorecard;
    let response = api.list_scorecard_rules_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_start = _parameters
        .get("filter[created_at][start]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_created_at_end = _parameters
        .get("filter[created_at][end]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_accounts::ListServiceAccountApplicationKeysOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter = filter;
    params.filter_created_at_start = filter_created_at_start;
    params.filter_created_at_end = filter_created_at_end;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_incident_services::ListIncidentServicesOptionalParams::default();
    params.include = include;
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.filter = filter;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incident_services::GetIncidentServiceOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let schema_version = _parameters
        .get("schema_version")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_definition::ListServiceDefinitionsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.schema_version = schema_version;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_service_definitions_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_definition
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let schema_version = _parameters
        .get("schema_version")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_definition::ListServiceDefinitionsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.schema_version = schema_version;
    let response = api.list_service_definitions_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let schema_version = _parameters
        .get("schema_version")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params =
        datadogV2::api_service_definition::GetServiceDefinitionOptionalParams::default();
    params.schema_version = schema_version;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_create_slo_report_job(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = match block_on(api.create_slo_report_job_with_http_info(body)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_slo_report(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let report_id = serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_slo_report_with_http_info(report_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_get_slo_report_job_status(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let report_id = serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap();
    let response = match block_on(api.get_slo_report_job_status_with_http_info(report_id)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => {
                    world.response.code = e.status.as_u16();
                    if let Some(entity) = e.entity {
                        world.response.object = serde_json::to_value(entity).unwrap();
                    }
                }
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_spans::ListSpansGetOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_spans_get_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_spans
        .as_ref()
        .expect("api instance not found");
    let filter_query = _parameters
        .get("filter[query]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_from = _parameters
        .get("filter[from]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_to = _parameters
        .get("filter[to]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_cursor = _parameters
        .get("page[cursor]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_limit = _parameters
        .get("page[limit]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_spans::ListSpansGetOptionalParams::default();
    params.filter_query = filter_query;
    params.filter_from = filter_from;
    params.filter_to = filter_to;
    params.sort = sort;
    params.page_cursor = page_cursor;
    params.page_limit = page_limit;
    let response = api.list_spans_get_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_spans_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_spans
        .as_ref()
        .expect("api instance not found");
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let response = api.list_spans_with_pagination(body);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_keyword = _parameters
        .get("filter[keyword]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_me = _parameters
        .get("filter[me]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_team = _parameters
        .get("fields[team]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_teams::ListTeamsOptionalParams::default();
    params.page_number = page_number;
    params.page_size = page_size;
    params.sort = sort;
    params.include = include;
    params.filter_keyword = filter_keyword;
    params.filter_me = filter_me;
    params.fields_team = fields_team;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_teams_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_keyword = _parameters
        .get("filter[keyword]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_me = _parameters
        .get("filter[me]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let fields_team = _parameters
        .get("fields[team]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_teams::ListTeamsOptionalParams::default();
    params.page_number = page_number;
    params.page_size = page_size;
    params.sort = sort;
    params.include = include;
    params.filter_keyword = filter_keyword;
    params.filter_me = filter_me;
    params.fields_team = fields_team;
    let response = api.list_teams_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_keyword = _parameters
        .get("filter[keyword]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_teams::GetTeamMembershipsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter_keyword = filter_keyword;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_get_team_memberships_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_teams
        .as_ref()
        .expect("api instance not found");
    let team_id = serde_json::from_value(_parameters.get("team_id").unwrap().clone()).unwrap();
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_keyword = _parameters
        .get("filter[keyword]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_teams::GetTeamMembershipsOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.filter_keyword = filter_keyword;
    let response = api.get_team_memberships_with_pagination(team_id, params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                    _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_offset = _parameters
        .get("page[offset]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incident_teams::ListIncidentTeamsOptionalParams::default();
    params.include = include;
    params.page_size = page_size;
    params.page_offset = page_offset;
    params.filter = filter;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let include = _parameters
        .get("include")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_incident_teams::GetIncidentTeamOptionalParams::default();
    params.include = include;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_status = _parameters
        .get("filter[status]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_users::ListUsersOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.sort_dir = sort_dir;
    params.filter = filter;
    params.filter_status = filter_status;
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
fn test_v2_list_users_with_pagination(
    world: &mut DatadogWorld,
    _parameters: &HashMap<String, Value>,
) {
    let api = world
        .api_instances
        .v2_api_users
        .as_ref()
        .expect("api instance not found");
    let page_size = _parameters
        .get("page[size]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let page_number = _parameters
        .get("page[number]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort = _parameters
        .get("sort")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let sort_dir = _parameters
        .get("sort_dir")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter = _parameters
        .get("filter")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let filter_status = _parameters
        .get("filter[status]")
        .and_then(|param| Some(serde_json::from_value(param.clone()).unwrap()));
    let mut params = datadogV2::api_users::ListUsersOptionalParams::default();
    params.page_size = page_size;
    params.page_number = page_number;
    params.sort = sort;
    params.sort_dir = sort_dir;
    params.filter = filter;
    params.filter_status = filter_status;
    let response = api.list_users_with_pagination(params);
    let mut result = Vec::new();

    block_on(async {
        pin_mut!(response);

        while let Some(resp) = response.next().await {
            match resp {
                Ok(response) => {
                    result.push(response);
                }
                Err(error) => {
                    return match error {
                        Error::ResponseError(e) => {
                            if let Some(entity) = e.entity {
                                world.response.object = serde_json::to_value(entity).unwrap();
                            }
                        }
                        _ => panic!("error parsing response: {}", error),
                    };
                }
            }
        }
    });
    world.response.object = serde_json::to_value(result).unwrap();
    world.response.code = 200;
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
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
                _ => panic!("error parsing response: {error}"),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
