// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response summarizing all usage aggregated across the months in the request for all organizations, and broken down by month and by organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSummaryResponse {
    /// Shows the 99th percentile of all agent hosts over all hours in the current months for all organizations.
    #[serde(rename = "agent_host_top99p_sum")]
    pub agent_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current months all organizations.
    #[serde(rename = "apm_azure_app_service_host_top99p_sum")]
    pub apm_azure_app_service_host_top99p_sum: Option<i64>,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "apm_fargate_count_avg_sum")]
    pub apm_fargate_count_avg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current months for all organizations.
    #[serde(rename = "apm_host_top99p_sum")]
    pub apm_host_top99p_sum: Option<i64>,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "appsec_fargate_count_avg_sum")]
    pub appsec_fargate_count_avg_sum: Option<i64>,
    /// Shows the sum of all audit logs lines indexed over all hours in the current months for all organizations.
    #[deprecated]
    #[serde(rename = "audit_logs_lines_indexed_agg_sum")]
    pub audit_logs_lines_indexed_agg_sum: Option<i64>,
    /// Shows the total number of organizations that had Audit Trail enabled over a specific number of months.
    #[serde(rename = "audit_trail_enabled_hwm_sum")]
    pub audit_trail_enabled_hwm_sum: Option<i64>,
    /// Shows the average of all profiled Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "avg_profiled_fargate_tasks_sum")]
    pub avg_profiled_fargate_tasks_sum: Option<i64>,
    /// Shows the 99th percentile of all AWS hosts over all hours in the current months for all organizations.
    #[serde(rename = "aws_host_top99p_sum")]
    pub aws_host_top99p_sum: Option<i64>,
    /// Shows the average of the number of functions that executed 1 or more times each hour in the current months for all organizations.
    #[serde(rename = "aws_lambda_func_count")]
    pub aws_lambda_func_count: Option<i64>,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current months for all organizations.
    #[serde(rename = "aws_lambda_invocations_sum")]
    pub aws_lambda_invocations_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services over all hours in the current months for all organizations.
    #[serde(rename = "azure_app_service_top99p_sum")]
    pub azure_app_service_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure hosts over all hours in the current months for all organizations.
    #[serde(rename = "azure_host_top99p_sum")]
    pub azure_host_top99p_sum: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current months for all organizations.
    #[serde(rename = "billable_ingested_bytes_agg_sum")]
    pub billable_ingested_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all browser lite sessions over all hours in the current months for all organizations.
    #[serde(rename = "browser_rum_lite_session_count_agg_sum")]
    pub browser_rum_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser replay sessions over all hours in the current months for all organizations.
    #[serde(rename = "browser_rum_replay_session_count_agg_sum")]
    pub browser_rum_replay_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser RUM units over all hours in the current months for all organizations.
    #[serde(rename = "browser_rum_units_agg_sum")]
    pub browser_rum_units_agg_sum: Option<i64>,
    /// Shows the sum of all CI pipeline indexed spans over all hours in the current months for all organizations.
    #[serde(rename = "ci_pipeline_indexed_spans_agg_sum")]
    pub ci_pipeline_indexed_spans_agg_sum: Option<i64>,
    /// Shows the sum of all CI test indexed spans over all hours in the current months for all organizations.
    #[serde(rename = "ci_test_indexed_spans_agg_sum")]
    pub ci_test_indexed_spans_agg_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility intelligent test runner committers over all hours in the current months for all organizations.
    #[serde(rename = "ci_visibility_itr_committers_hwm_sum")]
    pub ci_visibility_itr_committers_hwm_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility pipeline committers over all hours in the current months for all organizations.
    #[serde(rename = "ci_visibility_pipeline_committers_hwm_sum")]
    pub ci_visibility_pipeline_committers_hwm_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility test committers over all hours in the current months for all organizations.
    #[serde(rename = "ci_visibility_test_committers_hwm_sum")]
    pub ci_visibility_test_committers_hwm_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for AWS.
    #[serde(rename = "cloud_cost_management_aws_host_count_avg_sum")]
    pub cloud_cost_management_aws_host_count_avg_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for Azure.
    #[serde(rename = "cloud_cost_management_azure_host_count_avg_sum")]
    pub cloud_cost_management_azure_host_count_avg_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for all cloud providers.
    #[serde(rename = "cloud_cost_management_host_count_avg_sum")]
    pub cloud_cost_management_host_count_avg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Information and Event Management events over all hours in the current months for all organizations.
    #[serde(rename = "cloud_siem_events_agg_sum")]
    pub cloud_siem_events_agg_sum: Option<i64>,
    /// Shows the average of all distinct containers over all hours in the current months for all organizations.
    #[serde(rename = "container_avg_sum")]
    pub container_avg_sum: Option<i64>,
    /// Shows the average of the containers without the Datadog Agent over all hours in the current month for all organizations.
    #[serde(rename = "container_excl_agent_avg_sum")]
    pub container_excl_agent_avg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of all distinct containers over all hours in the current months for all organizations.
    #[serde(rename = "container_hwm_sum")]
    pub container_hwm_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise compliance containers over all hours in the current months for all organizations.
    #[serde(rename = "csm_container_enterprise_compliance_count_agg_sum")]
    pub csm_container_enterprise_compliance_count_agg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise Cloud Workload Security containers over all hours in the current months for all organizations.
    #[serde(rename = "csm_container_enterprise_cws_count_agg_sum")]
    pub csm_container_enterprise_cws_count_agg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise containers over all hours in the current months for all organizations.
    #[serde(rename = "csm_container_enterprise_total_count_agg_sum")]
    pub csm_container_enterprise_total_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure app services hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_aas_host_count_top99p_sum")]
    pub csm_host_enterprise_aas_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise AWS hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_aws_host_count_top99p_sum")]
    pub csm_host_enterprise_aws_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_azure_host_count_top99p_sum")]
    pub csm_host_enterprise_azure_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise compliance hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_compliance_host_count_top99p_sum")]
    pub csm_host_enterprise_compliance_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Cloud Workload Security hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_cws_host_count_top99p_sum")]
    pub csm_host_enterprise_cws_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise GCP hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_gcp_host_count_top99p_sum")]
    pub csm_host_enterprise_gcp_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise hosts over all hours in the current months for all organizations.
    #[serde(rename = "csm_host_enterprise_total_host_count_top99p_sum")]
    pub csm_host_enterprise_total_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure app services hosts over all hours in the current months for all organizations.
    #[serde(rename = "cspm_aas_host_top99p_sum")]
    pub cspm_aas_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro AWS hosts over all hours in the current months for all organizations.
    #[serde(rename = "cspm_aws_host_top99p_sum")]
    pub cspm_aws_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure hosts over all hours in the current months for all organizations.
    #[serde(rename = "cspm_azure_host_top99p_sum")]
    pub cspm_azure_host_top99p_sum: Option<i64>,
    /// Shows the average number of Cloud Security Management Pro containers over all hours in the current months for all organizations.
    #[serde(rename = "cspm_container_avg_sum")]
    pub cspm_container_avg_sum: Option<i64>,
    /// Shows the sum of the the high-water marks of Cloud Security Management Pro containers over all hours in the current months for all organizations.
    #[serde(rename = "cspm_container_hwm_sum")]
    pub cspm_container_hwm_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro GCP hosts over all hours in the current months for all organizations.
    #[serde(rename = "cspm_gcp_host_top99p_sum")]
    pub cspm_gcp_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro hosts over all hours in the current months for all organizations.
    #[serde(rename = "cspm_host_top99p_sum")]
    pub cspm_host_top99p_sum: Option<i64>,
    /// Shows the average number of distinct historical custom metrics over all hours in the current months for all organizations.
    #[serde(rename = "custom_historical_ts_sum")]
    pub custom_historical_ts_sum: Option<i64>,
    /// Shows the average number of distinct live custom metrics over all hours in the current months for all organizations.
    #[serde(rename = "custom_live_ts_sum")]
    pub custom_live_ts_sum: Option<i64>,
    /// Shows the average number of distinct custom metrics over all hours in the current months for all organizations.
    #[serde(rename = "custom_ts_sum")]
    pub custom_ts_sum: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security containers over all hours in the current months for all organizations.
    #[serde(rename = "cws_containers_avg_sum")]
    pub cws_containers_avg_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Workload Security hosts over all hours in the current months for all organizations.
    #[serde(rename = "cws_host_top99p_sum")]
    pub cws_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Database Monitoring hosts over all hours in the current month for all organizations.
    #[serde(rename = "dbm_host_top99p_sum")]
    pub dbm_host_top99p_sum: Option<i64>,
    /// Shows the average of all distinct Database Monitoring Normalized Queries over all hours in the current month for all organizations.
    #[serde(rename = "dbm_queries_avg_sum")]
    pub dbm_queries_avg_sum: Option<i64>,
    /// Shows the last date of usage in the current months for all organizations.
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// Shows the average of all Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "fargate_tasks_count_avg_sum")]
    pub fargate_tasks_count_avg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of all Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "fargate_tasks_count_hwm_sum")]
    pub fargate_tasks_count_hwm_sum: Option<i64>,
    /// Shows the sum of all logs forwarding bytes over all hours in the current months for all organizations (data available as of April 1, 2023)
    #[serde(rename = "forwarding_events_bytes_agg_sum")]
    pub forwarding_events_bytes_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all GCP hosts over all hours in the current months for all organizations.
    #[serde(rename = "gcp_host_top99p_sum")]
    pub gcp_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Heroku dynos over all hours in the current months for all organizations.
    #[serde(rename = "heroku_host_top99p_sum")]
    pub heroku_host_top99p_sum: Option<i64>,
    /// Shows sum of the the high-water marks of incident management monthly active users in the current months for all organizations.
    #[serde(rename = "incident_management_monthly_active_users_hwm_sum")]
    pub incident_management_monthly_active_users_hwm_sum: Option<i64>,
    /// Shows the sum of all log events indexed over all hours in the current months for all organizations.
    #[serde(rename = "indexed_events_count_agg_sum")]
    pub indexed_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct infrastructure hosts over all hours in the current months for all organizations.
    #[serde(rename = "infra_host_top99p_sum")]
    pub infra_host_top99p_sum: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current months for all organizations.
    #[serde(rename = "ingested_events_bytes_agg_sum")]
    pub ingested_events_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all IoT devices over all hours in the current months for all organizations.
    #[serde(rename = "iot_device_agg_sum")]
    pub iot_device_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all IoT devices over all hours in the current months of all organizations.
    #[serde(rename = "iot_device_top99p_sum")]
    pub iot_device_top99p_sum: Option<i64>,
    /// Shows the the most recent hour in the current months for all organizations for which all usages were calculated.
    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,
    /// Shows the sum of all live logs indexed over all hours in the current months for all organizations (data available as of December 1, 2020).
    #[serde(rename = "live_indexed_events_agg_sum")]
    pub live_indexed_events_agg_sum: Option<i64>,
    /// Shows the sum of all live logs bytes ingested over all hours in the current months for all organizations (data available as of December 1, 2020).
    #[serde(rename = "live_ingested_bytes_agg_sum")]
    pub live_ingested_bytes_agg_sum: Option<i64>,
    /// Object containing logs usage data broken down by retention period.
    #[serde(rename = "logs_by_retention")]
    pub logs_by_retention: Option<crate::datadogV1::model::LogsByRetention>,
    /// Shows the sum of all mobile lite sessions over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_lite_session_count_agg_sum")]
    pub mobile_rum_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_agg_sum")]
    pub mobile_rum_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Android over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_android_agg_sum")]
    pub mobile_rum_session_count_android_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Flutter over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_flutter_agg_sum")]
    pub mobile_rum_session_count_flutter_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on iOS over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_ios_agg_sum")]
    pub mobile_rum_session_count_ios_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on React Native over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_reactnative_agg_sum")]
    pub mobile_rum_session_count_reactnative_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Roku over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_session_count_roku_agg_sum")]
    pub mobile_rum_session_count_roku_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM units over all hours in the current months for all organizations.
    #[serde(rename = "mobile_rum_units_agg_sum")]
    pub mobile_rum_units_agg_sum: Option<i64>,
    /// Shows the sum of all Network Device Monitoring NetFlow events over all hours in the current months for all organizations.
    #[serde(rename = "ndm_netflow_events_agg_sum")]
    pub ndm_netflow_events_agg_sum: Option<i64>,
    /// Shows the sum of all Network flows indexed over all hours in the current months for all organizations.
    #[serde(rename = "netflow_indexed_events_count_agg_sum")]
    pub netflow_indexed_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct Networks hosts over all hours in the current months for all organizations.
    #[serde(rename = "npm_host_top99p_sum")]
    pub npm_host_top99p_sum: Option<i64>,
    /// Sum of all observability pipelines bytes processed over all hours in the current months for all organizations.
    #[serde(rename = "observability_pipelines_bytes_processed_agg_sum")]
    pub observability_pipelines_bytes_processed_agg_sum: Option<i64>,
    /// Sum of all online archived events over all hours in the current months for all organizations.
    #[serde(rename = "online_archive_events_count_agg_sum")]
    pub online_archive_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current months for all organizations.
    #[serde(rename = "opentelemetry_apm_host_top99p_sum")]
    pub opentelemetry_apm_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current months for all organizations.
    #[serde(rename = "opentelemetry_host_top99p_sum")]
    pub opentelemetry_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all profiled Azure app services over all hours in the current months for all organizations.
    #[serde(rename = "profiling_aas_count_top99p_sum")]
    pub profiling_aas_count_top99p_sum: Option<i64>,
    /// Shows the average number of profiled containers over all hours in the current months for all organizations.
    #[serde(rename = "profiling_container_agent_count_avg")]
    pub profiling_container_agent_count_avg: Option<i64>,
    /// Shows the 99th percentile of all profiled hosts over all hours in the current months for all organizations.
    #[serde(rename = "profiling_host_count_top99p_sum")]
    pub profiling_host_count_top99p_sum: Option<i64>,
    /// Shows the sum of all rehydrated logs indexed over all hours in the current months for all organizations (data available as of December 1, 2020).
    #[serde(rename = "rehydrated_indexed_events_agg_sum")]
    pub rehydrated_indexed_events_agg_sum: Option<i64>,
    /// Shows the sum of all rehydrated logs bytes ingested over all hours in the current months for all organizations (data available as of December 1, 2020).
    #[serde(rename = "rehydrated_ingested_bytes_agg_sum")]
    pub rehydrated_ingested_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all mobile sessions and all browser lite and legacy sessions over all hours in the current month for all organizations.
    #[serde(rename = "rum_browser_and_mobile_session_count")]
    pub rum_browser_and_mobile_session_count: Option<i64>,
    /// Shows the sum of all browser RUM Lite Sessions over all hours in the current months for all organizations.
    #[serde(rename = "rum_session_count_agg_sum")]
    pub rum_session_count_agg_sum: Option<i64>,
    /// Shows the sum of RUM Sessions (browser and mobile) over all hours in the current months for all organizations.
    #[serde(rename = "rum_total_session_count_agg_sum")]
    pub rum_total_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser and mobile RUM units over all hours in the current months for all organizations.
    #[serde(rename = "rum_units_agg_sum")]
    pub rum_units_agg_sum: Option<i64>,
    /// Sum of all APM bytes scanned with sensitive data scanner in the current months for all organizations.
    #[serde(rename = "sds_apm_scanned_bytes_sum")]
    pub sds_apm_scanned_bytes_sum: Option<i64>,
    /// Sum of all event stream events bytes scanned with sensitive data scanner in the current months for all organizations.
    #[serde(rename = "sds_events_scanned_bytes_sum")]
    pub sds_events_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned of logs usage by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_logs_scanned_bytes_sum")]
    pub sds_logs_scanned_bytes_sum: Option<i64>,
    /// Sum of all RUM bytes scanned with sensitive data scanner in the current months for all organizations.
    #[serde(rename = "sds_rum_scanned_bytes_sum")]
    pub sds_rum_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned across all usage types by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_total_scanned_bytes_sum")]
    pub sds_total_scanned_bytes_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Azure in the current months for all organizations.
    #[serde(rename = "serverless_apps_azure_count_avg_sum")]
    pub serverless_apps_azure_count_avg_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Google Cloud in the current months for all organizations.
    #[serde(rename = "serverless_apps_google_count_avg_sum")]
    pub serverless_apps_google_count_avg_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Azure and Google Cloud in the current months for all organizations.
    #[serde(rename = "serverless_apps_total_count_avg_sum")]
    pub serverless_apps_total_count_avg_sum: Option<i64>,
    /// Shows the first date of usage in the current months for all organizations.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// Shows the sum of all Synthetic browser tests over all hours in the current months for all organizations.
    #[serde(rename = "synthetics_browser_check_calls_count_agg_sum")]
    pub synthetics_browser_check_calls_count_agg_sum: Option<i64>,
    /// Shows the sum of all Synthetic API tests over all hours in the current months for all organizations.
    #[serde(rename = "synthetics_check_calls_count_agg_sum")]
    pub synthetics_check_calls_count_agg_sum: Option<i64>,
    /// Shows the sum of Synthetic mobile application tests over all hours in the current months for all organizations.
    #[serde(rename = "synthetics_mobile_test_runs_agg_sum")]
    pub synthetics_mobile_test_runs_agg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of used synthetics parallel testing slots over all hours in the current month for all organizations.
    #[serde(rename = "synthetics_parallel_testing_max_slots_hwm_sum")]
    pub synthetics_parallel_testing_max_slots_hwm_sum: Option<i64>,
    /// Shows the sum of all Indexed Spans indexed over all hours in the current months for all organizations.
    #[serde(rename = "trace_search_indexed_events_count_agg_sum")]
    pub trace_search_indexed_events_count_agg_sum: Option<i64>,
    /// Shows the sum of all ingested APM span bytes over all hours in the current months for all organizations.
    #[serde(rename = "twol_ingested_events_bytes_agg_sum")]
    pub twol_ingested_events_bytes_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all Universal Service Monitoring hosts over all hours in the current months for all organizations.
    #[serde(rename = "universal_service_monitoring_host_top99p_sum")]
    pub universal_service_monitoring_host_top99p_sum: Option<i64>,
    /// An array of objects regarding hourly usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageSummaryDate>>,
    /// Shows the 99th percentile of all vSphere hosts over all hours in the current months for all organizations.
    #[serde(rename = "vsphere_host_top99p_sum")]
    pub vsphere_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Application Vulnerability Management hosts over all hours in the current months for all organizations.
    #[serde(rename = "vuln_management_host_count_top99p_sum")]
    pub vuln_management_host_count_top99p_sum: Option<i64>,
    /// Sum of all workflows executed over all hours in the current months for all organizations.
    #[serde(rename = "workflow_executions_usage_agg_sum")]
    pub workflow_executions_usage_agg_sum: Option<i64>,
}

