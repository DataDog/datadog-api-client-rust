use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;

use datadog_api_client::datadogV1::api::api_authentication::*;
use datadog_api_client::datadogV1::api::api_aws_integration::*;
use datadog_api_client::datadogV1::api::api_aws_logs_integration::*;
use datadog_api_client::datadogV1::api::api_azure_integration::*;
use datadog_api_client::datadogV1::api::api_dashboard_lists::*;
use datadog_api_client::datadogV1::api::api_downtimes::*;
use datadog_api_client::datadogV1::api::api_events::*;
use datadog_api_client::datadogV1::api::api_gcp_integration::*;
use datadog_api_client::datadogV1::api::api_hosts::*;
use datadog_api_client::datadogV1::api::api_ip_ranges::*;
use datadog_api_client::datadogV1::api::api_key_management::*;
use datadog_api_client::datadogV1::api::api_logs::*;
use datadog_api_client::datadogV1::api::api_logs_indexes::*;
use datadog_api_client::datadogV1::api::api_metrics::*;
use datadog_api_client::datadogV1::api::api_organizations::*;
use datadog_api_client::datadogV1::api::api_pager_duty_integration::*;
use datadog_api_client::datadogV1::api::api_security_monitoring::*;
use datadog_api_client::datadogV1::api::api_service_checks::*;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::*;
use datadog_api_client::datadogV1::api::api_service_level_objectives::*;
use datadog_api_client::datadogV1::api::api_slack_integration::*;
use datadog_api_client::datadogV1::api::api_snapshots::*;
use datadog_api_client::datadogV1::api::api_tags::*;
use datadog_api_client::datadogV1::api::api_usage_metering::*;
use datadog_api_client::datadogV1::api::api_users::*;
use datadog_api_client::datadogV1::api::api_webhooks_integration::*;
use datadog_api_client::datadogV2::api::api_apm_retention_filters::*;
use datadog_api_client::datadogV2::api::api_audit::*;
use datadog_api_client::datadogV2::api::api_cloud_workload_security::*;
use datadog_api_client::datadogV2::api::api_cloudflare_integration::*;
use datadog_api_client::datadogV2::api::api_confluent_cloud::*;
use datadog_api_client::datadogV2::api::api_dashboard_lists::*;
use datadog_api_client::datadogV2::api::api_events::*;
use datadog_api_client::datadogV2::api::api_fastly_integration::*;
use datadog_api_client::datadogV2::api::api_gcp_integration::*;
use datadog_api_client::datadogV2::api::api_ip_allowlist::*;
use datadog_api_client::datadogV2::api::api_logs::*;
use datadog_api_client::datadogV2::api::api_logs_metrics::*;
use datadog_api_client::datadogV2::api::api_metrics::*;
use datadog_api_client::datadogV2::api::api_opsgenie_integration::*;
use datadog_api_client::datadogV2::api::api_organizations::*;
use datadog_api_client::datadogV2::api::api_processes::*;
use datadog_api_client::datadogV2::api::api_restriction_policies::*;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::api::api_spans_metrics::*;
use datadog_api_client::datadogV2::api::api_synthetics::*;
use datadog_api_client::datadogV2::api::api_usage_metering::*;

#[derive(Debug, Default)]
pub struct ApiInstances {
    pub api_ip_ranges: Option<IpRangesAPI>,
    pub api_key_management: Option<KeyManagementAPI>,
    pub api_service_checks: Option<ServiceChecksAPI>,
    pub api_usage_metering: Option<UsageMeteringAPI>,
    pub api_dashboard_lists: Option<DashboardListsAPI>,
    pub api_downtimes: Option<DowntimesAPI>,
    pub api_events: Option<EventsAPI>,
    pub api_snapshots: Option<SnapshotsAPI>,
    pub api_hosts: Option<HostsAPI>,
    pub api_aws_integration: Option<AwsIntegrationAPI>,
    pub api_aws_logs_integration: Option<AwsLogsIntegrationAPI>,
    pub api_azure_integration: Option<AzureIntegrationAPI>,
    pub api_gcp_integration: Option<GcpIntegrationAPI>,
    pub api_pager_duty_integration: Option<PagerDutyIntegrationAPI>,
    pub api_slack_integration: Option<SlackIntegrationAPI>,
    pub api_webhooks_integration: Option<WebhooksIntegrationAPI>,
    pub api_logs: Option<LogsAPI>,
    pub api_logs_indexes: Option<LogsIndexesAPI>,
    pub api_metrics: Option<MetricsAPI>,
    pub api_organizations: Option<OrganizationsAPI>,
    pub api_security_monitoring: Option<SecurityMonitoringAPI>,
    pub api_service_level_objectives: Option<ServiceLevelObjectivesAPI>,
    pub api_service_level_objective_corrections: Option<ServiceLevelObjectiveCorrectionsAPI>,
    pub api_tags: Option<TagsAPI>,
    pub api_users: Option<UsersAPI>,
    pub api_authentication: Option<AuthenticationAPI>,
    pub api_spans_metrics: Option<SpansMetricsAPI>,
    pub api_apm_retention_filters: Option<ApmRetentionFiltersAPI>,
    pub api_audit: Option<AuditAPI>,
    pub api_dashboard_lists: Option<DashboardListsAPI>,
    pub api_events: Option<EventsAPI>,
    pub api_gcp_integration: Option<GcpIntegrationAPI>,
    pub api_opsgenie_integration: Option<OpsgenieIntegrationAPI>,
    pub api_cloudflare_integration: Option<CloudflareIntegrationAPI>,
    pub api_confluent_cloud: Option<ConfluentCloudAPI>,
    pub api_fastly_integration: Option<FastlyIntegrationAPI>,
    pub api_ip_allowlist: Option<IpAllowlistAPI>,
    pub api_logs: Option<LogsAPI>,
    pub api_logs_metrics: Option<LogsMetricsAPI>,
    pub api_roles: Option<RolesAPI>,
    pub api_security_monitoring: Option<SecurityMonitoringAPI>,
    pub api_processes: Option<ProcessesAPI>,
    pub api_restriction_policies: Option<RestrictionPoliciesAPI>,
    pub api_organizations: Option<OrganizationsAPI>,
    pub api_cloud_workload_security: Option<CloudWorkloadSecurityAPI>,
    pub api_metrics: Option<MetricsAPI>,
    pub api_synthetics: Option<SyntheticsAPI>,
    pub api_usage_metering: Option<UsageMeteringAPI>,
}

