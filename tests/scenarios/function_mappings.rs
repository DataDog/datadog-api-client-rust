use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;
use datadog_api_client::datadogV1;
use datadog_api_client::datadogV2;

#[derive(Debug, Default)]
pub struct ApiInstances {
    pub v1_api_ip_ranges: Option<datadogV1::api::api_ip_ranges::IpRangesAPI>,
    pub v1_api_key_management: Option<datadogV1::api::api_key_management::KeyManagementAPI>,
    pub v1_api_service_checks: Option<datadogV1::api::api_service_checks::ServiceChecksAPI>,
    pub v1_api_usage_metering: Option<datadogV1::api::api_usage_metering::UsageMeteringAPI>,
    pub v1_api_dashboard_lists: Option<datadogV1::api::api_dashboard_lists::DashboardListsAPI>,
    pub v1_api_downtimes: Option<datadogV1::api::api_downtimes::DowntimesAPI>,
    pub v1_api_events: Option<datadogV1::api::api_events::EventsAPI>,
    pub v1_api_snapshots: Option<datadogV1::api::api_snapshots::SnapshotsAPI>,
    pub v1_api_hosts: Option<datadogV1::api::api_hosts::HostsAPI>,
    pub v1_api_aws_integration: Option<datadogV1::api::api_aws_integration::AwsIntegrationAPI>,
    pub v1_api_aws_logs_integration: Option<datadogV1::api::api_aws_logs_integration::AwsLogsIntegrationAPI>,
    pub v1_api_azure_integration: Option<datadogV1::api::api_azure_integration::AzureIntegrationAPI>,
    pub v1_api_gcp_integration: Option<datadogV1::api::api_gcp_integration::GcpIntegrationAPI>,
    pub v1_api_pager_duty_integration: Option<datadogV1::api::api_pager_duty_integration::PagerDutyIntegrationAPI>,
    pub v1_api_slack_integration: Option<datadogV1::api::api_slack_integration::SlackIntegrationAPI>,
    pub v1_api_webhooks_integration: Option<datadogV1::api::api_webhooks_integration::WebhooksIntegrationAPI>,
    pub v1_api_logs: Option<datadogV1::api::api_logs::LogsAPI>,
    pub v1_api_logs_indexes: Option<datadogV1::api::api_logs_indexes::LogsIndexesAPI>,
    pub v1_api_metrics: Option<datadogV1::api::api_metrics::MetricsAPI>,
    pub v1_api_organizations: Option<datadogV1::api::api_organizations::OrganizationsAPI>,
    pub v1_api_security_monitoring: Option<datadogV1::api::api_security_monitoring::SecurityMonitoringAPI>,
    pub v1_api_service_level_objectives: Option<datadogV1::api::api_service_level_objectives::ServiceLevelObjectivesAPI>,
    pub v1_api_service_level_objective_corrections: Option<datadogV1::api::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI>,
    pub v1_api_tags: Option<datadogV1::api::api_tags::TagsAPI>,
    pub v1_api_users: Option<datadogV1::api::api_users::UsersAPI>,
    pub v1_api_authentication: Option<datadogV1::api::api_authentication::AuthenticationAPI>,
    pub v2_api_spans_metrics: Option<datadogV2::api::api_spans_metrics::SpansMetricsAPI>,
    pub v2_api_apm_retention_filters: Option<datadogV2::api::api_apm_retention_filters::ApmRetentionFiltersAPI>,
    pub v2_api_audit: Option<datadogV2::api::api_audit::AuditAPI>,
    pub v2_api_dashboard_lists: Option<datadogV2::api::api_dashboard_lists::DashboardListsAPI>,
    pub v2_api_events: Option<datadogV2::api::api_events::EventsAPI>,
    pub v2_api_gcp_integration: Option<datadogV2::api::api_gcp_integration::GcpIntegrationAPI>,
    pub v2_api_opsgenie_integration: Option<datadogV2::api::api_opsgenie_integration::OpsgenieIntegrationAPI>,
    pub v2_api_cloudflare_integration: Option<datadogV2::api::api_cloudflare_integration::CloudflareIntegrationAPI>,
    pub v2_api_confluent_cloud: Option<datadogV2::api::api_confluent_cloud::ConfluentCloudAPI>,
    pub v2_api_fastly_integration: Option<datadogV2::api::api_fastly_integration::FastlyIntegrationAPI>,
    pub v2_api_ip_allowlist: Option<datadogV2::api::api_ip_allowlist::IpAllowlistAPI>,
    pub v2_api_logs: Option<datadogV2::api::api_logs::LogsAPI>,
    pub v2_api_logs_metrics: Option<datadogV2::api::api_logs_metrics::LogsMetricsAPI>,
    pub v2_api_roles: Option<datadogV2::api::api_roles::RolesAPI>,
    pub v2_api_security_monitoring: Option<datadogV2::api::api_security_monitoring::SecurityMonitoringAPI>,
    pub v2_api_processes: Option<datadogV2::api::api_processes::ProcessesAPI>,
    pub v2_api_restriction_policies: Option<datadogV2::api::api_restriction_policies::RestrictionPoliciesAPI>,
    pub v2_api_organizations: Option<datadogV2::api::api_organizations::OrganizationsAPI>,
    pub v2_api_cloud_workload_security: Option<datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI>,
    pub v2_api_metrics: Option<datadogV2::api::api_metrics::MetricsAPI>,
    pub v2_api_synthetics: Option<datadogV2::api::api_synthetics::SyntheticsAPI>,
    pub v2_api_usage_metering: Option<datadogV2::api::api_usage_metering::UsageMeteringAPI>,
}