impl UsageSummaryResponse {
    pub fn new() -> UsageSummaryResponse {
        #[allow(deprecated)]
        UsageSummaryResponse {
            agent_host_top99p_sum: None,
            apm_azure_app_service_host_top99p_sum: None,
            apm_fargate_count_avg_sum: None,
            apm_host_top99p_sum: None,
            appsec_fargate_count_avg_sum: None,
            audit_logs_lines_indexed_agg_sum: None,
            audit_trail_enabled_hwm_sum: None,
            avg_profiled_fargate_tasks_sum: None,
            aws_host_top99p_sum: None,
            aws_lambda_func_count: None,
            aws_lambda_invocations_sum: None,
            azure_app_service_top99p_sum: None,
            azure_host_top99p_sum: None,
            billable_ingested_bytes_agg_sum: None,
            browser_rum_lite_session_count_agg_sum: None,
            browser_rum_replay_session_count_agg_sum: None,
            browser_rum_units_agg_sum: None,
            ci_pipeline_indexed_spans_agg_sum: None,
            ci_test_indexed_spans_agg_sum: None,
            ci_visibility_itr_committers_hwm_sum: None,
            ci_visibility_pipeline_committers_hwm_sum: None,
            ci_visibility_test_committers_hwm_sum: None,
            cloud_cost_management_aws_host_count_avg_sum: None,
            cloud_cost_management_azure_host_count_avg_sum: None,
            cloud_cost_management_host_count_avg_sum: None,
            cloud_siem_events_agg_sum: None,
            container_avg_sum: None,
            container_excl_agent_avg_sum: None,
            container_hwm_sum: None,
            csm_container_enterprise_compliance_count_agg_sum: None,
            csm_container_enterprise_cws_count_agg_sum: None,
            csm_container_enterprise_total_count_agg_sum: None,
            csm_host_enterprise_aas_host_count_top99p_sum: None,
            csm_host_enterprise_aws_host_count_top99p_sum: None,
            csm_host_enterprise_azure_host_count_top99p_sum: None,
            csm_host_enterprise_compliance_host_count_top99p_sum: None,
            csm_host_enterprise_cws_host_count_top99p_sum: None,
            csm_host_enterprise_gcp_host_count_top99p_sum: None,
            csm_host_enterprise_total_host_count_top99p_sum: None,
            cspm_aas_host_top99p_sum: None,
            cspm_aws_host_top99p_sum: None,
            cspm_azure_host_top99p_sum: None,
            cspm_container_avg_sum: None,
            cspm_container_hwm_sum: None,
            cspm_gcp_host_top99p_sum: None,
            cspm_host_top99p_sum: None,
            custom_historical_ts_sum: None,
            custom_live_ts_sum: None,
            custom_ts_sum: None,
            cws_containers_avg_sum: None,
            cws_host_top99p_sum: None,
            dbm_host_top99p_sum: None,
            dbm_queries_avg_sum: None,
            end_date: None,
            fargate_tasks_count_avg_sum: None,
            fargate_tasks_count_hwm_sum: None,
            forwarding_events_bytes_agg_sum: None,
            gcp_host_top99p_sum: None,
            heroku_host_top99p_sum: None,
            incident_management_monthly_active_users_hwm_sum: None,
            indexed_events_count_agg_sum: None,
            infra_host_top99p_sum: None,
            ingested_events_bytes_agg_sum: None,
            iot_device_agg_sum: None,
            iot_device_top99p_sum: None,
            last_updated: None,
            live_indexed_events_agg_sum: None,
            live_ingested_bytes_agg_sum: None,
            logs_by_retention: None,
            mobile_rum_lite_session_count_agg_sum: None,
            mobile_rum_session_count_agg_sum: None,
            mobile_rum_session_count_android_agg_sum: None,
            mobile_rum_session_count_flutter_agg_sum: None,
            mobile_rum_session_count_ios_agg_sum: None,
            mobile_rum_session_count_reactnative_agg_sum: None,
            mobile_rum_session_count_roku_agg_sum: None,
            mobile_rum_units_agg_sum: None,
            ndm_netflow_events_agg_sum: None,
            netflow_indexed_events_count_agg_sum: None,
            npm_host_top99p_sum: None,
            observability_pipelines_bytes_processed_agg_sum: None,
            online_archive_events_count_agg_sum: None,
            opentelemetry_apm_host_top99p_sum: None,
            opentelemetry_host_top99p_sum: None,
            profiling_aas_count_top99p_sum: None,
            profiling_container_agent_count_avg: None,
            profiling_host_count_top99p_sum: None,
            rehydrated_indexed_events_agg_sum: None,
            rehydrated_ingested_bytes_agg_sum: None,
            rum_browser_and_mobile_session_count: None,
            rum_session_count_agg_sum: None,
            rum_total_session_count_agg_sum: None,
            rum_units_agg_sum: None,
            sds_apm_scanned_bytes_sum: None,
            sds_events_scanned_bytes_sum: None,
            sds_logs_scanned_bytes_sum: None,
            sds_rum_scanned_bytes_sum: None,
            sds_total_scanned_bytes_sum: None,
            serverless_apps_azure_count_avg_sum: None,
            serverless_apps_google_count_avg_sum: None,
            serverless_apps_total_count_avg_sum: None,
            start_date: None,
            synthetics_browser_check_calls_count_agg_sum: None,
            synthetics_check_calls_count_agg_sum: None,
            synthetics_mobile_test_runs_agg_sum: None,
            synthetics_parallel_testing_max_slots_hwm_sum: None,
            trace_search_indexed_events_count_agg_sum: None,
            twol_ingested_events_bytes_agg_sum: None,
            universal_service_monitoring_host_top99p_sum: None,
            usage: None,
            vsphere_host_top99p_sum: None,
            vuln_management_host_count_top99p_sum: None,
            workflow_executions_usage_agg_sum: None,
        }
    }

