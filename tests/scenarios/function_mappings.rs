use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;

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