pub fn initialize_api_instance(world: &mut DatadogWorld, api: String) {
    match api.as_str() {
        "IpRanges" => {
            if world.api_instances.v1_api_ip_ranges.is_none() {
                world.api_instances.v1_api_ip_ranges = Some(
                    datadogV1::api::api_ip_ranges::IpRangesAPI::with_config(world.config.clone()),
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
        "Downtimes" => {
            if world.api_instances.v1_api_downtimes.is_none() {
                world.api_instances.v1_api_downtimes = Some(
                    datadogV1::api::api_downtimes::DowntimesAPI::with_config(world.config.clone()),
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
        "AwsIntegration" => {
            if world.api_instances.v1_api_aws_integration.is_none() {
                world.api_instances.v1_api_aws_integration = Some(
                    datadogV1::api::api_aws_integration::AwsIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
        }
        "AwsLogsIntegration" => {
            if world.api_instances.v1_api_aws_logs_integration.is_none() {
                world.api_instances.v1_api_aws_logs_integration = Some(
                    datadogV1::api::api_aws_logs_integration::AwsLogsIntegrationAPI::with_config(
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
        "GcpIntegration" => {
            if world.api_instances.v1_api_gcp_integration.is_none() {
                world.api_instances.v1_api_gcp_integration = Some(
                    datadogV1::api::api_gcp_integration::GcpIntegrationAPI::with_config(
                        world.config.clone(),
                    ),
                );
            }
            if world.api_instances.v2_api_gcp_integration.is_none() {
                world.api_instances.v2_api_gcp_integration = Some(
                    datadogV2::api::api_gcp_integration::GcpIntegrationAPI::with_config(
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
        "ApmRetentionFilters" => {
            if world.api_instances.v2_api_apm_retention_filters.is_none() {
                world.api_instances.v2_api_apm_retention_filters = Some(
                    datadogV2::api::api_apm_retention_filters::ApmRetentionFiltersAPI::with_config(
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
        "IpAllowlist" => {
            if world.api_instances.v2_api_ip_allowlist.is_none() {
                world.api_instances.v2_api_ip_allowlist = Some(
                    datadogV2::api::api_ip_allowlist::IpAllowlistAPI::with_config(
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
        "CloudWorkloadSecurity" => {
            if world.api_instances.v2_api_cloud_workload_security.is_none() {
                world.api_instances.v2_api_cloud_workload_security = Some(datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI::with_config(world.config.clone()));
            }
        }
        "Synthetics" => {
            if world.api_instances.v2_api_synthetics.is_none() {
                world.api_instances.v2_api_synthetics =
                    Some(datadogV2::api::api_synthetics::SyntheticsAPI::with_config(
                        world.config.clone(),
                    ));
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
    world
        .function_mappings
        .insert("v2.ListEvents".to_string(), test_v2_list_events);
    world
        .function_mappings
        .insert("v2.SearchEvents".to_string(), test_v2_search_events);
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
    world
        .function_mappings
        .insert("v2.ListPermissions".to_string(), test_v2_list_permissions);
    world
        .function_mappings
        .insert("v2.ListFindings".to_string(), test_v2_list_findings);
    world
        .function_mappings
        .insert("v2.GetFinding".to_string(), test_v2_get_finding);
    world
        .function_mappings
        .insert("v2.UpdateFinding".to_string(), test_v2_update_finding);
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
        "v2.UploadIdPMetadata".to_string(),
        test_v2_upload_id_p_metadata,
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
    world
        .function_mappings
        .insert("v2.SubmitMetrics".to_string(), test_v2_submit_metrics);
    world.function_mappings.insert(
        "v2.GetOnDemandConcurrencyCap".to_string(),
        test_v2_get_on_demand_concurrency_cap,
    );
    world.function_mappings.insert(
        "v2.SetOnDemandConcurrencyCap".to_string(),
        test_v2_set_on_demand_concurrency_cap,
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::CreateAPIKeyParams { body };
    let response = match block_on(api.create_api_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::DeleteAPIKeyParams { key };
    let response = match block_on(api.delete_api_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::GetAPIKeyParams { key };
    let response = match block_on(api.get_api_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::UpdateAPIKeyParams { key, body };
    let response = match block_on(api.update_api_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::CreateApplicationKeyParams { body };
    let response = match block_on(api.create_application_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::DeleteApplicationKeyParams { key };
    let response = match block_on(api.delete_application_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::GetApplicationKeyParams { key };
    let response = match block_on(api.get_application_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_key_management::UpdateApplicationKeyParams { key, body };
    let response = match block_on(api.update_application_key_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_checks::SubmitServiceCheckParams { body };
    let response = match block_on(api.submit_service_check_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetDailyCustomReportsParams {
        page_size,
        page_number,
        sort_dir,
        sort,
    };
    let response = match block_on(api.get_daily_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetSpecifiedDailyCustomReportsParams { report_id };
    let response = match block_on(api.get_specified_daily_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetMonthlyCustomReportsParams {
        page_size,
        page_number,
        sort_dir,
        sort,
    };
    let response = match block_on(api.get_monthly_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetSpecifiedMonthlyCustomReportsParams { report_id };
    let response = match block_on(api.get_specified_monthly_custom_reports_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageAnalyzedLogsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_analyzed_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageAttributionParams {
        start_month,
        fields,
        end_month,
        sort_direction,
        sort_name,
        include_descendants,
        offset,
        limit,
    };
    let response = match block_on(api.get_usage_attribution_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageAuditLogsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_audit_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageLambdaParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_lambda_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageBillableSummaryParams { month };
    let response = match block_on(api.get_usage_billable_summary_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageCIAppParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_ci_app_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageCloudSecurityPostureManagementParams {
        start_hr,
        end_hr,
    };
    let response =
        match block_on(api.get_usage_cloud_security_posture_management_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageCWSParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_cws_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageDBMParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_dbm_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageFargateParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_fargate_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageHostsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_hosts_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetHourlyUsageAttributionParams {
        start_hr,
        usage_type,
        end_hr,
        next_record_id,
        tag_breakdown_keys,
        include_descendants,
    };
    let response = match block_on(api.get_hourly_usage_attribution_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetIncidentManagementParams { start_hr, end_hr };
    let response = match block_on(api.get_incident_management_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageIndexedSpansParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_indexed_spans_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetIngestedSpansParams { start_hr, end_hr };
    let response = match block_on(api.get_ingested_spans_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
        datadogV1::api::api_usage_metering::GetUsageInternetOfThingsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_internet_of_things_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageLogsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
        datadogV1::api::api_usage_metering::GetUsageLogsByRetentionParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_logs_by_retention_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageLogsByIndexParams {
        start_hr,
        end_hr,
        index_name,
    };
    let response = match block_on(api.get_usage_logs_by_index_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetMonthlyUsageAttributionParams {
        start_month,
        fields,
        end_month,
        sort_direction,
        sort_name,
        tag_breakdown_keys,
        next_record_id,
        include_descendants,
    };
    let response = match block_on(api.get_monthly_usage_attribution_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageNetworkFlowsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_network_flows_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageNetworkHostsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_network_hosts_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageOnlineArchiveParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_online_archive_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageProfilingParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_profiling_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageRumUnitsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_rum_units_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageRumSessionsParams {
        start_hr,
        end_hr,
        type_,
    };
    let response = match block_on(api.get_usage_rum_sessions_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageSDSParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_sds_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageSNMPParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_snmp_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageSummaryParams {
        start_month,
        end_month,
        include_org_details,
    };
    let response = match block_on(api.get_usage_summary_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageSyntheticsParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_synthetics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_usage_metering::GetUsageSyntheticsAPIParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_synthetics_api_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
        datadogV1::api::api_usage_metering::GetUsageSyntheticsBrowserParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_synthetics_browser_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageTimeseriesParams { start_hr, end_hr };
    let response = match block_on(api.get_usage_timeseries_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_usage_metering::GetUsageTopAvgMetricsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_dashboard_lists::CreateDashboardListParams { body };
    let response = match block_on(api.create_dashboard_list_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_dashboard_lists::DeleteDashboardListParams { list_id };
    let response = match block_on(api.delete_dashboard_list_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_dashboard_lists::GetDashboardListParams { list_id };
    let response = match block_on(api.get_dashboard_list_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_dashboard_lists::UpdateDashboardListParams { list_id, body };
    let response = match block_on(api.update_dashboard_list_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::ListDowntimesParams {
        current_only,
        with_creator,
    };
    let response = match block_on(api.list_downtimes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::CreateDowntimeParams { body };
    let response = match block_on(api.create_downtime_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::CancelDowntimesByScopeParams { body };
    let response = match block_on(api.cancel_downtimes_by_scope_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::CancelDowntimeParams { downtime_id };
    let response = match block_on(api.cancel_downtime_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::GetDowntimeParams { downtime_id };
    let response = match block_on(api.get_downtime_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::UpdateDowntimeParams { downtime_id, body };
    let response = match block_on(api.update_downtime_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_downtimes::ListMonitorDowntimesParams { monitor_id };
    let response = match block_on(api.list_monitor_downtimes_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_events::ListEventsParams {
        start,
        end,
        priority,
        sources,
        tags,
        unaggregated,
        exclude_aggregate,
        page,
    };
    let response = match block_on(api.list_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_events::CreateEventParams { body };
    let response = match block_on(api.create_event_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_events::GetEventParams { event_id };
    let response = match block_on(api.get_event_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_snapshots::GetGraphSnapshotParams {
        start,
        end,
        metric_query,
        event_query,
        graph_def,
        title,
        height,
        width,
    };
    let response = match block_on(api.get_graph_snapshot_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_hosts::MuteHostParams { host_name, body };
    let response = match block_on(api.mute_host_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_hosts::UnmuteHostParams { host_name };
    let response = match block_on(api.unmute_host_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_hosts::ListHostsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_hosts::GetHostTotalsParams { from };
    let response = match block_on(api.get_host_totals_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::DeleteAWSAccountParams { body };
    let response = match block_on(api.delete_aws_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::ListAWSAccountsParams {
        account_id,
        role_name,
        access_key_id,
    };
    let response = match block_on(api.list_aws_accounts_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::CreateAWSAccountParams { body };
    let response = match block_on(api.create_aws_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::UpdateAWSAccountParams {
        body,
        account_id,
        role_name,
        access_key_id,
    };
    let response = match block_on(api.update_aws_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::DeleteAWSEventBridgeSourceParams { body };
    let response = match block_on(api.delete_aws_event_bridge_source_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::CreateAWSEventBridgeSourceParams { body };
    let response = match block_on(api.create_aws_event_bridge_source_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::DeleteAWSTagFilterParams { body };
    let response = match block_on(api.delete_aws_tag_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::ListAWSTagFiltersParams { account_id };
    let response = match block_on(api.list_aws_tag_filters_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::CreateAWSTagFilterParams { body };
    let response = match block_on(api.create_aws_tag_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_integration::CreateNewAWSExternalIDParams { body };
    let response = match block_on(api.create_new_aws_external_id_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_logs_integration::DeleteAWSLambdaARNParams { body };
    let response = match block_on(api.delete_aws_lambda_arn_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_logs_integration::CreateAWSLambdaARNParams { body };
    let response = match block_on(api.create_aws_lambda_arn_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_logs_integration::CheckAWSLogsLambdaAsyncParams { body };
    let response = match block_on(api.check_aws_logs_lambda_async_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_logs_integration::EnableAWSLogServicesParams { body };
    let response = match block_on(api.enable_aws_log_services_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_aws_logs_integration::CheckAWSLogsServicesAsyncParams { body };
    let response = match block_on(api.check_aws_logs_services_async_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_azure_integration::DeleteAzureIntegrationParams { body };
    let response = match block_on(api.delete_azure_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_azure_integration::CreateAzureIntegrationParams { body };
    let response = match block_on(api.create_azure_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_azure_integration::UpdateAzureIntegrationParams { body };
    let response = match block_on(api.update_azure_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_azure_integration::UpdateAzureHostFiltersParams { body };
    let response = match block_on(api.update_azure_host_filters_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_gcp_integration::DeleteGCPIntegrationParams { body };
    let response = match block_on(api.delete_gcp_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_gcp_integration::CreateGCPIntegrationParams { body };
    let response = match block_on(api.create_gcp_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_gcp_integration::UpdateGCPIntegrationParams { body };
    let response = match block_on(api.update_gcp_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_pager_duty_integration::CreatePagerDutyIntegrationServiceParams {
            body,
        };
    let response = match block_on(api.create_pager_duty_integration_service_with_http_info(params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_pager_duty_integration::DeletePagerDutyIntegrationServiceParams {
            service_name,
        };
    let response = match block_on(api.delete_pager_duty_integration_service_with_http_info(params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_pager_duty_integration::GetPagerDutyIntegrationServiceParams {
        service_name,
    };
    let response = match block_on(api.get_pager_duty_integration_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_pager_duty_integration::UpdatePagerDutyIntegrationServiceParams {
            service_name,
            body,
        };
    let response = match block_on(api.update_pager_duty_integration_service_with_http_info(params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_slack_integration::GetSlackIntegrationChannelsParams { account_name };
    let response = match block_on(api.get_slack_integration_channels_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_slack_integration::CreateSlackIntegrationChannelParams {
        account_name,
        body,
    };
    let response = match block_on(api.create_slack_integration_channel_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_slack_integration::RemoveSlackIntegrationChannelParams {
        account_name,
        channel_name,
    };
    let response = match block_on(api.remove_slack_integration_channel_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_slack_integration::GetSlackIntegrationChannelParams {
        account_name,
        channel_name,
    };
    let response = match block_on(api.get_slack_integration_channel_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_slack_integration::UpdateSlackIntegrationChannelParams {
        account_name,
        channel_name,
        body,
    };
    let response = match block_on(api.update_slack_integration_channel_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::CreateWebhooksIntegrationCustomVariableParams {
            body,
        };
    let response =
        match block_on(api.create_webhooks_integration_custom_variable_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::DeleteWebhooksIntegrationCustomVariableParams {
            custom_variable_name,
        };
    let response =
        match block_on(api.delete_webhooks_integration_custom_variable_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::GetWebhooksIntegrationCustomVariableParams {
            custom_variable_name,
        };
    let response =
        match block_on(api.get_webhooks_integration_custom_variable_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::UpdateWebhooksIntegrationCustomVariableParams {
            custom_variable_name,
            body,
        };
    let response =
        match block_on(api.update_webhooks_integration_custom_variable_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_webhooks_integration::CreateWebhooksIntegrationParams { body };
    let response = match block_on(api.create_webhooks_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::DeleteWebhooksIntegrationParams { webhook_name };
    let response = match block_on(api.delete_webhooks_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_webhooks_integration::GetWebhooksIntegrationParams { webhook_name };
    let response = match block_on(api.get_webhooks_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_webhooks_integration::UpdateWebhooksIntegrationParams {
        webhook_name,
        body,
    };
    let response = match block_on(api.update_webhooks_integration_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs::ListLogsParams { body };
    let response = match block_on(api.list_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs::SubmitLogParams {
        body,
        content_encoding,
        ddtags,
    };
    let response = match block_on(api.submit_log_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs_indexes::UpdateLogsIndexOrderParams { body };
    let response = match block_on(api.update_logs_index_order_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs_indexes::CreateLogsIndexParams { body };
    let response = match block_on(api.create_logs_index_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs_indexes::GetLogsIndexParams { name };
    let response = match block_on(api.get_logs_index_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_logs_indexes::UpdateLogsIndexParams { name, body };
    let response = match block_on(api.update_logs_index_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::ListActiveMetricsParams {
        from,
        host,
        tag_filter,
    };
    let response = match block_on(api.list_active_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::GetMetricMetadataParams { metric_name };
    let response = match block_on(api.get_metric_metadata_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::UpdateMetricMetadataParams { metric_name, body };
    let response = match block_on(api.update_metric_metadata_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::QueryMetricsParams { from, to, query };
    let response = match block_on(api.query_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::ListMetricsParams { q };
    let response = match block_on(api.list_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_metrics::SubmitMetricsParams {
        body,
        content_encoding,
    };
    let response = match block_on(api.submit_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_organizations::CreateChildOrgParams { body };
    let response = match block_on(api.create_child_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_organizations::GetOrgParams { public_id };
    let response = match block_on(api.get_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_organizations::UpdateOrgParams { public_id, body };
    let response = match block_on(api.update_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_organizations::DowngradeOrgParams { public_id };
    let response = match block_on(api.downgrade_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_organizations::UploadIdPForOrgParams {
        public_id,
        idp_file,
    };
    let response = match block_on(api.upload_id_p_for_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_security_monitoring::AddSecurityMonitoringSignalToIncidentParams {
            signal_id,
            body,
        };
    let response =
        match block_on(api.add_security_monitoring_signal_to_incident_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_security_monitoring::EditSecurityMonitoringSignalAssigneeParams {
            signal_id,
            body,
        };
    let response =
        match block_on(api.edit_security_monitoring_signal_assignee_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_security_monitoring::EditSecurityMonitoringSignalStateParams {
        signal_id,
        body,
    };
    let response = match block_on(api.edit_security_monitoring_signal_state_with_http_info(params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::ListSLOsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::CreateSLOParams { body };
    let response = match block_on(api.create_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_service_level_objectives::DeleteSLOTimeframeInBulkParams { body };
    let response = match block_on(api.delete_slo_timeframe_in_bulk_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::CheckCanDeleteSLOParams { ids };
    let response = match block_on(api.check_can_delete_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::SearchSLOParams {
        query,
        page_size,
        page_number,
        include_facets,
    };
    let response = match block_on(api.search_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::DeleteSLOParams { slo_id, force };
    let response = match block_on(api.delete_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::GetSLOParams {
        slo_id,
        with_configured_alert_ids,
    };
    let response = match block_on(api.get_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::UpdateSLOParams { slo_id, body };
    let response = match block_on(api.update_slo_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::GetSLOCorrectionsParams { slo_id };
    let response = match block_on(api.get_slo_corrections_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objectives::GetSLOHistoryParams {
        slo_id,
        from_ts,
        to_ts,
        target,
        apply_correction,
    };
    let response = match block_on(api.get_slo_history_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objective_corrections::ListSLOCorrectionParams {
        offset,
        limit,
    };
    let response = match block_on(api.list_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_service_level_objective_corrections::CreateSLOCorrectionParams { body };
    let response = match block_on(api.create_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_service_level_objective_corrections::DeleteSLOCorrectionParams {
            slo_correction_id,
        };
    let response = match block_on(api.delete_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_service_level_objective_corrections::GetSLOCorrectionParams {
        slo_correction_id,
    };
    let response = match block_on(api.get_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV1::api::api_service_level_objective_corrections::UpdateSLOCorrectionParams {
            slo_correction_id,
            body,
        };
    let response = match block_on(api.update_slo_correction_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_tags::ListHostTagsParams { source };
    let response = match block_on(api.list_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_tags::DeleteHostTagsParams { host_name, source };
    let response = match block_on(api.delete_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_tags::GetHostTagsParams { host_name, source };
    let response = match block_on(api.get_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_tags::CreateHostTagsParams {
        host_name,
        body,
        source,
    };
    let response = match block_on(api.create_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_tags::UpdateHostTagsParams {
        host_name,
        body,
        source,
    };
    let response = match block_on(api.update_host_tags_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_users::CreateUserParams { body };
    let response = match block_on(api.create_user_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_users::DisableUserParams { user_handle };
    let response = match block_on(api.disable_user_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_users::GetUserParams { user_handle };
    let response = match block_on(api.get_user_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV1::api::api_users::UpdateUserParams { user_handle, body };
    let response = match block_on(api.update_user_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_spans_metrics::CreateSpansMetricParams { body };
    let response = match block_on(api.create_spans_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_spans_metrics::DeleteSpansMetricParams { metric_id };
    let response = match block_on(api.delete_spans_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_spans_metrics::GetSpansMetricParams { metric_id };
    let response = match block_on(api.get_spans_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_spans_metrics::UpdateSpansMetricParams { metric_id, body };
    let response = match block_on(api.update_spans_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_apm_retention_filters::CreateApmRetentionFilterParams { body };
    let response = match block_on(api.create_apm_retention_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_apm_retention_filters::ReorderApmRetentionFiltersParams { body };
    let response = match block_on(api.reorder_apm_retention_filters_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_apm_retention_filters::DeleteApmRetentionFilterParams { filter_id };
    let response = match block_on(api.delete_apm_retention_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_apm_retention_filters::GetApmRetentionFilterParams { filter_id };
    let response = match block_on(api.get_apm_retention_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_apm_retention_filters::UpdateApmRetentionFilterParams {
        filter_id,
        body,
    };
    let response = match block_on(api.update_apm_retention_filter_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_audit::ListAuditLogsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_audit::SearchAuditLogsParams { body };
    let response = match block_on(api.search_audit_logs_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_dashboard_lists::DeleteDashboardListItemsParams {
        dashboard_list_id,
        body,
    };
    let response = match block_on(api.delete_dashboard_list_items_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_dashboard_lists::GetDashboardListItemsParams { dashboard_list_id };
    let response = match block_on(api.get_dashboard_list_items_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_dashboard_lists::CreateDashboardListItemsParams {
        dashboard_list_id,
        body,
    };
    let response = match block_on(api.create_dashboard_list_items_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_dashboard_lists::UpdateDashboardListItemsParams {
        dashboard_list_id,
        body,
    };
    let response = match block_on(api.update_dashboard_list_items_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_events::ListEventsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_events::SearchEventsParams { body };
    let response = match block_on(api.search_events_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_gcp_integration::CreateGCPSTSAccountParams { body };
    let response = match block_on(api.create_gcpsts_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_gcp_integration::DeleteGCPSTSAccountParams { account_id };
    let response = match block_on(api.delete_gcpsts_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_gcp_integration::UpdateGCPSTSAccountParams { account_id, body };
    let response = match block_on(api.update_gcpsts_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_gcp_integration::MakeGCPSTSDelegateParams { body };
    let response = match block_on(api.make_gcpsts_delegate_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_opsgenie_integration::CreateOpsgenieServiceParams { body };
    let response = match block_on(api.create_opsgenie_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_opsgenie_integration::DeleteOpsgenieServiceParams {
        integration_service_id,
    };
    let response = match block_on(api.delete_opsgenie_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_opsgenie_integration::GetOpsgenieServiceParams {
        integration_service_id,
    };
    let response = match block_on(api.get_opsgenie_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_opsgenie_integration::UpdateOpsgenieServiceParams {
        integration_service_id,
        body,
    };
    let response = match block_on(api.update_opsgenie_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_cloudflare_integration::CreateCloudflareAccountParams { body };
    let response = match block_on(api.create_cloudflare_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloudflare_integration::DeleteCloudflareAccountParams { account_id };
    let response = match block_on(api.delete_cloudflare_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloudflare_integration::GetCloudflareAccountParams { account_id };
    let response = match block_on(api.get_cloudflare_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_cloudflare_integration::UpdateCloudflareAccountParams {
        account_id,
        body,
    };
    let response = match block_on(api.update_cloudflare_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::CreateConfluentAccountParams { body };
    let response = match block_on(api.create_confluent_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::DeleteConfluentAccountParams { account_id };
    let response = match block_on(api.delete_confluent_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::GetConfluentAccountParams { account_id };
    let response = match block_on(api.get_confluent_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_confluent_cloud::UpdateConfluentAccountParams { account_id, body };
    let response = match block_on(api.update_confluent_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::ListConfluentResourceParams { account_id };
    let response = match block_on(api.list_confluent_resource_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_confluent_cloud::CreateConfluentResourceParams { account_id, body };
    let response = match block_on(api.create_confluent_resource_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::DeleteConfluentResourceParams {
        account_id,
        resource_id,
    };
    let response = match block_on(api.delete_confluent_resource_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::GetConfluentResourceParams {
        account_id,
        resource_id,
    };
    let response = match block_on(api.get_confluent_resource_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_confluent_cloud::UpdateConfluentResourceParams {
        account_id,
        resource_id,
        body,
    };
    let response = match block_on(api.update_confluent_resource_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::CreateFastlyAccountParams { body };
    let response = match block_on(api.create_fastly_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::DeleteFastlyAccountParams { account_id };
    let response = match block_on(api.delete_fastly_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::GetFastlyAccountParams { account_id };
    let response = match block_on(api.get_fastly_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_fastly_integration::UpdateFastlyAccountParams { account_id, body };
    let response = match block_on(api.update_fastly_account_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::ListFastlyServicesParams { account_id };
    let response = match block_on(api.list_fastly_services_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_fastly_integration::CreateFastlyServiceParams { account_id, body };
    let response = match block_on(api.create_fastly_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::DeleteFastlyServiceParams {
        account_id,
        service_id,
    };
    let response = match block_on(api.delete_fastly_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::GetFastlyServiceParams {
        account_id,
        service_id,
    };
    let response = match block_on(api.get_fastly_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_fastly_integration::UpdateFastlyServiceParams {
        account_id,
        service_id,
        body,
    };
    let response = match block_on(api.update_fastly_service_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_ip_allowlist::UpdateIPAllowlistParams { body };
    let response = match block_on(api.update_ip_allowlist_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_logs::SubmitLogParams {
        body,
        content_encoding,
        ddtags,
    };
    let response = match block_on(api.submit_log_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_logs_metrics::CreateLogsMetricParams { body };
    let response = match block_on(api.create_logs_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_logs_metrics::DeleteLogsMetricParams { metric_id };
    let response = match block_on(api.delete_logs_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_logs_metrics::GetLogsMetricParams { metric_id };
    let response = match block_on(api.get_logs_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_logs_metrics::UpdateLogsMetricParams { metric_id, body };
    let response = match block_on(api.update_logs_metric_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_security_monitoring::ListFindingsParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_security_monitoring::GetFindingParams {
        finding_id,
        snapshot_timestamp,
    };
    let response = match block_on(api.get_finding_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn test_v2_update_finding(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .v2_api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let finding_id =
        serde_json::from_value(_parameters.get("finding_id").unwrap().clone()).unwrap();
    let body = serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap();
    let params = datadogV2::api::api_security_monitoring::UpdateFindingParams { finding_id, body };
    let response = match block_on(api.update_finding_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_processes::ListProcessesParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_restriction_policies::DeleteRestrictionPolicyParams { resource_id };
    let response = match block_on(api.delete_restriction_policy_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_restriction_policies::GetRestrictionPolicyParams { resource_id };
    let response = match block_on(api.get_restriction_policy_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_restriction_policies::UpdateRestrictionPolicyParams {
        resource_id,
        body,
    };
    let response = match block_on(api.update_restriction_policy_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_organizations::UploadIdPMetadataParams { idp_file };
    let response = match block_on(api.upload_id_p_metadata_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloud_workload_security::CreateCloudWorkloadSecurityAgentRuleParams {
            body,
        };
    let response =
        match block_on(api.create_cloud_workload_security_agent_rule_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloud_workload_security::DeleteCloudWorkloadSecurityAgentRuleParams {
            agent_rule_id,
        };
    let response =
        match block_on(api.delete_cloud_workload_security_agent_rule_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloud_workload_security::GetCloudWorkloadSecurityAgentRuleParams {
            agent_rule_id,
        };
    let response = match block_on(api.get_cloud_workload_security_agent_rule_with_http_info(params))
    {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params =
        datadogV2::api::api_cloud_workload_security::UpdateCloudWorkloadSecurityAgentRuleParams {
            agent_rule_id,
            body,
        };
    let response =
        match block_on(api.update_cloud_workload_security_agent_rule_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_metrics::SubmitMetricsParams {
        body,
        content_encoding,
    };
    let response = match block_on(api.submit_metrics_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_synthetics::SetOnDemandConcurrencyCapParams { body };
    let response = match block_on(api.set_on_demand_concurrency_cap_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetUsageApplicationSecurityMonitoringParams {
        start_hr,
        end_hr,
    };
    let response =
        match block_on(api.get_usage_application_security_monitoring_with_http_info(params)) {
            Ok(response) => response,
            Err(error) => {
                return match error {
                    Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetCostByOrgParams {
        start_month,
        end_month,
    };
    let response = match block_on(api.get_cost_by_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetEstimatedCostByOrgParams {
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
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetHistoricalCostByOrgParams {
        start_month,
        view,
        end_month,
    };
    let response = match block_on(api.get_historical_cost_by_org_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetHourlyUsageParams {
        filter_timestamp_start,
        filter_product_families,
        filter_timestamp_end,
        filter_include_descendants,
        filter_include_breakdown,
        filter_versions,
        page_limit,
        page_next_record_id,
    };
    let response = match block_on(api.get_hourly_usage_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetUsageLambdaTracedInvocationsParams {
        start_hr,
        end_hr,
    };
    let response = match block_on(api.get_usage_lambda_traced_invocations_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
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
    let params = datadogV2::api::api_usage_metering::GetUsageObservabilityPipelinesParams {
        start_hr,
        end_hr,
    };
    let response = match block_on(api.get_usage_observability_pipelines_with_http_info(params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
                _ => panic!("error parsing response: {}", error),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}