    #[allow(deprecated)]
    pub fn agent_host_top99p_sum(mut self, value: i64) -> Self {
        self.agent_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_azure_app_service_host_top99p_sum(mut self, value: i64) -> Self {
        self.apm_azure_app_service_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_fargate_count_avg_sum(mut self, value: i64) -> Self {
        self.apm_fargate_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_host_top99p_sum(mut self, value: i64) -> Self {
        self.apm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn appsec_fargate_count_avg_sum(mut self, value: i64) -> Self {
        self.appsec_fargate_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn audit_logs_lines_indexed_agg_sum(mut self, value: i64) -> Self {
        self.audit_logs_lines_indexed_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn audit_trail_enabled_hwm_sum(mut self, value: i64) -> Self {
        self.audit_trail_enabled_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn avg_profiled_fargate_tasks_sum(mut self, value: i64) -> Self {
        self.avg_profiled_fargate_tasks_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn aws_host_top99p_sum(mut self, value: i64) -> Self {
        self.aws_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn aws_lambda_func_count(mut self, value: i64) -> Self {
        self.aws_lambda_func_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn aws_lambda_invocations_sum(mut self, value: i64) -> Self {
        self.aws_lambda_invocations_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn azure_app_service_top99p_sum(mut self, value: i64) -> Self {
        self.azure_app_service_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn azure_host_top99p_sum(mut self, value: i64) -> Self {
        self.azure_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn billable_ingested_bytes_agg_sum(mut self, value: i64) -> Self {
        self.billable_ingested_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_lite_session_count_agg_sum(mut self, value: i64) -> Self {
        self.browser_rum_lite_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_replay_session_count_agg_sum(mut self, value: i64) -> Self {
        self.browser_rum_replay_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_units_agg_sum(mut self, value: i64) -> Self {
        self.browser_rum_units_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_pipeline_indexed_spans_agg_sum(mut self, value: i64) -> Self {
        self.ci_pipeline_indexed_spans_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_test_indexed_spans_agg_sum(mut self, value: i64) -> Self {
        self.ci_test_indexed_spans_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_itr_committers_hwm_sum(mut self, value: i64) -> Self {
        self.ci_visibility_itr_committers_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_pipeline_committers_hwm_sum(mut self, value: i64) -> Self {
        self.ci_visibility_pipeline_committers_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_test_committers_hwm_sum(mut self, value: i64) -> Self {
        self.ci_visibility_test_committers_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_aws_host_count_avg_sum(mut self, value: i64) -> Self {
        self.cloud_cost_management_aws_host_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_azure_host_count_avg_sum(mut self, value: i64) -> Self {
        self.cloud_cost_management_azure_host_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_host_count_avg_sum(mut self, value: i64) -> Self {
        self.cloud_cost_management_host_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_siem_events_agg_sum(mut self, value: i64) -> Self {
        self.cloud_siem_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_avg_sum(mut self, value: i64) -> Self {
        self.container_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_excl_agent_avg_sum(mut self, value: i64) -> Self {
        self.container_excl_agent_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_hwm_sum(mut self, value: i64) -> Self {
        self.container_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_compliance_count_agg_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_compliance_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_cws_count_agg_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_cws_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_total_count_agg_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_total_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_aas_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_aas_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_aws_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_aws_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_azure_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_azure_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_compliance_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_compliance_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_cws_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_cws_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_gcp_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_gcp_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_total_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.csm_host_enterprise_total_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_aas_host_top99p_sum(mut self, value: i64) -> Self {
        self.cspm_aas_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_aws_host_top99p_sum(mut self, value: i64) -> Self {
        self.cspm_aws_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_azure_host_top99p_sum(mut self, value: i64) -> Self {
        self.cspm_azure_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_container_avg_sum(mut self, value: i64) -> Self {
        self.cspm_container_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_container_hwm_sum(mut self, value: i64) -> Self {
        self.cspm_container_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_gcp_host_top99p_sum(mut self, value: i64) -> Self {
        self.cspm_gcp_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_host_top99p_sum(mut self, value: i64) -> Self {
        self.cspm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_historical_ts_sum(mut self, value: i64) -> Self {
        self.custom_historical_ts_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_live_ts_sum(mut self, value: i64) -> Self {
        self.custom_live_ts_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_ts_sum(mut self, value: i64) -> Self {
        self.custom_ts_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_containers_avg_sum(mut self, value: i64) -> Self {
        self.cws_containers_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_host_top99p_sum(mut self, value: i64) -> Self {
        self.cws_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn dbm_host_top99p_sum(mut self, value: i64) -> Self {
        self.dbm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn dbm_queries_avg_sum(mut self, value: i64) -> Self {
        self.dbm_queries_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn end_date(mut self, value: String) -> Self {
        self.end_date = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_tasks_count_avg_sum(mut self, value: i64) -> Self {
        self.fargate_tasks_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_tasks_count_hwm_sum(mut self, value: i64) -> Self {
        self.fargate_tasks_count_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn forwarding_events_bytes_agg_sum(mut self, value: i64) -> Self {
        self.forwarding_events_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn gcp_host_top99p_sum(mut self, value: i64) -> Self {
        self.gcp_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn heroku_host_top99p_sum(mut self, value: i64) -> Self {
        self.heroku_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn incident_management_monthly_active_users_hwm_sum(mut self, value: i64) -> Self {
        self.incident_management_monthly_active_users_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn indexed_events_count_agg_sum(mut self, value: i64) -> Self {
        self.indexed_events_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn infra_host_top99p_sum(mut self, value: i64) -> Self {
        self.infra_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ingested_events_bytes_agg_sum(mut self, value: i64) -> Self {
        self.ingested_events_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn iot_device_agg_sum(mut self, value: i64) -> Self {
        self.iot_device_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn iot_device_top99p_sum(mut self, value: i64) -> Self {
        self.iot_device_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn last_updated(mut self, value: String) -> Self {
        self.last_updated = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn live_indexed_events_agg_sum(mut self, value: i64) -> Self {
        self.live_indexed_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn live_ingested_bytes_agg_sum(mut self, value: i64) -> Self {
        self.live_ingested_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn logs_by_retention(mut self, value: crate::datadogV1::model::LogsByRetention) -> Self {
        self.logs_by_retention = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_lite_session_count_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_lite_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_android_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_android_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_flutter_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_flutter_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_ios_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_ios_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_reactnative_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_reactnative_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_roku_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_roku_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_units_agg_sum(mut self, value: i64) -> Self {
        self.mobile_rum_units_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ndm_netflow_events_agg_sum(mut self, value: i64) -> Self {
        self.ndm_netflow_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn netflow_indexed_events_count_agg_sum(mut self, value: i64) -> Self {
        self.netflow_indexed_events_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn npm_host_top99p_sum(mut self, value: i64) -> Self {
        self.npm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn observability_pipelines_bytes_processed_agg_sum(mut self, value: i64) -> Self {
        self.observability_pipelines_bytes_processed_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn online_archive_events_count_agg_sum(mut self, value: i64) -> Self {
        self.online_archive_events_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn opentelemetry_apm_host_top99p_sum(mut self, value: i64) -> Self {
        self.opentelemetry_apm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn opentelemetry_host_top99p_sum(mut self, value: i64) -> Self {
        self.opentelemetry_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn profiling_aas_count_top99p_sum(mut self, value: i64) -> Self {
        self.profiling_aas_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn profiling_container_agent_count_avg(mut self, value: i64) -> Self {
        self.profiling_container_agent_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn profiling_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.profiling_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rehydrated_indexed_events_agg_sum(mut self, value: i64) -> Self {
        self.rehydrated_indexed_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rehydrated_ingested_bytes_agg_sum(mut self, value: i64) -> Self {
        self.rehydrated_ingested_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_and_mobile_session_count(mut self, value: i64) -> Self {
        self.rum_browser_and_mobile_session_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_total_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_total_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_units_agg_sum(mut self, value: i64) -> Self {
        self.rum_units_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sds_apm_scanned_bytes_sum(mut self, value: i64) -> Self {
        self.sds_apm_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sds_events_scanned_bytes_sum(mut self, value: i64) -> Self {
        self.sds_events_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sds_logs_scanned_bytes_sum(mut self, value: i64) -> Self {
        self.sds_logs_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sds_rum_scanned_bytes_sum(mut self, value: i64) -> Self {
        self.sds_rum_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sds_total_scanned_bytes_sum(mut self, value: i64) -> Self {
        self.sds_total_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_azure_count_avg_sum(mut self, value: i64) -> Self {
        self.serverless_apps_azure_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_google_count_avg_sum(mut self, value: i64) -> Self {
        self.serverless_apps_google_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_total_count_avg_sum(mut self, value: i64) -> Self {
        self.serverless_apps_total_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_browser_check_calls_count_agg_sum(mut self, value: i64) -> Self {
        self.synthetics_browser_check_calls_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_check_calls_count_agg_sum(mut self, value: i64) -> Self {
        self.synthetics_check_calls_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_mobile_test_runs_agg_sum(mut self, value: i64) -> Self {
        self.synthetics_mobile_test_runs_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_parallel_testing_max_slots_hwm_sum(mut self, value: i64) -> Self {
        self.synthetics_parallel_testing_max_slots_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn trace_search_indexed_events_count_agg_sum(mut self, value: i64) -> Self {
        self.trace_search_indexed_events_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn twol_ingested_events_bytes_agg_sum(mut self, value: i64) -> Self {
        self.twol_ingested_events_bytes_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn universal_service_monitoring_host_top99p_sum(mut self, value: i64) -> Self {
        self.universal_service_monitoring_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageSummaryDate>) -> Self {
        self.usage = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn vsphere_host_top99p_sum(mut self, value: i64) -> Self {
        self.vsphere_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn vuln_management_host_count_top99p_sum(mut self, value: i64) -> Self {
        self.vuln_management_host_count_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn workflow_executions_usage_agg_sum(mut self, value: i64) -> Self {
        self.workflow_executions_usage_agg_sum = Some(value);
        self
    }
}

impl Default for UsageSummaryResponse {
    fn default() -> Self {
        Self::new()
    }
}