pub fn initialize_api_instance(world: &mut DatadogWorld, api: String) {
    match api.as_str() {
        "IpRanges" => {
            if world.api_instances.api_ip_ranges.is_none() {
                world.api_instances.api_ip_ranges = Some(IpRangesAPI::with_config(world.config.clone()));
            }
        }
        "KeyManagement" => {
            if world.api_instances.api_key_management.is_none() {
                world.api_instances.api_key_management = Some(KeyManagementAPI::with_config(world.config.clone()));
            }
        }
        "ServiceChecks" => {
            if world.api_instances.api_service_checks.is_none() {
                world.api_instances.api_service_checks = Some(ServiceChecksAPI::with_config(world.config.clone()));
            }
        }
        "UsageMetering" => {
            if world.api_instances.api_usage_metering.is_none() {
                world.api_instances.api_usage_metering = Some(UsageMeteringAPI::with_config(world.config.clone()));
            }
        }
        "DashboardLists" => {
            if world.api_instances.api_dashboard_lists.is_none() {
                world.api_instances.api_dashboard_lists = Some(DashboardListsAPI::with_config(world.config.clone()));
            }
        }
        "Downtimes" => {
            if world.api_instances.api_downtimes.is_none() {
                world.api_instances.api_downtimes = Some(DowntimesAPI::with_config(world.config.clone()));
            }
        }
        "Events" => {
            if world.api_instances.api_events.is_none() {
                world.api_instances.api_events = Some(EventsAPI::with_config(world.config.clone()));
            }
        }
        "Snapshots" => {
            if world.api_instances.api_snapshots.is_none() {
                world.api_instances.api_snapshots = Some(SnapshotsAPI::with_config(world.config.clone()));
            }
        }
        "Hosts" => {
            if world.api_instances.api_hosts.is_none() {
                world.api_instances.api_hosts = Some(HostsAPI::with_config(world.config.clone()));
            }
        }
        "AwsIntegration" => {
            if world.api_instances.api_aws_integration.is_none() {
                world.api_instances.api_aws_integration = Some(AwsIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "AwsLogsIntegration" => {
            if world.api_instances.api_aws_logs_integration.is_none() {
                world.api_instances.api_aws_logs_integration =
                    Some(AwsLogsIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "AzureIntegration" => {
            if world.api_instances.api_azure_integration.is_none() {
                world.api_instances.api_azure_integration =
                    Some(AzureIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "GcpIntegration" => {
            if world.api_instances.api_gcp_integration.is_none() {
                world.api_instances.api_gcp_integration = Some(GcpIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "PagerDutyIntegration" => {
            if world.api_instances.api_pager_duty_integration.is_none() {
                world.api_instances.api_pager_duty_integration =
                    Some(PagerDutyIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "SlackIntegration" => {
            if world.api_instances.api_slack_integration.is_none() {
                world.api_instances.api_slack_integration =
                    Some(SlackIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "WebhooksIntegration" => {
            if world.api_instances.api_webhooks_integration.is_none() {
                world.api_instances.api_webhooks_integration =
                    Some(WebhooksIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "Logs" => {
            if world.api_instances.api_logs.is_none() {
                world.api_instances.api_logs = Some(LogsAPI::with_config(world.config.clone()));
            }
        }
        "LogsIndexes" => {
            if world.api_instances.api_logs_indexes.is_none() {
                world.api_instances.api_logs_indexes = Some(LogsIndexesAPI::with_config(world.config.clone()));
            }
        }
        "Metrics" => {
            if world.api_instances.api_metrics.is_none() {
                world.api_instances.api_metrics = Some(MetricsAPI::with_config(world.config.clone()));
            }
        }
        "Organizations" => {
            if world.api_instances.api_organizations.is_none() {
                world.api_instances.api_organizations = Some(OrganizationsAPI::with_config(world.config.clone()));
            }
        }
        "SecurityMonitoring" => {
            if world.api_instances.api_security_monitoring.is_none() {
                world.api_instances.api_security_monitoring =
                    Some(SecurityMonitoringAPI::with_config(world.config.clone()));
            }
        }
        "ServiceLevelObjectives" => {
            if world.api_instances.api_service_level_objectives.is_none() {
                world.api_instances.api_service_level_objectives =
                    Some(ServiceLevelObjectivesAPI::with_config(world.config.clone()));
            }
        }
        "ServiceLevelObjectiveCorrections" => {
            if world.api_instances.api_service_level_objective_corrections.is_none() {
                world.api_instances.api_service_level_objective_corrections =
                    Some(ServiceLevelObjectiveCorrectionsAPI::with_config(world.config.clone()));
            }
        }
        "Tags" => {
            if world.api_instances.api_tags.is_none() {
                world.api_instances.api_tags = Some(TagsAPI::with_config(world.config.clone()));
            }
        }
        "Users" => {
            if world.api_instances.api_users.is_none() {
                world.api_instances.api_users = Some(UsersAPI::with_config(world.config.clone()));
            }
        }
        "Authentication" => {
            if world.api_instances.api_authentication.is_none() {
                world.api_instances.api_authentication = Some(AuthenticationAPI::with_config(world.config.clone()));
            }
        }
        "SpansMetrics" => {
            if world.api_instances.api_spans_metrics.is_none() {
                world.api_instances.api_spans_metrics = Some(SpansMetricsAPI::with_config(world.config.clone()));
            }
        }
        "ApmRetentionFilters" => {
            if world.api_instances.api_apm_retention_filters.is_none() {
                world.api_instances.api_apm_retention_filters =
                    Some(ApmRetentionFiltersAPI::with_config(world.config.clone()));
            }
        }
        "Audit" => {
            if world.api_instances.api_audit.is_none() {
                world.api_instances.api_audit = Some(AuditAPI::with_config(world.config.clone()));
            }
        }
        "DashboardLists" => {
            if world.api_instances.api_dashboard_lists.is_none() {
                world.api_instances.api_dashboard_lists = Some(DashboardListsAPI::with_config(world.config.clone()));
            }
        }
        "Events" => {
            if world.api_instances.api_events.is_none() {
                world.api_instances.api_events = Some(EventsAPI::with_config(world.config.clone()));
            }
        }
        "GcpIntegration" => {
            if world.api_instances.api_gcp_integration.is_none() {
                world.api_instances.api_gcp_integration = Some(GcpIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "OpsgenieIntegration" => {
            if world.api_instances.api_opsgenie_integration.is_none() {
                world.api_instances.api_opsgenie_integration =
                    Some(OpsgenieIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "CloudflareIntegration" => {
            if world.api_instances.api_cloudflare_integration.is_none() {
                world.api_instances.api_cloudflare_integration =
                    Some(CloudflareIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "ConfluentCloud" => {
            if world.api_instances.api_confluent_cloud.is_none() {
                world.api_instances.api_confluent_cloud = Some(ConfluentCloudAPI::with_config(world.config.clone()));
            }
        }
        "FastlyIntegration" => {
            if world.api_instances.api_fastly_integration.is_none() {
                world.api_instances.api_fastly_integration =
                    Some(FastlyIntegrationAPI::with_config(world.config.clone()));
            }
        }
        "IpAllowlist" => {
            if world.api_instances.api_ip_allowlist.is_none() {
                world.api_instances.api_ip_allowlist = Some(IpAllowlistAPI::with_config(world.config.clone()));
            }
        }
        "Logs" => {
            if world.api_instances.api_logs.is_none() {
                world.api_instances.api_logs = Some(LogsAPI::with_config(world.config.clone()));
            }
        }
        "LogsMetrics" => {
            if world.api_instances.api_logs_metrics.is_none() {
                world.api_instances.api_logs_metrics = Some(LogsMetricsAPI::with_config(world.config.clone()));
            }
        }
        "Roles" => {
            if world.api_instances.api_roles.is_none() {
                world.api_instances.api_roles = Some(RolesAPI::with_config(world.config.clone()));
            }
        }
        "SecurityMonitoring" => {
            if world.api_instances.api_security_monitoring.is_none() {
                world.api_instances.api_security_monitoring =
                    Some(SecurityMonitoringAPI::with_config(world.config.clone()));
            }
        }
        "Processes" => {
            if world.api_instances.api_processes.is_none() {
                world.api_instances.api_processes = Some(ProcessesAPI::with_config(world.config.clone()));
            }
        }
        "RestrictionPolicies" => {
            if world.api_instances.api_restriction_policies.is_none() {
                world.api_instances.api_restriction_policies =
                    Some(RestrictionPoliciesAPI::with_config(world.config.clone()));
            }
        }
        "Organizations" => {
            if world.api_instances.api_organizations.is_none() {
                world.api_instances.api_organizations = Some(OrganizationsAPI::with_config(world.config.clone()));
            }
        }
        "CloudWorkloadSecurity" => {
            if world.api_instances.api_cloud_workload_security.is_none() {
                world.api_instances.api_cloud_workload_security =
                    Some(CloudWorkloadSecurityAPI::with_config(world.config.clone()));
            }
        }
        "Metrics" => {
            if world.api_instances.api_metrics.is_none() {
                world.api_instances.api_metrics = Some(MetricsAPI::with_config(world.config.clone()));
            }
        }
        "Synthetics" => {
            if world.api_instances.api_synthetics.is_none() {
                world.api_instances.api_synthetics = Some(SyntheticsAPI::with_config(world.config.clone()));
            }
        }
        "UsageMetering" => {
            if world.api_instances.api_usage_metering.is_none() {
                world.api_instances.api_usage_metering = Some(UsageMeteringAPI::with_config(world.config.clone()));
            }
        }
        _ => panic!("{api} API instance not found"),
    }
}

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("GetIPRanges".to_string(), test_get_ip_ranges);
    world
        .function_mappings
        .insert("ListAPIKeys".to_string(), test_list_api_keys);
    world
        .function_mappings
        .insert("CreateAPIKey".to_string(), test_create_api_key);
    world
        .function_mappings
        .insert("DeleteAPIKey".to_string(), test_delete_api_key);
    world
        .function_mappings
        .insert("GetAPIKey".to_string(), test_get_api_key);
    world
        .function_mappings
        .insert("UpdateAPIKey".to_string(), test_update_api_key);
    world
        .function_mappings
        .insert("ListApplicationKeys".to_string(), test_list_application_keys);
    world
        .function_mappings
        .insert("CreateApplicationKey".to_string(), test_create_application_key);
    world
        .function_mappings
        .insert("DeleteApplicationKey".to_string(), test_delete_application_key);
    world
        .function_mappings
        .insert("GetApplicationKey".to_string(), test_get_application_key);
    world
        .function_mappings
        .insert("UpdateApplicationKey".to_string(), test_update_application_key);
    world
        .function_mappings
        .insert("SubmitServiceCheck".to_string(), test_submit_service_check);
    world
        .function_mappings
        .insert("GetDailyCustomReports".to_string(), test_get_daily_custom_reports);
    world.function_mappings.insert(
        "GetSpecifiedDailyCustomReports".to_string(),
        test_get_specified_daily_custom_reports,
    );
    world
        .function_mappings
        .insert("GetMonthlyCustomReports".to_string(), test_get_monthly_custom_reports);
    world.function_mappings.insert(
        "GetSpecifiedMonthlyCustomReports".to_string(),
        test_get_specified_monthly_custom_reports,
    );
    world
        .function_mappings
        .insert("GetUsageAnalyzedLogs".to_string(), test_get_usage_analyzed_logs);
    world
        .function_mappings
        .insert("GetUsageAttribution".to_string(), test_get_usage_attribution);
    world
        .function_mappings
        .insert("GetUsageAuditLogs".to_string(), test_get_usage_audit_logs);
    world
        .function_mappings
        .insert("GetUsageLambda".to_string(), test_get_usage_lambda);
    world
        .function_mappings
        .insert("GetUsageBillableSummary".to_string(), test_get_usage_billable_summary);
    world
        .function_mappings
        .insert("GetUsageCIApp".to_string(), test_get_usage_ci_app);
    world.function_mappings.insert(
        "GetUsageCloudSecurityPostureManagement".to_string(),
        test_get_usage_cloud_security_posture_management,
    );
    world
        .function_mappings
        .insert("GetUsageCWS".to_string(), test_get_usage_cws);
    world
        .function_mappings
        .insert("GetUsageDBM".to_string(), test_get_usage_dbm);
    world
        .function_mappings
        .insert("GetUsageFargate".to_string(), test_get_usage_fargate);
    world
        .function_mappings
        .insert("GetUsageHosts".to_string(), test_get_usage_hosts);
    world.function_mappings.insert(
        "GetHourlyUsageAttribution".to_string(),
        test_get_hourly_usage_attribution,
    );
    world
        .function_mappings
        .insert("GetIncidentManagement".to_string(), test_get_incident_management);
    world
        .function_mappings
        .insert("GetUsageIndexedSpans".to_string(), test_get_usage_indexed_spans);
    world
        .function_mappings
        .insert("GetIngestedSpans".to_string(), test_get_ingested_spans);
    world.function_mappings.insert(
        "GetUsageInternetOfThings".to_string(),
        test_get_usage_internet_of_things,
    );
    world
        .function_mappings
        .insert("GetUsageLogs".to_string(), test_get_usage_logs);
    world
        .function_mappings
        .insert("GetUsageLogsByRetention".to_string(), test_get_usage_logs_by_retention);
    world
        .function_mappings
        .insert("GetUsageLogsByIndex".to_string(), test_get_usage_logs_by_index);
    world.function_mappings.insert(
        "GetMonthlyUsageAttribution".to_string(),
        test_get_monthly_usage_attribution,
    );
    world
        .function_mappings
        .insert("GetUsageNetworkFlows".to_string(), test_get_usage_network_flows);
    world
        .function_mappings
        .insert("GetUsageNetworkHosts".to_string(), test_get_usage_network_hosts);
    world
        .function_mappings
        .insert("GetUsageOnlineArchive".to_string(), test_get_usage_online_archive);
    world
        .function_mappings
        .insert("GetUsageProfiling".to_string(), test_get_usage_profiling);
    world
        .function_mappings
        .insert("GetUsageRumUnits".to_string(), test_get_usage_rum_units);
    world
        .function_mappings
        .insert("GetUsageRumSessions".to_string(), test_get_usage_rum_sessions);
    world
        .function_mappings
        .insert("GetUsageSDS".to_string(), test_get_usage_sds);
    world
        .function_mappings
        .insert("GetUsageSNMP".to_string(), test_get_usage_snmp);
    world
        .function_mappings
        .insert("GetUsageSummary".to_string(), test_get_usage_summary);
    world
        .function_mappings
        .insert("GetUsageSynthetics".to_string(), test_get_usage_synthetics);
    world
        .function_mappings
        .insert("GetUsageSyntheticsAPI".to_string(), test_get_usage_synthetics_api);
    world.function_mappings.insert(
        "GetUsageSyntheticsBrowser".to_string(),
        test_get_usage_synthetics_browser,
    );
    world
        .function_mappings
        .insert("GetUsageTimeseries".to_string(), test_get_usage_timeseries);
    world
        .function_mappings
        .insert("GetUsageTopAvgMetrics".to_string(), test_get_usage_top_avg_metrics);
    world
        .function_mappings
        .insert("ListDashboardLists".to_string(), test_list_dashboard_lists);
    world
        .function_mappings
        .insert("CreateDashboardList".to_string(), test_create_dashboard_list);
    world
        .function_mappings
        .insert("DeleteDashboardList".to_string(), test_delete_dashboard_list);
    world
        .function_mappings
        .insert("GetDashboardList".to_string(), test_get_dashboard_list);
    world
        .function_mappings
        .insert("UpdateDashboardList".to_string(), test_update_dashboard_list);
    world
        .function_mappings
        .insert("ListDowntimes".to_string(), test_list_downtimes);
    world
        .function_mappings
        .insert("CreateDowntime".to_string(), test_create_downtime);
    world
        .function_mappings
        .insert("CancelDowntimesByScope".to_string(), test_cancel_downtimes_by_scope);
    world
        .function_mappings
        .insert("CancelDowntime".to_string(), test_cancel_downtime);
    world
        .function_mappings
        .insert("GetDowntime".to_string(), test_get_downtime);
    world
        .function_mappings
        .insert("UpdateDowntime".to_string(), test_update_downtime);
    world
        .function_mappings
        .insert("ListMonitorDowntimes".to_string(), test_list_monitor_downtimes);
    world
        .function_mappings
        .insert("ListEvents".to_string(), test_list_events);
    world
        .function_mappings
        .insert("CreateEvent".to_string(), test_create_event);
    world.function_mappings.insert("GetEvent".to_string(), test_get_event);
    world
        .function_mappings
        .insert("GetGraphSnapshot".to_string(), test_get_graph_snapshot);
    world.function_mappings.insert("MuteHost".to_string(), test_mute_host);
    world
        .function_mappings
        .insert("UnmuteHost".to_string(), test_unmute_host);
    world.function_mappings.insert("ListHosts".to_string(), test_list_hosts);
    world
        .function_mappings
        .insert("GetHostTotals".to_string(), test_get_host_totals);
    world
        .function_mappings
        .insert("DeleteAWSAccount".to_string(), test_delete_aws_account);
    world
        .function_mappings
        .insert("ListAWSAccounts".to_string(), test_list_aws_accounts);
    world
        .function_mappings
        .insert("CreateAWSAccount".to_string(), test_create_aws_account);
    world
        .function_mappings
        .insert("UpdateAWSAccount".to_string(), test_update_aws_account);
    world.function_mappings.insert(
        "ListAvailableAWSNamespaces".to_string(),
        test_list_available_aws_namespaces,
    );
    world.function_mappings.insert(
        "DeleteAWSEventBridgeSource".to_string(),
        test_delete_aws_event_bridge_source,
    );
    world.function_mappings.insert(
        "ListAWSEventBridgeSources".to_string(),
        test_list_aws_event_bridge_sources,
    );
    world.function_mappings.insert(
        "CreateAWSEventBridgeSource".to_string(),
        test_create_aws_event_bridge_source,
    );
    world
        .function_mappings
        .insert("DeleteAWSTagFilter".to_string(), test_delete_aws_tag_filter);
    world
        .function_mappings
        .insert("ListAWSTagFilters".to_string(), test_list_aws_tag_filters);
    world
        .function_mappings
        .insert("CreateAWSTagFilter".to_string(), test_create_aws_tag_filter);
    world
        .function_mappings
        .insert("CreateNewAWSExternalID".to_string(), test_create_new_aws_external_id);
    world
        .function_mappings
        .insert("DeleteAWSLambdaARN".to_string(), test_delete_aws_lambda_arn);
    world
        .function_mappings
        .insert("ListAWSLogsIntegrations".to_string(), test_list_aws_logs_integrations);
    world
        .function_mappings
        .insert("CreateAWSLambdaARN".to_string(), test_create_aws_lambda_arn);
    world
        .function_mappings
        .insert("CheckAWSLogsLambdaAsync".to_string(), test_check_aws_logs_lambda_async);
    world
        .function_mappings
        .insert("ListAWSLogsServices".to_string(), test_list_aws_logs_services);
    world
        .function_mappings
        .insert("EnableAWSLogServices".to_string(), test_enable_aws_log_services);
    world.function_mappings.insert(
        "CheckAWSLogsServicesAsync".to_string(),
        test_check_aws_logs_services_async,
    );
    world
        .function_mappings
        .insert("DeleteAzureIntegration".to_string(), test_delete_azure_integration);
    world
        .function_mappings
        .insert("ListAzureIntegration".to_string(), test_list_azure_integration);
    world
        .function_mappings
        .insert("CreateAzureIntegration".to_string(), test_create_azure_integration);
    world
        .function_mappings
        .insert("UpdateAzureIntegration".to_string(), test_update_azure_integration);
    world
        .function_mappings
        .insert("UpdateAzureHostFilters".to_string(), test_update_azure_host_filters);
    world
        .function_mappings
        .insert("DeleteGCPIntegration".to_string(), test_delete_gcp_integration);
    world
        .function_mappings
        .insert("ListGCPIntegration".to_string(), test_list_gcp_integration);
    world
        .function_mappings
        .insert("CreateGCPIntegration".to_string(), test_create_gcp_integration);
    world
        .function_mappings
        .insert("UpdateGCPIntegration".to_string(), test_update_gcp_integration);
    world.function_mappings.insert(
        "CreatePagerDutyIntegrationService".to_string(),
        test_create_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "DeletePagerDutyIntegrationService".to_string(),
        test_delete_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "GetPagerDutyIntegrationService".to_string(),
        test_get_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "UpdatePagerDutyIntegrationService".to_string(),
        test_update_pager_duty_integration_service,
    );
    world.function_mappings.insert(
        "GetSlackIntegrationChannels".to_string(),
        test_get_slack_integration_channels,
    );
    world.function_mappings.insert(
        "CreateSlackIntegrationChannel".to_string(),
        test_create_slack_integration_channel,
    );
    world.function_mappings.insert(
        "RemoveSlackIntegrationChannel".to_string(),
        test_remove_slack_integration_channel,
    );
    world.function_mappings.insert(
        "GetSlackIntegrationChannel".to_string(),
        test_get_slack_integration_channel,
    );
    world.function_mappings.insert(
        "UpdateSlackIntegrationChannel".to_string(),
        test_update_slack_integration_channel,
    );
    world.function_mappings.insert(
        "CreateWebhooksIntegrationCustomVariable".to_string(),
        test_create_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "DeleteWebhooksIntegrationCustomVariable".to_string(),
        test_delete_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "GetWebhooksIntegrationCustomVariable".to_string(),
        test_get_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "UpdateWebhooksIntegrationCustomVariable".to_string(),
        test_update_webhooks_integration_custom_variable,
    );
    world.function_mappings.insert(
        "CreateWebhooksIntegration".to_string(),
        test_create_webhooks_integration,
    );
    world.function_mappings.insert(
        "DeleteWebhooksIntegration".to_string(),
        test_delete_webhooks_integration,
    );
    world
        .function_mappings
        .insert("GetWebhooksIntegration".to_string(), test_get_webhooks_integration);
    world.function_mappings.insert(
        "UpdateWebhooksIntegration".to_string(),
        test_update_webhooks_integration,
    );
    world.function_mappings.insert("ListLogs".to_string(), test_list_logs);
    world.function_mappings.insert("SubmitLog".to_string(), test_submit_log);
    world
        .function_mappings
        .insert("GetLogsIndexOrder".to_string(), test_get_logs_index_order);
    world
        .function_mappings
        .insert("UpdateLogsIndexOrder".to_string(), test_update_logs_index_order);
    world
        .function_mappings
        .insert("ListLogIndexes".to_string(), test_list_log_indexes);
    world
        .function_mappings
        .insert("CreateLogsIndex".to_string(), test_create_logs_index);
    world
        .function_mappings
        .insert("GetLogsIndex".to_string(), test_get_logs_index);
    world
        .function_mappings
        .insert("UpdateLogsIndex".to_string(), test_update_logs_index);
    world
        .function_mappings
        .insert("ListActiveMetrics".to_string(), test_list_active_metrics);
    world
        .function_mappings
        .insert("GetMetricMetadata".to_string(), test_get_metric_metadata);
    world
        .function_mappings
        .insert("UpdateMetricMetadata".to_string(), test_update_metric_metadata);
    world
        .function_mappings
        .insert("QueryMetrics".to_string(), test_query_metrics);
    world
        .function_mappings
        .insert("ListMetrics".to_string(), test_list_metrics);
    world
        .function_mappings
        .insert("SubmitMetrics".to_string(), test_submit_metrics);
    world.function_mappings.insert("ListOrgs".to_string(), test_list_orgs);
    world
        .function_mappings
        .insert("CreateChildOrg".to_string(), test_create_child_org);
    world.function_mappings.insert("GetOrg".to_string(), test_get_org);
    world.function_mappings.insert("UpdateOrg".to_string(), test_update_org);
    world
        .function_mappings
        .insert("DowngradeOrg".to_string(), test_downgrade_org);
    world
        .function_mappings
        .insert("UploadIdPForOrg".to_string(), test_upload_id_p_for_org);
    world.function_mappings.insert(
        "AddSecurityMonitoringSignalToIncident".to_string(),
        test_add_security_monitoring_signal_to_incident,
    );
    world.function_mappings.insert(
        "EditSecurityMonitoringSignalAssignee".to_string(),
        test_edit_security_monitoring_signal_assignee,
    );
    world.function_mappings.insert(
        "EditSecurityMonitoringSignalState".to_string(),
        test_edit_security_monitoring_signal_state,
    );
    world.function_mappings.insert("ListSLOs".to_string(), test_list_sl_os);
    world.function_mappings.insert("CreateSLO".to_string(), test_create_slo);
    world.function_mappings.insert(
        "DeleteSLOTimeframeInBulk".to_string(),
        test_delete_slo_timeframe_in_bulk,
    );
    world
        .function_mappings
        .insert("CheckCanDeleteSLO".to_string(), test_check_can_delete_slo);
    world.function_mappings.insert("SearchSLO".to_string(), test_search_slo);
    world.function_mappings.insert("DeleteSLO".to_string(), test_delete_slo);
    world.function_mappings.insert("GetSLO".to_string(), test_get_slo);
    world.function_mappings.insert("UpdateSLO".to_string(), test_update_slo);
    world
        .function_mappings
        .insert("GetSLOCorrections".to_string(), test_get_slo_corrections);
    world
        .function_mappings
        .insert("GetSLOHistory".to_string(), test_get_slo_history);
    world
        .function_mappings
        .insert("ListSLOCorrection".to_string(), test_list_slo_correction);
    world
        .function_mappings
        .insert("CreateSLOCorrection".to_string(), test_create_slo_correction);
    world
        .function_mappings
        .insert("DeleteSLOCorrection".to_string(), test_delete_slo_correction);
    world
        .function_mappings
        .insert("GetSLOCorrection".to_string(), test_get_slo_correction);
    world
        .function_mappings
        .insert("UpdateSLOCorrection".to_string(), test_update_slo_correction);
    world
        .function_mappings
        .insert("ListHostTags".to_string(), test_list_host_tags);
    world
        .function_mappings
        .insert("DeleteHostTags".to_string(), test_delete_host_tags);
    world
        .function_mappings
        .insert("GetHostTags".to_string(), test_get_host_tags);
    world
        .function_mappings
        .insert("CreateHostTags".to_string(), test_create_host_tags);
    world
        .function_mappings
        .insert("UpdateHostTags".to_string(), test_update_host_tags);
    world.function_mappings.insert("ListUsers".to_string(), test_list_users);
    world
        .function_mappings
        .insert("CreateUser".to_string(), test_create_user);
    world
        .function_mappings
        .insert("DisableUser".to_string(), test_disable_user);
    world.function_mappings.insert("GetUser".to_string(), test_get_user);
    world
        .function_mappings
        .insert("UpdateUser".to_string(), test_update_user);
    world.function_mappings.insert("Validate".to_string(), test_validate);
    world
        .function_mappings
        .insert("ListSpansMetrics".to_string(), test_list_spans_metrics);
    world
        .function_mappings
        .insert("CreateSpansMetric".to_string(), test_create_spans_metric);
    world
        .function_mappings
        .insert("DeleteSpansMetric".to_string(), test_delete_spans_metric);
    world
        .function_mappings
        .insert("GetSpansMetric".to_string(), test_get_spans_metric);
    world
        .function_mappings
        .insert("UpdateSpansMetric".to_string(), test_update_spans_metric);
    world
        .function_mappings
        .insert("ListApmRetentionFilters".to_string(), test_list_apm_retention_filters);
    world
        .function_mappings
        .insert("CreateApmRetentionFilter".to_string(), test_create_apm_retention_filter);
    world.function_mappings.insert(
        "ReorderApmRetentionFilters".to_string(),
        test_reorder_apm_retention_filters,
    );
    world
        .function_mappings
        .insert("DeleteApmRetentionFilter".to_string(), test_delete_apm_retention_filter);
    world
        .function_mappings
        .insert("GetApmRetentionFilter".to_string(), test_get_apm_retention_filter);
    world
        .function_mappings
        .insert("UpdateApmRetentionFilter".to_string(), test_update_apm_retention_filter);
    world
        .function_mappings
        .insert("ListAuditLogs".to_string(), test_list_audit_logs);
    world
        .function_mappings
        .insert("SearchAuditLogs".to_string(), test_search_audit_logs);
    world
        .function_mappings
        .insert("DeleteDashboardListItems".to_string(), test_delete_dashboard_list_items);
    world
        .function_mappings
        .insert("GetDashboardListItems".to_string(), test_get_dashboard_list_items);
    world
        .function_mappings
        .insert("CreateDashboardListItems".to_string(), test_create_dashboard_list_items);
    world
        .function_mappings
        .insert("UpdateDashboardListItems".to_string(), test_update_dashboard_list_items);
    world
        .function_mappings
        .insert("ListEvents".to_string(), test_list_events);
    world
        .function_mappings
        .insert("SearchEvents".to_string(), test_search_events);
    world
        .function_mappings
        .insert("ListGCPSTSAccounts".to_string(), test_list_gcpsts_accounts);
    world
        .function_mappings
        .insert("CreateGCPSTSAccount".to_string(), test_create_gcpsts_account);
    world
        .function_mappings
        .insert("DeleteGCPSTSAccount".to_string(), test_delete_gcpsts_account);
    world
        .function_mappings
        .insert("UpdateGCPSTSAccount".to_string(), test_update_gcpsts_account);
    world
        .function_mappings
        .insert("GetGCPSTSDelegate".to_string(), test_get_gcpsts_delegate);
    world
        .function_mappings
        .insert("MakeGCPSTSDelegate".to_string(), test_make_gcpsts_delegate);
    world
        .function_mappings
        .insert("ListOpsgenieServices".to_string(), test_list_opsgenie_services);
    world
        .function_mappings
        .insert("CreateOpsgenieService".to_string(), test_create_opsgenie_service);
    world
        .function_mappings
        .insert("DeleteOpsgenieService".to_string(), test_delete_opsgenie_service);
    world
        .function_mappings
        .insert("GetOpsgenieService".to_string(), test_get_opsgenie_service);
    world
        .function_mappings
        .insert("UpdateOpsgenieService".to_string(), test_update_opsgenie_service);
    world
        .function_mappings
        .insert("ListCloudflareAccounts".to_string(), test_list_cloudflare_accounts);
    world
        .function_mappings
        .insert("CreateCloudflareAccount".to_string(), test_create_cloudflare_account);
    world
        .function_mappings
        .insert("DeleteCloudflareAccount".to_string(), test_delete_cloudflare_account);
    world
        .function_mappings
        .insert("GetCloudflareAccount".to_string(), test_get_cloudflare_account);
    world
        .function_mappings
        .insert("UpdateCloudflareAccount".to_string(), test_update_cloudflare_account);
    world
        .function_mappings
        .insert("ListConfluentAccount".to_string(), test_list_confluent_account);
    world
        .function_mappings
        .insert("CreateConfluentAccount".to_string(), test_create_confluent_account);
    world
        .function_mappings
        .insert("DeleteConfluentAccount".to_string(), test_delete_confluent_account);
    world
        .function_mappings
        .insert("GetConfluentAccount".to_string(), test_get_confluent_account);
    world
        .function_mappings
        .insert("UpdateConfluentAccount".to_string(), test_update_confluent_account);
    world
        .function_mappings
        .insert("ListConfluentResource".to_string(), test_list_confluent_resource);
    world
        .function_mappings
        .insert("CreateConfluentResource".to_string(), test_create_confluent_resource);
    world
        .function_mappings
        .insert("DeleteConfluentResource".to_string(), test_delete_confluent_resource);
    world
        .function_mappings
        .insert("GetConfluentResource".to_string(), test_get_confluent_resource);
    world
        .function_mappings
        .insert("UpdateConfluentResource".to_string(), test_update_confluent_resource);
    world
        .function_mappings
        .insert("ListFastlyAccounts".to_string(), test_list_fastly_accounts);
    world
        .function_mappings
        .insert("CreateFastlyAccount".to_string(), test_create_fastly_account);
    world
        .function_mappings
        .insert("DeleteFastlyAccount".to_string(), test_delete_fastly_account);
    world
        .function_mappings
        .insert("GetFastlyAccount".to_string(), test_get_fastly_account);
    world
        .function_mappings
        .insert("UpdateFastlyAccount".to_string(), test_update_fastly_account);
    world
        .function_mappings
        .insert("ListFastlyServices".to_string(), test_list_fastly_services);
    world
        .function_mappings
        .insert("CreateFastlyService".to_string(), test_create_fastly_service);
    world
        .function_mappings
        .insert("DeleteFastlyService".to_string(), test_delete_fastly_service);
    world
        .function_mappings
        .insert("GetFastlyService".to_string(), test_get_fastly_service);
    world
        .function_mappings
        .insert("UpdateFastlyService".to_string(), test_update_fastly_service);
    world
        .function_mappings
        .insert("GetIPAllowlist".to_string(), test_get_ip_allowlist);
    world
        .function_mappings
        .insert("UpdateIPAllowlist".to_string(), test_update_ip_allowlist);
    world.function_mappings.insert("SubmitLog".to_string(), test_submit_log);
    world
        .function_mappings
        .insert("ListLogsMetrics".to_string(), test_list_logs_metrics);
    world
        .function_mappings
        .insert("CreateLogsMetric".to_string(), test_create_logs_metric);
    world
        .function_mappings
        .insert("DeleteLogsMetric".to_string(), test_delete_logs_metric);
    world
        .function_mappings
        .insert("GetLogsMetric".to_string(), test_get_logs_metric);
    world
        .function_mappings
        .insert("UpdateLogsMetric".to_string(), test_update_logs_metric);
    world
        .function_mappings
        .insert("ListPermissions".to_string(), test_list_permissions);
    world
        .function_mappings
        .insert("ListFindings".to_string(), test_list_findings);
    world
        .function_mappings
        .insert("GetFinding".to_string(), test_get_finding);
    world
        .function_mappings
        .insert("UpdateFinding".to_string(), test_update_finding);
    world
        .function_mappings
        .insert("ListProcesses".to_string(), test_list_processes);
    world
        .function_mappings
        .insert("DeleteRestrictionPolicy".to_string(), test_delete_restriction_policy);
    world
        .function_mappings
        .insert("GetRestrictionPolicy".to_string(), test_get_restriction_policy);
    world
        .function_mappings
        .insert("UpdateRestrictionPolicy".to_string(), test_update_restriction_policy);
    world
        .function_mappings
        .insert("UploadIdPMetadata".to_string(), test_upload_id_p_metadata);
    world.function_mappings.insert(
        "DownloadCloudWorkloadPolicyFile".to_string(),
        test_download_cloud_workload_policy_file,
    );
    world.function_mappings.insert(
        "ListCloudWorkloadSecurityAgentRules".to_string(),
        test_list_cloud_workload_security_agent_rules,
    );
    world.function_mappings.insert(
        "CreateCloudWorkloadSecurityAgentRule".to_string(),
        test_create_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "DeleteCloudWorkloadSecurityAgentRule".to_string(),
        test_delete_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "GetCloudWorkloadSecurityAgentRule".to_string(),
        test_get_cloud_workload_security_agent_rule,
    );
    world.function_mappings.insert(
        "UpdateCloudWorkloadSecurityAgentRule".to_string(),
        test_update_cloud_workload_security_agent_rule,
    );
    world
        .function_mappings
        .insert("SubmitMetrics".to_string(), test_submit_metrics);
    world.function_mappings.insert(
        "GetOnDemandConcurrencyCap".to_string(),
        test_get_on_demand_concurrency_cap,
    );
    world.function_mappings.insert(
        "SetOnDemandConcurrencyCap".to_string(),
        test_set_on_demand_concurrency_cap,
    );
    world.function_mappings.insert(
        "GetUsageApplicationSecurityMonitoring".to_string(),
        test_get_usage_application_security_monitoring,
    );
    world
        .function_mappings
        .insert("GetCostByOrg".to_string(), test_get_cost_by_org);
    world
        .function_mappings
        .insert("GetEstimatedCostByOrg".to_string(), test_get_estimated_cost_by_org);
    world
        .function_mappings
        .insert("GetHistoricalCostByOrg".to_string(), test_get_historical_cost_by_org);
    world
        .function_mappings
        .insert("GetHourlyUsage".to_string(), test_get_hourly_usage);
    world.function_mappings.insert(
        "GetUsageLambdaTracedInvocations".to_string(),
        test_get_usage_lambda_traced_invocations,
    );
    world.function_mappings.insert(
        "GetUsageObservabilityPipelines".to_string(),
        test_get_usage_observability_pipelines,
    );
}

fn test_get_ip_ranges(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_ip_ranges
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

fn test_list_api_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
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

fn test_create_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = CreateAPIKeyParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAPIKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
    };
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

fn test_get_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = GetAPIKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
    };
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

fn test_update_api_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = UpdateAPIKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_application_keys(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
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

fn test_create_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = CreateApplicationKeyParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = DeleteApplicationKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
    };
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

fn test_get_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = GetApplicationKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
    };
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

fn test_update_application_key(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_key_management
        .as_ref()
        .expect("api instance not found");
    let params = UpdateApplicationKeyParams {
        key: serde_json::from_value(_parameters.get("key").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_submit_service_check(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_checks
        .as_ref()
        .expect("api instance not found");
    let params = SubmitServiceCheckParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_daily_custom_reports(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetDailyCustomReportsParams {
        page_size: serde_json::from_value(_parameters.get("page[size]").unwrap().clone()).unwrap(),
        page_number: serde_json::from_value(_parameters.get("page[number]").unwrap().clone()).unwrap(),
        sort_dir: serde_json::from_value(_parameters.get("sort_dir").unwrap().clone()).unwrap(),
        sort: serde_json::from_value(_parameters.get("sort").unwrap().clone()).unwrap(),
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

fn test_get_specified_daily_custom_reports(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetSpecifiedDailyCustomReportsParams {
        report_id: serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_monthly_custom_reports(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetMonthlyCustomReportsParams {
        page_size: serde_json::from_value(_parameters.get("page[size]").unwrap().clone()).unwrap(),
        page_number: serde_json::from_value(_parameters.get("page[number]").unwrap().clone()).unwrap(),
        sort_dir: serde_json::from_value(_parameters.get("sort_dir").unwrap().clone()).unwrap(),
        sort: serde_json::from_value(_parameters.get("sort").unwrap().clone()).unwrap(),
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

fn test_get_specified_monthly_custom_reports(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetSpecifiedMonthlyCustomReportsParams {
        report_id: serde_json::from_value(_parameters.get("report_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_analyzed_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageAnalyzedLogsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_attribution(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageAttributionParams {
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        fields: serde_json::from_value(_parameters.get("fields").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
        sort_direction: serde_json::from_value(_parameters.get("sort_direction").unwrap().clone()).unwrap(),
        sort_name: serde_json::from_value(_parameters.get("sort_name").unwrap().clone()).unwrap(),
        include_descendants: serde_json::from_value(_parameters.get("include_descendants").unwrap().clone()).unwrap(),
        offset: serde_json::from_value(_parameters.get("offset").unwrap().clone()).unwrap(),
        limit: serde_json::from_value(_parameters.get("limit").unwrap().clone()).unwrap(),
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

fn test_get_usage_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageAuditLogsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_lambda(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageLambdaParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_billable_summary(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageBillableSummaryParams {
        month: serde_json::from_value(_parameters.get("month").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_ci_app(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageCIAppParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_cloud_security_posture_management(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageCloudSecurityPostureManagementParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.get_usage_cloud_security_posture_management_with_http_info(params)) {
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

fn test_get_usage_cws(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageCWSParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_dbm(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageDBMParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_fargate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageFargateParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageHostsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_hourly_usage_attribution(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetHourlyUsageAttributionParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        usage_type: serde_json::from_value(_parameters.get("usage_type").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
        next_record_id: serde_json::from_value(_parameters.get("next_record_id").unwrap().clone()).unwrap(),
        tag_breakdown_keys: serde_json::from_value(_parameters.get("tag_breakdown_keys").unwrap().clone()).unwrap(),
        include_descendants: serde_json::from_value(_parameters.get("include_descendants").unwrap().clone()).unwrap(),
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

fn test_get_incident_management(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetIncidentManagementParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_indexed_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageIndexedSpansParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_ingested_spans(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetIngestedSpansParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_internet_of_things(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageInternetOfThingsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageLogsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_logs_by_retention(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageLogsByRetentionParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_logs_by_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageLogsByIndexParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
        index_name: serde_json::from_value(_parameters.get("index_name").unwrap().clone()).unwrap(),
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

fn test_get_monthly_usage_attribution(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetMonthlyUsageAttributionParams {
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        fields: serde_json::from_value(_parameters.get("fields").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
        sort_direction: serde_json::from_value(_parameters.get("sort_direction").unwrap().clone()).unwrap(),
        sort_name: serde_json::from_value(_parameters.get("sort_name").unwrap().clone()).unwrap(),
        tag_breakdown_keys: serde_json::from_value(_parameters.get("tag_breakdown_keys").unwrap().clone()).unwrap(),
        next_record_id: serde_json::from_value(_parameters.get("next_record_id").unwrap().clone()).unwrap(),
        include_descendants: serde_json::from_value(_parameters.get("include_descendants").unwrap().clone()).unwrap(),
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

fn test_get_usage_network_flows(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageNetworkFlowsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_network_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageNetworkHostsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_online_archive(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageOnlineArchiveParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_profiling(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageProfilingParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_rum_units(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageRumUnitsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_rum_sessions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageRumSessionsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
        type_: serde_json::from_value(_parameters.get("type").unwrap().clone()).unwrap(),
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

fn test_get_usage_sds(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSDSParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_snmp(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSNMPParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_summary(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSummaryParams {
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
        include_org_details: serde_json::from_value(_parameters.get("include_org_details").unwrap().clone()).unwrap(),
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

fn test_get_usage_synthetics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSyntheticsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_synthetics_api(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSyntheticsAPIParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_synthetics_browser(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageSyntheticsBrowserParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_timeseries(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageTimeseriesParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_top_avg_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageTopAvgMetricsParams {
        month: serde_json::from_value(_parameters.get("month").unwrap().clone()).unwrap(),
        day: serde_json::from_value(_parameters.get("day").unwrap().clone()).unwrap(),
        names: serde_json::from_value(_parameters.get("names").unwrap().clone()).unwrap(),
        limit: serde_json::from_value(_parameters.get("limit").unwrap().clone()).unwrap(),
        next_record_id: serde_json::from_value(_parameters.get("next_record_id").unwrap().clone()).unwrap(),
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

fn test_list_dashboard_lists(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
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

fn test_create_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = CreateDashboardListParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = DeleteDashboardListParams {
        list_id: serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = GetDashboardListParams {
        list_id: serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_dashboard_list(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = UpdateDashboardListParams {
        list_id: serde_json::from_value(_parameters.get("list_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = ListDowntimesParams {
        current_only: serde_json::from_value(_parameters.get("current_only").unwrap().clone()).unwrap(),
        with_creator: serde_json::from_value(_parameters.get("with_creator").unwrap().clone()).unwrap(),
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

fn test_create_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = CreateDowntimeParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_cancel_downtimes_by_scope(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = CancelDowntimesByScopeParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_cancel_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = CancelDowntimeParams {
        downtime_id: serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = GetDowntimeParams {
        downtime_id: serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_downtime(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = UpdateDowntimeParams {
        downtime_id: serde_json::from_value(_parameters.get("downtime_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_monitor_downtimes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_downtimes
        .as_ref()
        .expect("api instance not found");
    let params = ListMonitorDowntimesParams {
        monitor_id: serde_json::from_value(_parameters.get("monitor_id").unwrap().clone()).unwrap(),
    };
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

fn test_list_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_events.as_ref().expect("api instance not found");
    let params = ListEventsParams {
        start: serde_json::from_value(_parameters.get("start").unwrap().clone()).unwrap(),
        end: serde_json::from_value(_parameters.get("end").unwrap().clone()).unwrap(),
        priority: serde_json::from_value(_parameters.get("priority").unwrap().clone()).unwrap(),
        sources: serde_json::from_value(_parameters.get("sources").unwrap().clone()).unwrap(),
        tags: serde_json::from_value(_parameters.get("tags").unwrap().clone()).unwrap(),
        unaggregated: serde_json::from_value(_parameters.get("unaggregated").unwrap().clone()).unwrap(),
        exclude_aggregate: serde_json::from_value(_parameters.get("exclude_aggregate").unwrap().clone()).unwrap(),
        page: serde_json::from_value(_parameters.get("page").unwrap().clone()).unwrap(),
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

fn test_create_event(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_events.as_ref().expect("api instance not found");
    let params = CreateEventParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_event(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_events.as_ref().expect("api instance not found");
    let params = GetEventParams {
        event_id: serde_json::from_value(_parameters.get("event_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_graph_snapshot(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_snapshots
        .as_ref()
        .expect("api instance not found");
    let params = GetGraphSnapshotParams {
        start: serde_json::from_value(_parameters.get("start").unwrap().clone()).unwrap(),
        end: serde_json::from_value(_parameters.get("end").unwrap().clone()).unwrap(),
        metric_query: serde_json::from_value(_parameters.get("metric_query").unwrap().clone()).unwrap(),
        event_query: serde_json::from_value(_parameters.get("event_query").unwrap().clone()).unwrap(),
        graph_def: serde_json::from_value(_parameters.get("graph_def").unwrap().clone()).unwrap(),
        title: serde_json::from_value(_parameters.get("title").unwrap().clone()).unwrap(),
        height: serde_json::from_value(_parameters.get("height").unwrap().clone()).unwrap(),
        width: serde_json::from_value(_parameters.get("width").unwrap().clone()).unwrap(),
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

fn test_mute_host(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_hosts.as_ref().expect("api instance not found");
    let params = MuteHostParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_unmute_host(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_hosts.as_ref().expect("api instance not found");
    let params = UnmuteHostParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
    };
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

fn test_list_hosts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_hosts.as_ref().expect("api instance not found");
    let params = ListHostsParams {
        filter: serde_json::from_value(_parameters.get("filter").unwrap().clone()).unwrap(),
        sort_field: serde_json::from_value(_parameters.get("sort_field").unwrap().clone()).unwrap(),
        sort_dir: serde_json::from_value(_parameters.get("sort_dir").unwrap().clone()).unwrap(),
        start: serde_json::from_value(_parameters.get("start").unwrap().clone()).unwrap(),
        count: serde_json::from_value(_parameters.get("count").unwrap().clone()).unwrap(),
        from: serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap(),
        include_muted_hosts_data: serde_json::from_value(_parameters.get("include_muted_hosts_data").unwrap().clone())
            .unwrap(),
        include_hosts_metadata: serde_json::from_value(_parameters.get("include_hosts_metadata").unwrap().clone())
            .unwrap(),
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

fn test_get_host_totals(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_hosts.as_ref().expect("api instance not found");
    let params = GetHostTotalsParams {
        from: serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap(),
    };
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

fn test_delete_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAWSAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_aws_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = ListAWSAccountsParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        role_name: serde_json::from_value(_parameters.get("role_name").unwrap().clone()).unwrap(),
        access_key_id: serde_json::from_value(_parameters.get("access_key_id").unwrap().clone()).unwrap(),
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

fn test_create_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateAWSAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_update_aws_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateAWSAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        role_name: serde_json::from_value(_parameters.get("role_name").unwrap().clone()).unwrap(),
        access_key_id: serde_json::from_value(_parameters.get("access_key_id").unwrap().clone()).unwrap(),
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

fn test_list_available_aws_namespaces(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
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

fn test_delete_aws_event_bridge_source(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAWSEventBridgeSourceParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_aws_event_bridge_sources(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
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

fn test_create_aws_event_bridge_source(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateAWSEventBridgeSourceParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_aws_tag_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAWSTagFilterParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_aws_tag_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = ListAWSTagFiltersParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_create_aws_tag_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateAWSTagFilterParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_create_new_aws_external_id(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateNewAWSExternalIDParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_aws_lambda_arn(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAWSLambdaARNParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_aws_logs_integrations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
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

fn test_create_aws_lambda_arn(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateAWSLambdaARNParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_check_aws_logs_lambda_async(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let params = CheckAWSLogsLambdaAsyncParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_aws_logs_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
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

fn test_enable_aws_log_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let params = EnableAWSLogServicesParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_check_aws_logs_services_async(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_aws_logs_integration
        .as_ref()
        .expect("api instance not found");
    let params = CheckAWSLogsServicesAsyncParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_azure_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteAzureIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_azure_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_azure_integration
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

fn test_create_azure_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateAzureIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_update_azure_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateAzureIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_update_azure_host_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_azure_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateAzureHostFiltersParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteGCPIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
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

fn test_create_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateGCPIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_update_gcp_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateGCPIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_create_pager_duty_integration_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreatePagerDutyIntegrationServiceParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.create_pager_duty_integration_service_with_http_info(params)) {
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

fn test_delete_pager_duty_integration_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeletePagerDutyIntegrationServiceParams {
        service_name: serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.delete_pager_duty_integration_service_with_http_info(params)) {
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

fn test_get_pager_duty_integration_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetPagerDutyIntegrationServiceParams {
        service_name: serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap(),
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

fn test_update_pager_duty_integration_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_pager_duty_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdatePagerDutyIntegrationServiceParams {
        service_name: serde_json::from_value(_parameters.get("service_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.update_pager_duty_integration_service_with_http_info(params)) {
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

fn test_get_slack_integration_channels(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetSlackIntegrationChannelsParams {
        account_name: serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap(),
    };
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

fn test_create_slack_integration_channel(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateSlackIntegrationChannelParams {
        account_name: serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_remove_slack_integration_channel(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let params = RemoveSlackIntegrationChannelParams {
        account_name: serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap(),
        channel_name: serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap(),
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

fn test_get_slack_integration_channel(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetSlackIntegrationChannelParams {
        account_name: serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap(),
        channel_name: serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap(),
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

fn test_update_slack_integration_channel(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_slack_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateSlackIntegrationChannelParams {
        account_name: serde_json::from_value(_parameters.get("account_name").unwrap().clone()).unwrap(),
        channel_name: serde_json::from_value(_parameters.get("channel_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_create_webhooks_integration_custom_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateWebhooksIntegrationCustomVariableParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.create_webhooks_integration_custom_variable_with_http_info(params)) {
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

fn test_delete_webhooks_integration_custom_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteWebhooksIntegrationCustomVariableParams {
        custom_variable_name: serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.delete_webhooks_integration_custom_variable_with_http_info(params)) {
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

fn test_get_webhooks_integration_custom_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetWebhooksIntegrationCustomVariableParams {
        custom_variable_name: serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.get_webhooks_integration_custom_variable_with_http_info(params)) {
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

fn test_update_webhooks_integration_custom_variable(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateWebhooksIntegrationCustomVariableParams {
        custom_variable_name: serde_json::from_value(_parameters.get("custom_variable_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.update_webhooks_integration_custom_variable_with_http_info(params)) {
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

fn test_create_webhooks_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateWebhooksIntegrationParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_webhooks_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteWebhooksIntegrationParams {
        webhook_name: serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap(),
    };
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

fn test_get_webhooks_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetWebhooksIntegrationParams {
        webhook_name: serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap(),
    };
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

fn test_update_webhooks_integration(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_webhooks_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateWebhooksIntegrationParams {
        webhook_name: serde_json::from_value(_parameters.get("webhook_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_logs.as_ref().expect("api instance not found");
    let params = ListLogsParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_submit_log(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_logs.as_ref().expect("api instance not found");
    let params = SubmitLogParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        content_encoding: serde_json::from_value(_parameters.get("Content-Encoding").unwrap().clone()).unwrap(),
        ddtags: serde_json::from_value(_parameters.get("ddtags").unwrap().clone()).unwrap(),
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

fn test_get_logs_index_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
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

fn test_update_logs_index_order(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let params = UpdateLogsIndexOrderParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_log_indexes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
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

fn test_create_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let params = CreateLogsIndexParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let params = GetLogsIndexParams {
        name: serde_json::from_value(_parameters.get("name").unwrap().clone()).unwrap(),
    };
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

fn test_update_logs_index(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_indexes
        .as_ref()
        .expect("api instance not found");
    let params = UpdateLogsIndexParams {
        name: serde_json::from_value(_parameters.get("name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_active_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = ListActiveMetricsParams {
        from: serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap(),
        host: serde_json::from_value(_parameters.get("host").unwrap().clone()).unwrap(),
        tag_filter: serde_json::from_value(_parameters.get("tag_filter").unwrap().clone()).unwrap(),
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

fn test_get_metric_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = GetMetricMetadataParams {
        metric_name: serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap(),
    };
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

fn test_update_metric_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = UpdateMetricMetadataParams {
        metric_name: serde_json::from_value(_parameters.get("metric_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_query_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = QueryMetricsParams {
        from: serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap(),
        to: serde_json::from_value(_parameters.get("to").unwrap().clone()).unwrap(),
        query: serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap(),
    };
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

fn test_list_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = ListMetricsParams {
        q: serde_json::from_value(_parameters.get("q").unwrap().clone()).unwrap(),
    };
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

fn test_submit_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = SubmitMetricsParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        content_encoding: serde_json::from_value(_parameters.get("Content-Encoding").unwrap().clone()).unwrap(),
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

fn test_list_orgs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
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

fn test_create_child_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = CreateChildOrgParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = GetOrgParams {
        public_id: serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = UpdateOrgParams {
        public_id: serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_downgrade_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = DowngradeOrgParams {
        public_id: serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap(),
    };
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

fn test_upload_id_p_for_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = UploadIdPForOrgParams {
        public_id: serde_json::from_value(_parameters.get("public_id").unwrap().clone()).unwrap(),
        idp_file: serde_json::from_value(_parameters.get("idp_file").unwrap().clone()).unwrap(),
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

fn test_add_security_monitoring_signal_to_incident(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = AddSecurityMonitoringSignalToIncidentParams {
        signal_id: serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.add_security_monitoring_signal_to_incident_with_http_info(params)) {
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

fn test_edit_security_monitoring_signal_assignee(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = EditSecurityMonitoringSignalAssigneeParams {
        signal_id: serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.edit_security_monitoring_signal_assignee_with_http_info(params)) {
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

fn test_edit_security_monitoring_signal_state(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = EditSecurityMonitoringSignalStateParams {
        signal_id: serde_json::from_value(_parameters.get("signal_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.edit_security_monitoring_signal_state_with_http_info(params)) {
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

fn test_list_sl_os(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = ListSLOsParams {
        ids: serde_json::from_value(_parameters.get("ids").unwrap().clone()).unwrap(),
        query: serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap(),
        tags_query: serde_json::from_value(_parameters.get("tags_query").unwrap().clone()).unwrap(),
        metrics_query: serde_json::from_value(_parameters.get("metrics_query").unwrap().clone()).unwrap(),
        limit: serde_json::from_value(_parameters.get("limit").unwrap().clone()).unwrap(),
        offset: serde_json::from_value(_parameters.get("offset").unwrap().clone()).unwrap(),
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

fn test_create_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = CreateSLOParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_slo_timeframe_in_bulk(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = DeleteSLOTimeframeInBulkParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_check_can_delete_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = CheckCanDeleteSLOParams {
        ids: serde_json::from_value(_parameters.get("ids").unwrap().clone()).unwrap(),
    };
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

fn test_search_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = SearchSLOParams {
        query: serde_json::from_value(_parameters.get("query").unwrap().clone()).unwrap(),
        page_size: serde_json::from_value(_parameters.get("page[size]").unwrap().clone()).unwrap(),
        page_number: serde_json::from_value(_parameters.get("page[number]").unwrap().clone()).unwrap(),
        include_facets: serde_json::from_value(_parameters.get("include_facets").unwrap().clone()).unwrap(),
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

fn test_delete_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = DeleteSLOParams {
        slo_id: serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap(),
        force: serde_json::from_value(_parameters.get("force").unwrap().clone()).unwrap(),
    };
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

fn test_get_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = GetSLOParams {
        slo_id: serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap(),
        with_configured_alert_ids: serde_json::from_value(
            _parameters.get("with_configured_alert_ids").unwrap().clone(),
        )
        .unwrap(),
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

fn test_update_slo(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = UpdateSLOParams {
        slo_id: serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_slo_corrections(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = GetSLOCorrectionsParams {
        slo_id: serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_slo_history(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objectives
        .as_ref()
        .expect("api instance not found");
    let params = GetSLOHistoryParams {
        slo_id: serde_json::from_value(_parameters.get("slo_id").unwrap().clone()).unwrap(),
        from_ts: serde_json::from_value(_parameters.get("from_ts").unwrap().clone()).unwrap(),
        to_ts: serde_json::from_value(_parameters.get("to_ts").unwrap().clone()).unwrap(),
        target: serde_json::from_value(_parameters.get("target").unwrap().clone()).unwrap(),
        apply_correction: serde_json::from_value(_parameters.get("apply_correction").unwrap().clone()).unwrap(),
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

fn test_list_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let params = ListSLOCorrectionParams {
        offset: serde_json::from_value(_parameters.get("offset").unwrap().clone()).unwrap(),
        limit: serde_json::from_value(_parameters.get("limit").unwrap().clone()).unwrap(),
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

fn test_create_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let params = CreateSLOCorrectionParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let params = DeleteSLOCorrectionParams {
        slo_correction_id: serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap(),
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

fn test_get_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let params = GetSLOCorrectionParams {
        slo_correction_id: serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap(),
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

fn test_update_slo_correction(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_service_level_objective_corrections
        .as_ref()
        .expect("api instance not found");
    let params = UpdateSLOCorrectionParams {
        slo_correction_id: serde_json::from_value(_parameters.get("slo_correction_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_tags.as_ref().expect("api instance not found");
    let params = ListHostTagsParams {
        source: serde_json::from_value(_parameters.get("source").unwrap().clone()).unwrap(),
    };
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

fn test_delete_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_tags.as_ref().expect("api instance not found");
    let params = DeleteHostTagsParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
        source: serde_json::from_value(_parameters.get("source").unwrap().clone()).unwrap(),
    };
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

fn test_get_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_tags.as_ref().expect("api instance not found");
    let params = GetHostTagsParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
        source: serde_json::from_value(_parameters.get("source").unwrap().clone()).unwrap(),
    };
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

fn test_create_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_tags.as_ref().expect("api instance not found");
    let params = CreateHostTagsParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        source: serde_json::from_value(_parameters.get("source").unwrap().clone()).unwrap(),
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

fn test_update_host_tags(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_tags.as_ref().expect("api instance not found");
    let params = UpdateHostTagsParams {
        host_name: serde_json::from_value(_parameters.get("host_name").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        source: serde_json::from_value(_parameters.get("source").unwrap().clone()).unwrap(),
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

fn test_list_users(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_users.as_ref().expect("api instance not found");
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

fn test_create_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_users.as_ref().expect("api instance not found");
    let params = CreateUserParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_disable_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_users.as_ref().expect("api instance not found");
    let params = DisableUserParams {
        user_handle: serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap(),
    };
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

fn test_get_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_users.as_ref().expect("api instance not found");
    let params = GetUserParams {
        user_handle: serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap(),
    };
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

fn test_update_user(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_users.as_ref().expect("api instance not found");
    let params = UpdateUserParams {
        user_handle: serde_json::from_value(_parameters.get("user_handle").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_validate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_authentication
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

fn test_list_spans_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_spans_metrics
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

fn test_create_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let params = CreateSpansMetricParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let params = DeleteSpansMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let params = GetSpansMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_spans_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_spans_metrics
        .as_ref()
        .expect("api instance not found");
    let params = UpdateSpansMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_apm_retention_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
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

fn test_create_apm_retention_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let params = CreateApmRetentionFilterParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_reorder_apm_retention_filters(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let params = ReorderApmRetentionFiltersParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_apm_retention_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let params = DeleteApmRetentionFilterParams {
        filter_id: serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_apm_retention_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let params = GetApmRetentionFilterParams {
        filter_id: serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_apm_retention_filter(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_apm_retention_filters
        .as_ref()
        .expect("api instance not found");
    let params = UpdateApmRetentionFilterParams {
        filter_id: serde_json::from_value(_parameters.get("filter_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_audit.as_ref().expect("api instance not found");
    let params = ListAuditLogsParams {
        filter_query: serde_json::from_value(_parameters.get("filter[query]").unwrap().clone()).unwrap(),
        filter_from: serde_json::from_value(_parameters.get("filter[from]").unwrap().clone()).unwrap(),
        filter_to: serde_json::from_value(_parameters.get("filter[to]").unwrap().clone()).unwrap(),
        sort: serde_json::from_value(_parameters.get("sort").unwrap().clone()).unwrap(),
        page_cursor: serde_json::from_value(_parameters.get("page[cursor]").unwrap().clone()).unwrap(),
        page_limit: serde_json::from_value(_parameters.get("page[limit]").unwrap().clone()).unwrap(),
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

fn test_search_audit_logs(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_audit.as_ref().expect("api instance not found");
    let params = SearchAuditLogsParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_dashboard_list_items(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = DeleteDashboardListItemsParams {
        dashboard_list_id: serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_get_dashboard_list_items(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = GetDashboardListItemsParams {
        dashboard_list_id: serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap(),
    };
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

fn test_create_dashboard_list_items(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = CreateDashboardListItemsParams {
        dashboard_list_id: serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_update_dashboard_list_items(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_dashboard_lists
        .as_ref()
        .expect("api instance not found");
    let params = UpdateDashboardListItemsParams {
        dashboard_list_id: serde_json::from_value(_parameters.get("dashboard_list_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_events.as_ref().expect("api instance not found");
    let params = ListEventsParams {
        filter_query: serde_json::from_value(_parameters.get("filter[query]").unwrap().clone()).unwrap(),
        filter_from: serde_json::from_value(_parameters.get("filter[from]").unwrap().clone()).unwrap(),
        filter_to: serde_json::from_value(_parameters.get("filter[to]").unwrap().clone()).unwrap(),
        sort: serde_json::from_value(_parameters.get("sort").unwrap().clone()).unwrap(),
        page_cursor: serde_json::from_value(_parameters.get("page[cursor]").unwrap().clone()).unwrap(),
        page_limit: serde_json::from_value(_parameters.get("page[limit]").unwrap().clone()).unwrap(),
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

fn test_search_events(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_events.as_ref().expect("api instance not found");
    let params = SearchEventsParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_gcpsts_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
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

fn test_create_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateGCPSTSAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteGCPSTSAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_gcpsts_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateGCPSTSAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_gcpsts_delegate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
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

fn test_make_gcpsts_delegate(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_gcp_integration
        .as_ref()
        .expect("api instance not found");
    let params = MakeGCPSTSDelegateParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_opsgenie_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_opsgenie_integration
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

fn test_create_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateOpsgenieServiceParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteOpsgenieServiceParams {
        integration_service_id: serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone())
            .unwrap(),
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

fn test_get_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetOpsgenieServiceParams {
        integration_service_id: serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone())
            .unwrap(),
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

fn test_update_opsgenie_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_opsgenie_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateOpsgenieServiceParams {
        integration_service_id: serde_json::from_value(_parameters.get("integration_service_id").unwrap().clone())
            .unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_cloudflare_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloudflare_integration
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

fn test_create_cloudflare_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateCloudflareAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_cloudflare_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteCloudflareAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_cloudflare_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetCloudflareAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_cloudflare_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloudflare_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateCloudflareAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
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

fn test_create_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = CreateConfluentAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = DeleteConfluentAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = GetConfluentAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_confluent_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = UpdateConfluentAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = ListConfluentResourceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_create_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = CreateConfluentResourceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = DeleteConfluentResourceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
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

fn test_get_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = GetConfluentResourceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
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

fn test_update_confluent_resource(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_confluent_cloud
        .as_ref()
        .expect("api instance not found");
    let params = UpdateConfluentResourceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_list_fastly_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
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

fn test_create_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateFastlyAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_fastly_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = ListFastlyServicesParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
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

fn test_create_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = CreateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = DeleteFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
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

fn test_get_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = GetFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
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

fn test_update_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_fastly_integration
        .as_ref()
        .expect("api instance not found");
    let params = UpdateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_get_ip_allowlist(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_ip_allowlist
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

fn test_update_ip_allowlist(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_ip_allowlist
        .as_ref()
        .expect("api instance not found");
    let params = UpdateIPAllowlistParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_submit_log(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_logs.as_ref().expect("api instance not found");
    let params = SubmitLogParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        content_encoding: serde_json::from_value(_parameters.get("Content-Encoding").unwrap().clone()).unwrap(),
        ddtags: serde_json::from_value(_parameters.get("ddtags").unwrap().clone()).unwrap(),
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

fn test_list_logs_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_metrics
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

fn test_create_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let params = CreateLogsMetricParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_delete_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let params = DeleteLogsMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let params = GetLogsMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_logs_metric(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_logs_metrics
        .as_ref()
        .expect("api instance not found");
    let params = UpdateLogsMetricParams {
        metric_id: serde_json::from_value(_parameters.get("metric_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_permissions(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world.api_instances.api_roles.as_ref().expect("api instance not found");
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

fn test_list_findings(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = ListFindingsParams {
        page_limit: serde_json::from_value(_parameters.get("page[limit]").unwrap().clone()).unwrap(),
        snapshot_timestamp: serde_json::from_value(_parameters.get("snapshot_timestamp").unwrap().clone()).unwrap(),
        page_cursor: serde_json::from_value(_parameters.get("page[cursor]").unwrap().clone()).unwrap(),
        filter_tags: serde_json::from_value(_parameters.get("filter[tags]").unwrap().clone()).unwrap(),
        filter_evaluation_changed_at: serde_json::from_value(
            _parameters.get("filter[evaluation_changed_at]").unwrap().clone(),
        )
        .unwrap(),
        filter_muted: serde_json::from_value(_parameters.get("filter[muted]").unwrap().clone()).unwrap(),
        filter_rule_id: serde_json::from_value(_parameters.get("filter[rule_id]").unwrap().clone()).unwrap(),
        filter_rule_name: serde_json::from_value(_parameters.get("filter[rule_name]").unwrap().clone()).unwrap(),
        filter_resource_type: serde_json::from_value(_parameters.get("filter[resource_type]").unwrap().clone())
            .unwrap(),
        filter_discovery_timestamp: serde_json::from_value(
            _parameters.get("filter[discovery_timestamp]").unwrap().clone(),
        )
        .unwrap(),
        filter_evaluation: serde_json::from_value(_parameters.get("filter[evaluation]").unwrap().clone()).unwrap(),
        filter_status: serde_json::from_value(_parameters.get("filter[status]").unwrap().clone()).unwrap(),
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

fn test_get_finding(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = GetFindingParams {
        finding_id: serde_json::from_value(_parameters.get("finding_id").unwrap().clone()).unwrap(),
        snapshot_timestamp: serde_json::from_value(_parameters.get("snapshot_timestamp").unwrap().clone()).unwrap(),
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

fn test_update_finding(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_security_monitoring
        .as_ref()
        .expect("api instance not found");
    let params = UpdateFindingParams {
        finding_id: serde_json::from_value(_parameters.get("finding_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_list_processes(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_processes
        .as_ref()
        .expect("api instance not found");
    let params = ListProcessesParams {
        search: serde_json::from_value(_parameters.get("search").unwrap().clone()).unwrap(),
        tags: serde_json::from_value(_parameters.get("tags").unwrap().clone()).unwrap(),
        from: serde_json::from_value(_parameters.get("from").unwrap().clone()).unwrap(),
        to: serde_json::from_value(_parameters.get("to").unwrap().clone()).unwrap(),
        page_limit: serde_json::from_value(_parameters.get("page[limit]").unwrap().clone()).unwrap(),
        page_cursor: serde_json::from_value(_parameters.get("page[cursor]").unwrap().clone()).unwrap(),
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

fn test_delete_restriction_policy(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let params = DeleteRestrictionPolicyParams {
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
    };
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

fn test_get_restriction_policy(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let params = GetRestrictionPolicyParams {
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
    };
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

fn test_update_restriction_policy(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_restriction_policies
        .as_ref()
        .expect("api instance not found");
    let params = UpdateRestrictionPolicyParams {
        resource_id: serde_json::from_value(_parameters.get("resource_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
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

fn test_upload_id_p_metadata(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_organizations
        .as_ref()
        .expect("api instance not found");
    let params = UploadIdPMetadataParams {
        idp_file: serde_json::from_value(_parameters.get("idp_file").unwrap().clone()).unwrap(),
    };
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

fn test_download_cloud_workload_policy_file(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
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

fn test_list_cloud_workload_security_agent_rules(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
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

fn test_create_cloud_workload_security_agent_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let params = CreateCloudWorkloadSecurityAgentRuleParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.create_cloud_workload_security_agent_rule_with_http_info(params)) {
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

fn test_delete_cloud_workload_security_agent_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let params = DeleteCloudWorkloadSecurityAgentRuleParams {
        agent_rule_id: serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.delete_cloud_workload_security_agent_rule_with_http_info(params)) {
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

fn test_get_cloud_workload_security_agent_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let params = GetCloudWorkloadSecurityAgentRuleParams {
        agent_rule_id: serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.get_cloud_workload_security_agent_rule_with_http_info(params)) {
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

fn test_update_cloud_workload_security_agent_rule(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_cloud_workload_security
        .as_ref()
        .expect("api instance not found");
    let params = UpdateCloudWorkloadSecurityAgentRuleParams {
        agent_rule_id: serde_json::from_value(_parameters.get("agent_rule_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.update_cloud_workload_security_agent_rule_with_http_info(params)) {
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

fn test_submit_metrics(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_metrics
        .as_ref()
        .expect("api instance not found");
    let params = SubmitMetricsParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
        content_encoding: serde_json::from_value(_parameters.get("Content-Encoding").unwrap().clone()).unwrap(),
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

fn test_get_on_demand_concurrency_cap(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_synthetics
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

fn test_set_on_demand_concurrency_cap(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_synthetics
        .as_ref()
        .expect("api instance not found");
    let params = SetOnDemandConcurrencyCapParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
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

fn test_get_usage_application_security_monitoring(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageApplicationSecurityMonitoringParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
    };
    let response = match block_on(api.get_usage_application_security_monitoring_with_http_info(params)) {
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

fn test_get_cost_by_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetCostByOrgParams {
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
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

fn test_get_estimated_cost_by_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetEstimatedCostByOrgParams {
        view: serde_json::from_value(_parameters.get("view").unwrap().clone()).unwrap(),
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
        start_date: serde_json::from_value(_parameters.get("start_date").unwrap().clone()).unwrap(),
        end_date: serde_json::from_value(_parameters.get("end_date").unwrap().clone()).unwrap(),
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

fn test_get_historical_cost_by_org(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetHistoricalCostByOrgParams {
        start_month: serde_json::from_value(_parameters.get("start_month").unwrap().clone()).unwrap(),
        view: serde_json::from_value(_parameters.get("view").unwrap().clone()).unwrap(),
        end_month: serde_json::from_value(_parameters.get("end_month").unwrap().clone()).unwrap(),
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

fn test_get_hourly_usage(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetHourlyUsageParams {
        filter_timestamp_start: serde_json::from_value(_parameters.get("filter[timestamp][start]").unwrap().clone())
            .unwrap(),
        filter_product_families: serde_json::from_value(_parameters.get("filter[product_families]").unwrap().clone())
            .unwrap(),
        filter_timestamp_end: serde_json::from_value(_parameters.get("filter[timestamp][end]").unwrap().clone())
            .unwrap(),
        filter_include_descendants: serde_json::from_value(
            _parameters.get("filter[include_descendants]").unwrap().clone(),
        )
        .unwrap(),
        filter_include_breakdown: serde_json::from_value(_parameters.get("filter[include_breakdown]").unwrap().clone())
            .unwrap(),
        filter_versions: serde_json::from_value(_parameters.get("filter[versions]").unwrap().clone()).unwrap(),
        page_limit: serde_json::from_value(_parameters.get("page[limit]").unwrap().clone()).unwrap(),
        page_next_record_id: serde_json::from_value(_parameters.get("page[next_record_id]").unwrap().clone()).unwrap(),
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

fn test_get_usage_lambda_traced_invocations(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageLambdaTracedInvocationsParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
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

fn test_get_usage_observability_pipelines(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let api = world
        .api_instances
        .api_usage_metering
        .as_ref()
        .expect("api instance not found");
    let params = GetUsageObservabilityPipelinesParams {
        start_hr: serde_json::from_value(_parameters.get("start_hr").unwrap().clone()).unwrap(),
        end_hr: serde_json::from_value(_parameters.get("end_hr").unwrap().clone()).unwrap(),
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
