// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Global hourly report of all data billed by Datadog for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSummaryDateOrg {
    /// Shows the 99th percentile of all agent hosts over all hours in the current date for the given org.
    #[serde(rename = "agent_host_top99p")]
    pub agent_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current date for the given org.
    #[serde(rename = "apm_azure_app_service_host_top99p")]
    pub apm_azure_app_service_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all APM DevSecOps hosts over all hours in the current date for the given org.
    #[serde(rename = "apm_devsecops_host_top99p")]
    pub apm_devsecops_host_top99p: Option<i64>,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current month for the given org.
    #[serde(rename = "apm_fargate_count_avg")]
    pub apm_fargate_count_avg: Option<i64>,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current date for the given org.
    #[serde(rename = "apm_host_top99p")]
    pub apm_host_top99p: Option<i64>,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current month for the given org.
    #[serde(rename = "appsec_fargate_count_avg")]
    pub appsec_fargate_count_avg: Option<i64>,
    /// Shows the sum of all Application Security Monitoring Serverless invocations over all hours in the current month for the given org.
    #[serde(rename = "asm_serverless_sum")]
    pub asm_serverless_sum: Option<i64>,
    /// Shows the sum of all audit logs lines indexed over all hours in the current date for the given org.
    #[deprecated]
    #[serde(rename = "audit_logs_lines_indexed_sum")]
    pub audit_logs_lines_indexed_sum: Option<i64>,
    /// Shows whether Audit Trail is enabled for the current date for the given org.
    #[serde(rename = "audit_trail_enabled_hwm")]
    pub audit_trail_enabled_hwm: Option<i64>,
    /// The average profiled task count for Fargate Profiling.
    #[serde(rename = "avg_profiled_fargate_tasks")]
    pub avg_profiled_fargate_tasks: Option<i64>,
    /// Shows the 99th percentile of all AWS hosts over all hours in the current date for the given org.
    #[serde(rename = "aws_host_top99p")]
    pub aws_host_top99p: Option<i64>,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current date for the given org.
    #[serde(rename = "aws_lambda_func_count")]
    pub aws_lambda_func_count: Option<i64>,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current date for the given org.
    #[serde(rename = "aws_lambda_invocations_sum")]
    pub aws_lambda_invocations_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services over all hours in the current date for the given org.
    #[serde(rename = "azure_app_service_top99p")]
    pub azure_app_service_top99p: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current date for the given org.
    #[serde(rename = "billable_ingested_bytes_sum")]
    pub billable_ingested_bytes_sum: Option<i64>,
    /// Shows the sum of all browser lite sessions over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_lite_session_count_sum")]
    pub browser_rum_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all browser replay sessions over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_replay_session_count_sum")]
    pub browser_rum_replay_session_count_sum: Option<i64>,
    /// Shows the sum of all browser RUM units over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_units_sum")]
    pub browser_rum_units_sum: Option<i64>,
    /// Shows the sum of all CI pipeline indexed spans over all hours in the current date for the given org.
    #[serde(rename = "ci_pipeline_indexed_spans_sum")]
    pub ci_pipeline_indexed_spans_sum: Option<i64>,
    /// Shows the sum of all CI test indexed spans over all hours in the current date for the given org.
    #[serde(rename = "ci_test_indexed_spans_sum")]
    pub ci_test_indexed_spans_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility intelligent test runner committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_itr_committers_hwm")]
    pub ci_visibility_itr_committers_hwm: Option<i64>,
    /// Shows the high-water mark of all CI visibility pipeline committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_pipeline_committers_hwm")]
    pub ci_visibility_pipeline_committers_hwm: Option<i64>,
    /// Shows the high-water mark of all CI visibility test committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_test_committers_hwm")]
    pub ci_visibility_test_committers_hwm: Option<i64>,
    /// Host count average of Cloud Cost Management for AWS for the given date and given org.
    #[serde(rename = "cloud_cost_management_aws_host_count_avg")]
    pub cloud_cost_management_aws_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for Azure for the given date and given org.
    #[serde(rename = "cloud_cost_management_azure_host_count_avg")]
    pub cloud_cost_management_azure_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for GCP for the given date and given org.
    #[serde(rename = "cloud_cost_management_gcp_host_count_avg")]
    pub cloud_cost_management_gcp_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for all cloud providers for the given date and given org.
    #[serde(rename = "cloud_cost_management_host_count_avg")]
    pub cloud_cost_management_host_count_avg: Option<i64>,
    /// Shows the sum of all Cloud Security Information and Event Management events over all hours in the current date for the given org.
    #[serde(rename = "cloud_siem_events_sum")]
    pub cloud_siem_events_sum: Option<i64>,
    /// Shows the average of all distinct containers over all hours in the current date for the given org.
    #[serde(rename = "container_avg")]
    pub container_avg: Option<i64>,
    /// Shows the average of containers without the Datadog Agent over all hours in the current date for the given organization.
    #[serde(rename = "container_excl_agent_avg")]
    pub container_excl_agent_avg: Option<i64>,
    /// Shows the high-water mark of all distinct containers over all hours in the current date for the given org.
    #[serde(rename = "container_hwm")]
    pub container_hwm: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise compliance containers over all hours in the current date for the given org.
    #[serde(rename = "csm_container_enterprise_compliance_count_sum")]
    pub csm_container_enterprise_compliance_count_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise Cloud Workload Security containers over all hours in the current date for the given org.
    #[serde(rename = "csm_container_enterprise_cws_count_sum")]
    pub csm_container_enterprise_cws_count_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise containers over all hours in the current date for the given org.
    #[serde(rename = "csm_container_enterprise_total_count_sum")]
    pub csm_container_enterprise_total_count_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure app services hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_aas_host_count_top99p")]
    pub csm_host_enterprise_aas_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise AWS hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_aws_host_count_top99p")]
    pub csm_host_enterprise_aws_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_azure_host_count_top99p")]
    pub csm_host_enterprise_azure_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise compliance hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_compliance_host_count_top99p")]
    pub csm_host_enterprise_compliance_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Cloud Workload Security hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_cws_host_count_top99p")]
    pub csm_host_enterprise_cws_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise GCP hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_gcp_host_count_top99p")]
    pub csm_host_enterprise_gcp_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise hosts over all hours in the current date for the given org.
    #[serde(rename = "csm_host_enterprise_total_host_count_top99p")]
    pub csm_host_enterprise_total_host_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure app services hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_aas_host_top99p")]
    pub cspm_aas_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro AWS hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_aws_host_top99p")]
    pub cspm_aws_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_azure_host_top99p")]
    pub cspm_azure_host_top99p: Option<i64>,
    /// Shows the average number of Cloud Security Management Pro containers over all hours in the current date for the given org.
    #[serde(rename = "cspm_container_avg")]
    pub cspm_container_avg: Option<i64>,
    /// Shows the high-water mark of Cloud Security Management Pro containers over all hours in the current date for the given org.
    #[serde(rename = "cspm_container_hwm")]
    pub cspm_container_hwm: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro GCP hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_gcp_host_top99p")]
    pub cspm_gcp_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_host_top99p")]
    pub cspm_host_top99p: Option<i64>,
    /// Shows the average number of distinct historical custom metrics over all hours in the current date for the given org.
    #[serde(rename = "custom_historical_ts_avg")]
    pub custom_historical_ts_avg: Option<i64>,
    /// Shows the average number of distinct live custom metrics over all hours in the current date for the given org.
    #[serde(rename = "custom_live_ts_avg")]
    pub custom_live_ts_avg: Option<i64>,
    /// Shows the average number of distinct custom metrics over all hours in the current date for the given org.
    #[serde(rename = "custom_ts_avg")]
    pub custom_ts_avg: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security containers over all hours in the current date for the given org.
    #[serde(rename = "cws_container_count_avg")]
    pub cws_container_count_avg: Option<i64>,
    /// Shows the 99th percentile of all Cloud Workload Security hosts over all hours in the current date for the given org.
    #[serde(rename = "cws_host_top99p")]
    pub cws_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Database Monitoring hosts over all hours in the current month for the given org.
    #[serde(rename = "dbm_host_top99p_sum")]
    pub dbm_host_top99p_sum: Option<i64>,
    /// Shows the average of all distinct Database Monitoring normalized queries over all hours in the current month for the given org.
    #[serde(rename = "dbm_queries_avg_sum")]
    pub dbm_queries_avg_sum: Option<i64>,
    /// Shows the sum of all Error Tracking events over all hours in the current date for the given org.
    #[serde(rename = "error_tracking_events_sum")]
    pub error_tracking_events_sum: Option<i64>,
    /// The average task count for Fargate.
    #[serde(rename = "fargate_tasks_count_avg")]
    pub fargate_tasks_count_avg: Option<i64>,
    /// Shows the high-water mark of all Fargate tasks over all hours in the current date for the given org.
    #[serde(rename = "fargate_tasks_count_hwm")]
    pub fargate_tasks_count_hwm: Option<i64>,
    /// Shows the average number of Flex Logs Compute Large Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_large_avg")]
    pub flex_logs_compute_large_avg: Option<i64>,
    /// Shows the average number of Flex Logs Compute Medium Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_medium_avg")]
    pub flex_logs_compute_medium_avg: Option<i64>,
    /// Shows the average number of Flex Logs Compute Small Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_small_avg")]
    pub flex_logs_compute_small_avg: Option<i64>,
    /// Shows the average number of Flex Logs Compute Extra Small Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_xsmall_avg")]
    pub flex_logs_compute_xsmall_avg: Option<i64>,
    /// Shows the average of all Flex Stored Logs over all hours in the current date for the given org.
    #[serde(rename = "flex_stored_logs_avg")]
    pub flex_stored_logs_avg: Option<i64>,
    /// Shows the sum of all log bytes forwarded over all hours in the current date for the given org.
    #[serde(rename = "forwarding_events_bytes_sum")]
    pub forwarding_events_bytes_sum: Option<i64>,
    /// Shows the 99th percentile of all GCP hosts over all hours in the current date for the given org.
    #[serde(rename = "gcp_host_top99p")]
    pub gcp_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Heroku dynos over all hours in the current date for the given org.
    #[serde(rename = "heroku_host_top99p")]
    pub heroku_host_top99p: Option<i64>,
    /// The organization id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Shows the high-water mark of incident management monthly active users over all hours in the current date for the given org.
    #[serde(rename = "incident_management_monthly_active_users_hwm")]
    pub incident_management_monthly_active_users_hwm: Option<i64>,
    /// Shows the sum of all log events indexed over all hours in the current date for the given org.
    #[serde(rename = "indexed_events_count_sum")]
    pub indexed_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct infrastructure hosts over all hours in the current date for the given org.
    #[serde(rename = "infra_host_top99p")]
    pub infra_host_top99p: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current date for the given org.
    #[serde(rename = "ingested_events_bytes_sum")]
    pub ingested_events_bytes_sum: Option<i64>,
    /// Shows the sum of all IoT devices over all hours in the current date for the given org.
    #[serde(rename = "iot_device_agg_sum")]
    pub iot_device_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all IoT devices over all hours in the current date for the given org.
    #[serde(rename = "iot_device_top99p_sum")]
    pub iot_device_top99p_sum: Option<i64>,
    /// Shows the sum of all mobile lite sessions over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_lite_session_count_sum")]
    pub mobile_rum_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Android over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_android_sum")]
    pub mobile_rum_session_count_android_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Flutter over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_flutter_sum")]
    pub mobile_rum_session_count_flutter_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on iOS over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_ios_sum")]
    pub mobile_rum_session_count_ios_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on React Native over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_reactnative_sum")]
    pub mobile_rum_session_count_reactnative_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions on Roku over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_roku_sum")]
    pub mobile_rum_session_count_roku_sum: Option<i64>,
    /// Shows the sum of all mobile RUM Sessions over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_sum")]
    pub mobile_rum_session_count_sum: Option<i64>,
    /// Shows the sum of all mobile RUM units over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_units_sum")]
    pub mobile_rum_units_sum: Option<i64>,
    /// The organization name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Shows the sum of all Network Device Monitoring NetFlow events over all hours in the current date for the given org.
    #[serde(rename = "ndm_netflow_events_sum")]
    pub ndm_netflow_events_sum: Option<i64>,
    /// Shows the sum of all Network flows indexed over all hours in the current date for the given org.
    #[serde(rename = "netflow_indexed_events_count_sum")]
    pub netflow_indexed_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct Networks hosts over all hours in the current date for the given org.
    #[serde(rename = "npm_host_top99p")]
    pub npm_host_top99p: Option<i64>,
    /// Sum of all observability pipelines bytes processed over all hours in the current date for the given org.
    #[serde(rename = "observability_pipelines_bytes_processed_sum")]
    pub observability_pipelines_bytes_processed_sum: Option<i64>,
    /// Sum of all online archived events over all hours in the current date for the given org.
    #[serde(rename = "online_archive_events_count_sum")]
    pub online_archive_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "opentelemetry_apm_host_top99p")]
    pub opentelemetry_apm_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "opentelemetry_host_top99p")]
    pub opentelemetry_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all profiled Azure app services over all hours in the current date for all organizations.
    #[serde(rename = "profiling_aas_count_top99p")]
    pub profiling_aas_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all profiled hosts over all hours in the current date for the given org.
    #[serde(rename = "profiling_host_top99p")]
    pub profiling_host_top99p: Option<i64>,
    /// The organization public id.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the organization.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Shows the sum of all mobile sessions and all browser lite and legacy sessions over all hours in the current date for the given org.
    #[serde(rename = "rum_browser_and_mobile_session_count")]
    pub rum_browser_and_mobile_session_count: Option<i64>,
    /// Shows the sum of all browser RUM Lite Sessions over all hours in the current date for the given org.
    #[serde(rename = "rum_session_count_sum")]
    pub rum_session_count_sum: Option<i64>,
    /// Shows the sum of RUM Sessions (browser and mobile) over all hours in the current date for the given org.
    #[serde(rename = "rum_total_session_count_sum")]
    pub rum_total_session_count_sum: Option<i64>,
    /// Shows the sum of all browser and mobile RUM units over all hours in the current date for the given org.
    #[serde(rename = "rum_units_sum")]
    pub rum_units_sum: Option<i64>,
    /// Sum of all APM bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_apm_scanned_bytes_sum")]
    pub sds_apm_scanned_bytes_sum: Option<i64>,
    /// Sum of all event stream events bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_events_scanned_bytes_sum")]
    pub sds_events_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned of logs usage by the Sensitive Data Scanner over all hours in the current month for the given org.
    #[serde(rename = "sds_logs_scanned_bytes_sum")]
    pub sds_logs_scanned_bytes_sum: Option<i64>,
    /// Sum of all RUM bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_rum_scanned_bytes_sum")]
    pub sds_rum_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned across all usage types by the Sensitive Data Scanner over all hours in the current month for the given org.
    #[serde(rename = "sds_total_scanned_bytes_sum")]
    pub sds_total_scanned_bytes_sum: Option<i64>,
    /// Shows the average of the number of Serverless Apps for Azure for the given date and given org.
    #[serde(rename = "serverless_apps_azure_count_avg")]
    pub serverless_apps_azure_count_avg: Option<i64>,
    /// Shows the average of the number of Serverless Apps for Google Cloud for the given date and given org.
    #[serde(rename = "serverless_apps_google_count_avg")]
    pub serverless_apps_google_count_avg: Option<i64>,
    /// Shows the average of the number of Serverless Apps for Azure and Google Cloud for the given date and given org.
    #[serde(rename = "serverless_apps_total_count_avg")]
    pub serverless_apps_total_count_avg: Option<i64>,
    /// Shows the sum of all Synthetic browser tests over all hours in the current date for the given org.
    #[serde(rename = "synthetics_browser_check_calls_count_sum")]
    pub synthetics_browser_check_calls_count_sum: Option<i64>,
    /// Shows the sum of all Synthetic API tests over all hours in the current date for the given org.
    #[serde(rename = "synthetics_check_calls_count_sum")]
    pub synthetics_check_calls_count_sum: Option<i64>,
    /// Shows the sum of all Synthetic mobile application tests over all hours in the current date for the given org.
    #[serde(rename = "synthetics_mobile_test_runs_sum")]
    pub synthetics_mobile_test_runs_sum: Option<i64>,
    /// Shows the high-water mark of used synthetics parallel testing slots over all hours in the current date for the given org.
    #[serde(rename = "synthetics_parallel_testing_max_slots_hwm")]
    pub synthetics_parallel_testing_max_slots_hwm: Option<i64>,
    /// Shows the sum of all Indexed Spans indexed over all hours in the current date for the given org.
    #[serde(rename = "trace_search_indexed_events_count_sum")]
    pub trace_search_indexed_events_count_sum: Option<i64>,
    /// Shows the sum of all ingested APM span bytes over all hours in the current date for the given org.
    #[serde(rename = "twol_ingested_events_bytes_sum")]
    pub twol_ingested_events_bytes_sum: Option<i64>,
    /// Shows the 99th percentile of all Universal Service Monitoring hosts over all hours in the current date for the given org.
    #[serde(rename = "universal_service_monitoring_host_top99p")]
    pub universal_service_monitoring_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all vSphere hosts over all hours in the current date for the given org.
    #[serde(rename = "vsphere_host_top99p")]
    pub vsphere_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Application Vulnerability Management hosts over all hours in the current date for the given org.
    #[serde(rename = "vuln_management_host_count_top99p")]
    pub vuln_management_host_count_top99p: Option<i64>,
    /// Sum of all workflows executed over all hours in the current date for the given org.
    #[serde(rename = "workflow_executions_usage_sum")]
    pub workflow_executions_usage_sum: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSummaryDateOrg {
    pub fn new() -> UsageSummaryDateOrg {
        #[allow(deprecated)]
        UsageSummaryDateOrg {
            agent_host_top99p: None,
            apm_azure_app_service_host_top99p: None,
            apm_devsecops_host_top99p: None,
            apm_fargate_count_avg: None,
            apm_host_top99p: None,
            appsec_fargate_count_avg: None,
            asm_serverless_sum: None,
            audit_logs_lines_indexed_sum: None,
            audit_trail_enabled_hwm: None,
            avg_profiled_fargate_tasks: None,
            aws_host_top99p: None,
            aws_lambda_func_count: None,
            aws_lambda_invocations_sum: None,
            azure_app_service_top99p: None,
            billable_ingested_bytes_sum: None,
            browser_rum_lite_session_count_sum: None,
            browser_rum_replay_session_count_sum: None,
            browser_rum_units_sum: None,
            ci_pipeline_indexed_spans_sum: None,
            ci_test_indexed_spans_sum: None,
            ci_visibility_itr_committers_hwm: None,
            ci_visibility_pipeline_committers_hwm: None,
            ci_visibility_test_committers_hwm: None,
            cloud_cost_management_aws_host_count_avg: None,
            cloud_cost_management_azure_host_count_avg: None,
            cloud_cost_management_gcp_host_count_avg: None,
            cloud_cost_management_host_count_avg: None,
            cloud_siem_events_sum: None,
            container_avg: None,
            container_excl_agent_avg: None,
            container_hwm: None,
            csm_container_enterprise_compliance_count_sum: None,
            csm_container_enterprise_cws_count_sum: None,
            csm_container_enterprise_total_count_sum: None,
            csm_host_enterprise_aas_host_count_top99p: None,
            csm_host_enterprise_aws_host_count_top99p: None,
            csm_host_enterprise_azure_host_count_top99p: None,
            csm_host_enterprise_compliance_host_count_top99p: None,
            csm_host_enterprise_cws_host_count_top99p: None,
            csm_host_enterprise_gcp_host_count_top99p: None,
            csm_host_enterprise_total_host_count_top99p: None,
            cspm_aas_host_top99p: None,
            cspm_aws_host_top99p: None,
            cspm_azure_host_top99p: None,
            cspm_container_avg: None,
            cspm_container_hwm: None,
            cspm_gcp_host_top99p: None,
            cspm_host_top99p: None,
            custom_historical_ts_avg: None,
            custom_live_ts_avg: None,
            custom_ts_avg: None,
            cws_container_count_avg: None,
            cws_host_top99p: None,
            dbm_host_top99p_sum: None,
            dbm_queries_avg_sum: None,
            error_tracking_events_sum: None,
            fargate_tasks_count_avg: None,
            fargate_tasks_count_hwm: None,
            flex_logs_compute_large_avg: None,
            flex_logs_compute_medium_avg: None,
            flex_logs_compute_small_avg: None,
            flex_logs_compute_xsmall_avg: None,
            flex_stored_logs_avg: None,
            forwarding_events_bytes_sum: None,
            gcp_host_top99p: None,
            heroku_host_top99p: None,
            id: None,
            incident_management_monthly_active_users_hwm: None,
            indexed_events_count_sum: None,
            infra_host_top99p: None,
            ingested_events_bytes_sum: None,
            iot_device_agg_sum: None,
            iot_device_top99p_sum: None,
            mobile_rum_lite_session_count_sum: None,
            mobile_rum_session_count_android_sum: None,
            mobile_rum_session_count_flutter_sum: None,
            mobile_rum_session_count_ios_sum: None,
            mobile_rum_session_count_reactnative_sum: None,
            mobile_rum_session_count_roku_sum: None,
            mobile_rum_session_count_sum: None,
            mobile_rum_units_sum: None,
            name: None,
            ndm_netflow_events_sum: None,
            netflow_indexed_events_count_sum: None,
            npm_host_top99p: None,
            observability_pipelines_bytes_processed_sum: None,
            online_archive_events_count_sum: None,
            opentelemetry_apm_host_top99p: None,
            opentelemetry_host_top99p: None,
            profiling_aas_count_top99p: None,
            profiling_host_top99p: None,
            public_id: None,
            region: None,
            rum_browser_and_mobile_session_count: None,
            rum_session_count_sum: None,
            rum_total_session_count_sum: None,
            rum_units_sum: None,
            sds_apm_scanned_bytes_sum: None,
            sds_events_scanned_bytes_sum: None,
            sds_logs_scanned_bytes_sum: None,
            sds_rum_scanned_bytes_sum: None,
            sds_total_scanned_bytes_sum: None,
            serverless_apps_azure_count_avg: None,
            serverless_apps_google_count_avg: None,
            serverless_apps_total_count_avg: None,
            synthetics_browser_check_calls_count_sum: None,
            synthetics_check_calls_count_sum: None,
            synthetics_mobile_test_runs_sum: None,
            synthetics_parallel_testing_max_slots_hwm: None,
            trace_search_indexed_events_count_sum: None,
            twol_ingested_events_bytes_sum: None,
            universal_service_monitoring_host_top99p: None,
            vsphere_host_top99p: None,
            vuln_management_host_count_top99p: None,
            workflow_executions_usage_sum: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn agent_host_top99p(mut self, value: i64) -> Self {
        self.agent_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_azure_app_service_host_top99p(mut self, value: i64) -> Self {
        self.apm_azure_app_service_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_devsecops_host_top99p(mut self, value: i64) -> Self {
        self.apm_devsecops_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_fargate_count_avg(mut self, value: i64) -> Self {
        self.apm_fargate_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn apm_host_top99p(mut self, value: i64) -> Self {
        self.apm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn appsec_fargate_count_avg(mut self, value: i64) -> Self {
        self.appsec_fargate_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn asm_serverless_sum(mut self, value: i64) -> Self {
        self.asm_serverless_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn audit_logs_lines_indexed_sum(mut self, value: i64) -> Self {
        self.audit_logs_lines_indexed_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn audit_trail_enabled_hwm(mut self, value: i64) -> Self {
        self.audit_trail_enabled_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn avg_profiled_fargate_tasks(mut self, value: i64) -> Self {
        self.avg_profiled_fargate_tasks = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn aws_host_top99p(mut self, value: i64) -> Self {
        self.aws_host_top99p = Some(value);
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
    pub fn azure_app_service_top99p(mut self, value: i64) -> Self {
        self.azure_app_service_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn billable_ingested_bytes_sum(mut self, value: i64) -> Self {
        self.billable_ingested_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_lite_session_count_sum(mut self, value: i64) -> Self {
        self.browser_rum_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_replay_session_count_sum(mut self, value: i64) -> Self {
        self.browser_rum_replay_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn browser_rum_units_sum(mut self, value: i64) -> Self {
        self.browser_rum_units_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_pipeline_indexed_spans_sum(mut self, value: i64) -> Self {
        self.ci_pipeline_indexed_spans_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_test_indexed_spans_sum(mut self, value: i64) -> Self {
        self.ci_test_indexed_spans_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_itr_committers_hwm(mut self, value: i64) -> Self {
        self.ci_visibility_itr_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_pipeline_committers_hwm(mut self, value: i64) -> Self {
        self.ci_visibility_pipeline_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ci_visibility_test_committers_hwm(mut self, value: i64) -> Self {
        self.ci_visibility_test_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_aws_host_count_avg(mut self, value: i64) -> Self {
        self.cloud_cost_management_aws_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_azure_host_count_avg(mut self, value: i64) -> Self {
        self.cloud_cost_management_azure_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_gcp_host_count_avg(mut self, value: i64) -> Self {
        self.cloud_cost_management_gcp_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_cost_management_host_count_avg(mut self, value: i64) -> Self {
        self.cloud_cost_management_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_siem_events_sum(mut self, value: i64) -> Self {
        self.cloud_siem_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_avg(mut self, value: i64) -> Self {
        self.container_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_excl_agent_avg(mut self, value: i64) -> Self {
        self.container_excl_agent_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn container_hwm(mut self, value: i64) -> Self {
        self.container_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_compliance_count_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_compliance_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_cws_count_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_cws_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_container_enterprise_total_count_sum(mut self, value: i64) -> Self {
        self.csm_container_enterprise_total_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_aas_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_aas_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_aws_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_aws_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_azure_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_azure_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_compliance_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_compliance_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_cws_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_cws_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_gcp_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_gcp_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn csm_host_enterprise_total_host_count_top99p(mut self, value: i64) -> Self {
        self.csm_host_enterprise_total_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_aas_host_top99p(mut self, value: i64) -> Self {
        self.cspm_aas_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_aws_host_top99p(mut self, value: i64) -> Self {
        self.cspm_aws_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_azure_host_top99p(mut self, value: i64) -> Self {
        self.cspm_azure_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_container_avg(mut self, value: i64) -> Self {
        self.cspm_container_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_container_hwm(mut self, value: i64) -> Self {
        self.cspm_container_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_gcp_host_top99p(mut self, value: i64) -> Self {
        self.cspm_gcp_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_host_top99p(mut self, value: i64) -> Self {
        self.cspm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_historical_ts_avg(mut self, value: i64) -> Self {
        self.custom_historical_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_live_ts_avg(mut self, value: i64) -> Self {
        self.custom_live_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_ts_avg(mut self, value: i64) -> Self {
        self.custom_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_container_count_avg(mut self, value: i64) -> Self {
        self.cws_container_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_host_top99p(mut self, value: i64) -> Self {
        self.cws_host_top99p = Some(value);
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
    pub fn error_tracking_events_sum(mut self, value: i64) -> Self {
        self.error_tracking_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_tasks_count_avg(mut self, value: i64) -> Self {
        self.fargate_tasks_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_tasks_count_hwm(mut self, value: i64) -> Self {
        self.fargate_tasks_count_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_large_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_large_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_medium_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_medium_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_small_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_small_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_xsmall_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_xsmall_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_stored_logs_avg(mut self, value: i64) -> Self {
        self.flex_stored_logs_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn forwarding_events_bytes_sum(mut self, value: i64) -> Self {
        self.forwarding_events_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn gcp_host_top99p(mut self, value: i64) -> Self {
        self.gcp_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn heroku_host_top99p(mut self, value: i64) -> Self {
        self.heroku_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn incident_management_monthly_active_users_hwm(mut self, value: i64) -> Self {
        self.incident_management_monthly_active_users_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn indexed_events_count_sum(mut self, value: i64) -> Self {
        self.indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn infra_host_top99p(mut self, value: i64) -> Self {
        self.infra_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ingested_events_bytes_sum(mut self, value: i64) -> Self {
        self.ingested_events_bytes_sum = Some(value);
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
    pub fn mobile_rum_lite_session_count_sum(mut self, value: i64) -> Self {
        self.mobile_rum_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_android_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_android_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_flutter_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_flutter_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_ios_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_ios_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_reactnative_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_reactnative_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_roku_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_roku_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_session_count_sum(mut self, value: i64) -> Self {
        self.mobile_rum_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn mobile_rum_units_sum(mut self, value: i64) -> Self {
        self.mobile_rum_units_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn ndm_netflow_events_sum(mut self, value: i64) -> Self {
        self.ndm_netflow_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn netflow_indexed_events_count_sum(mut self, value: i64) -> Self {
        self.netflow_indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn npm_host_top99p(mut self, value: i64) -> Self {
        self.npm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn observability_pipelines_bytes_processed_sum(mut self, value: i64) -> Self {
        self.observability_pipelines_bytes_processed_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn online_archive_events_count_sum(mut self, value: i64) -> Self {
        self.online_archive_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn opentelemetry_apm_host_top99p(mut self, value: i64) -> Self {
        self.opentelemetry_apm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn opentelemetry_host_top99p(mut self, value: i64) -> Self {
        self.opentelemetry_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn profiling_aas_count_top99p(mut self, value: i64) -> Self {
        self.profiling_aas_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn profiling_host_top99p(mut self, value: i64) -> Self {
        self.profiling_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_and_mobile_session_count(mut self, value: i64) -> Self {
        self.rum_browser_and_mobile_session_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_session_count_sum(mut self, value: i64) -> Self {
        self.rum_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_total_session_count_sum(mut self, value: i64) -> Self {
        self.rum_total_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_units_sum(mut self, value: i64) -> Self {
        self.rum_units_sum = Some(value);
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
    pub fn serverless_apps_azure_count_avg(mut self, value: i64) -> Self {
        self.serverless_apps_azure_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_google_count_avg(mut self, value: i64) -> Self {
        self.serverless_apps_google_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_total_count_avg(mut self, value: i64) -> Self {
        self.serverless_apps_total_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_browser_check_calls_count_sum(mut self, value: i64) -> Self {
        self.synthetics_browser_check_calls_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_check_calls_count_sum(mut self, value: i64) -> Self {
        self.synthetics_check_calls_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_mobile_test_runs_sum(mut self, value: i64) -> Self {
        self.synthetics_mobile_test_runs_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_parallel_testing_max_slots_hwm(mut self, value: i64) -> Self {
        self.synthetics_parallel_testing_max_slots_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn trace_search_indexed_events_count_sum(mut self, value: i64) -> Self {
        self.trace_search_indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn twol_ingested_events_bytes_sum(mut self, value: i64) -> Self {
        self.twol_ingested_events_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn universal_service_monitoring_host_top99p(mut self, value: i64) -> Self {
        self.universal_service_monitoring_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn vsphere_host_top99p(mut self, value: i64) -> Self {
        self.vsphere_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn vuln_management_host_count_top99p(mut self, value: i64) -> Self {
        self.vuln_management_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn workflow_executions_usage_sum(mut self, value: i64) -> Self {
        self.workflow_executions_usage_sum = Some(value);
        self
    }
}

impl Default for UsageSummaryDateOrg {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSummaryDateOrg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSummaryDateOrgVisitor;
        impl<'a> Visitor<'a> for UsageSummaryDateOrgVisitor {
            type Value = UsageSummaryDateOrg;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_host_top99p: Option<i64> = None;
                let mut apm_azure_app_service_host_top99p: Option<i64> = None;
                let mut apm_devsecops_host_top99p: Option<i64> = None;
                let mut apm_fargate_count_avg: Option<i64> = None;
                let mut apm_host_top99p: Option<i64> = None;
                let mut appsec_fargate_count_avg: Option<i64> = None;
                let mut asm_serverless_sum: Option<i64> = None;
                let mut audit_logs_lines_indexed_sum: Option<i64> = None;
                let mut audit_trail_enabled_hwm: Option<i64> = None;
                let mut avg_profiled_fargate_tasks: Option<i64> = None;
                let mut aws_host_top99p: Option<i64> = None;
                let mut aws_lambda_func_count: Option<i64> = None;
                let mut aws_lambda_invocations_sum: Option<i64> = None;
                let mut azure_app_service_top99p: Option<i64> = None;
                let mut billable_ingested_bytes_sum: Option<i64> = None;
                let mut browser_rum_lite_session_count_sum: Option<i64> = None;
                let mut browser_rum_replay_session_count_sum: Option<i64> = None;
                let mut browser_rum_units_sum: Option<i64> = None;
                let mut ci_pipeline_indexed_spans_sum: Option<i64> = None;
                let mut ci_test_indexed_spans_sum: Option<i64> = None;
                let mut ci_visibility_itr_committers_hwm: Option<i64> = None;
                let mut ci_visibility_pipeline_committers_hwm: Option<i64> = None;
                let mut ci_visibility_test_committers_hwm: Option<i64> = None;
                let mut cloud_cost_management_aws_host_count_avg: Option<i64> = None;
                let mut cloud_cost_management_azure_host_count_avg: Option<i64> = None;
                let mut cloud_cost_management_gcp_host_count_avg: Option<i64> = None;
                let mut cloud_cost_management_host_count_avg: Option<i64> = None;
                let mut cloud_siem_events_sum: Option<i64> = None;
                let mut container_avg: Option<i64> = None;
                let mut container_excl_agent_avg: Option<i64> = None;
                let mut container_hwm: Option<i64> = None;
                let mut csm_container_enterprise_compliance_count_sum: Option<i64> = None;
                let mut csm_container_enterprise_cws_count_sum: Option<i64> = None;
                let mut csm_container_enterprise_total_count_sum: Option<i64> = None;
                let mut csm_host_enterprise_aas_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_aws_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_azure_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_compliance_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_cws_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_gcp_host_count_top99p: Option<i64> = None;
                let mut csm_host_enterprise_total_host_count_top99p: Option<i64> = None;
                let mut cspm_aas_host_top99p: Option<i64> = None;
                let mut cspm_aws_host_top99p: Option<i64> = None;
                let mut cspm_azure_host_top99p: Option<i64> = None;
                let mut cspm_container_avg: Option<i64> = None;
                let mut cspm_container_hwm: Option<i64> = None;
                let mut cspm_gcp_host_top99p: Option<i64> = None;
                let mut cspm_host_top99p: Option<i64> = None;
                let mut custom_historical_ts_avg: Option<i64> = None;
                let mut custom_live_ts_avg: Option<i64> = None;
                let mut custom_ts_avg: Option<i64> = None;
                let mut cws_container_count_avg: Option<i64> = None;
                let mut cws_host_top99p: Option<i64> = None;
                let mut dbm_host_top99p_sum: Option<i64> = None;
                let mut dbm_queries_avg_sum: Option<i64> = None;
                let mut error_tracking_events_sum: Option<i64> = None;
                let mut fargate_tasks_count_avg: Option<i64> = None;
                let mut fargate_tasks_count_hwm: Option<i64> = None;
                let mut flex_logs_compute_large_avg: Option<i64> = None;
                let mut flex_logs_compute_medium_avg: Option<i64> = None;
                let mut flex_logs_compute_small_avg: Option<i64> = None;
                let mut flex_logs_compute_xsmall_avg: Option<i64> = None;
                let mut flex_stored_logs_avg: Option<i64> = None;
                let mut forwarding_events_bytes_sum: Option<i64> = None;
                let mut gcp_host_top99p: Option<i64> = None;
                let mut heroku_host_top99p: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut incident_management_monthly_active_users_hwm: Option<i64> = None;
                let mut indexed_events_count_sum: Option<i64> = None;
                let mut infra_host_top99p: Option<i64> = None;
                let mut ingested_events_bytes_sum: Option<i64> = None;
                let mut iot_device_agg_sum: Option<i64> = None;
                let mut iot_device_top99p_sum: Option<i64> = None;
                let mut mobile_rum_lite_session_count_sum: Option<i64> = None;
                let mut mobile_rum_session_count_android_sum: Option<i64> = None;
                let mut mobile_rum_session_count_flutter_sum: Option<i64> = None;
                let mut mobile_rum_session_count_ios_sum: Option<i64> = None;
                let mut mobile_rum_session_count_reactnative_sum: Option<i64> = None;
                let mut mobile_rum_session_count_roku_sum: Option<i64> = None;
                let mut mobile_rum_session_count_sum: Option<i64> = None;
                let mut mobile_rum_units_sum: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut ndm_netflow_events_sum: Option<i64> = None;
                let mut netflow_indexed_events_count_sum: Option<i64> = None;
                let mut npm_host_top99p: Option<i64> = None;
                let mut observability_pipelines_bytes_processed_sum: Option<i64> = None;
                let mut online_archive_events_count_sum: Option<i64> = None;
                let mut opentelemetry_apm_host_top99p: Option<i64> = None;
                let mut opentelemetry_host_top99p: Option<i64> = None;
                let mut profiling_aas_count_top99p: Option<i64> = None;
                let mut profiling_host_top99p: Option<i64> = None;
                let mut public_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut rum_browser_and_mobile_session_count: Option<i64> = None;
                let mut rum_session_count_sum: Option<i64> = None;
                let mut rum_total_session_count_sum: Option<i64> = None;
                let mut rum_units_sum: Option<i64> = None;
                let mut sds_apm_scanned_bytes_sum: Option<i64> = None;
                let mut sds_events_scanned_bytes_sum: Option<i64> = None;
                let mut sds_logs_scanned_bytes_sum: Option<i64> = None;
                let mut sds_rum_scanned_bytes_sum: Option<i64> = None;
                let mut sds_total_scanned_bytes_sum: Option<i64> = None;
                let mut serverless_apps_azure_count_avg: Option<i64> = None;
                let mut serverless_apps_google_count_avg: Option<i64> = None;
                let mut serverless_apps_total_count_avg: Option<i64> = None;
                let mut synthetics_browser_check_calls_count_sum: Option<i64> = None;
                let mut synthetics_check_calls_count_sum: Option<i64> = None;
                let mut synthetics_mobile_test_runs_sum: Option<i64> = None;
                let mut synthetics_parallel_testing_max_slots_hwm: Option<i64> = None;
                let mut trace_search_indexed_events_count_sum: Option<i64> = None;
                let mut twol_ingested_events_bytes_sum: Option<i64> = None;
                let mut universal_service_monitoring_host_top99p: Option<i64> = None;
                let mut vsphere_host_top99p: Option<i64> = None;
                let mut vuln_management_host_count_top99p: Option<i64> = None;
                let mut workflow_executions_usage_sum: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_azure_app_service_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_azure_app_service_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_devsecops_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_devsecops_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_fargate_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_fargate_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_fargate_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "asm_serverless_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            asm_serverless_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audit_logs_lines_indexed_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_logs_lines_indexed_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audit_trail_enabled_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_trail_enabled_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_profiled_fargate_tasks" => {
                            if v.is_null() {
                                continue;
                            }
                            avg_profiled_fargate_tasks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_lambda_func_count" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_lambda_func_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_lambda_invocations_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_lambda_invocations_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_app_service_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_app_service_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billable_ingested_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            billable_ingested_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_lite_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_replay_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_replay_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_units_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_itr_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_itr_committers_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_pipeline_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_pipeline_committers_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_test_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_test_committers_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_aws_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_aws_host_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_azure_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_azure_host_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_gcp_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_gcp_host_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_host_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_siem_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_siem_events_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            container_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_excl_agent_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            container_excl_agent_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            container_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_compliance_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_compliance_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_cws_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_cws_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_total_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_total_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_aas_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aas_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_aws_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aws_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_azure_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_azure_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_compliance_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_compliance_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_cws_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_cws_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_gcp_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_gcp_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_total_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_total_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_aas_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aas_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_aws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aws_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_azure_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_azure_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_container_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_container_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_gcp_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_gcp_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_historical_ts_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_historical_ts_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_live_ts_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_live_ts_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_ts_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_ts_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_container_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_container_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_queries_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_queries_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_events_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_tasks_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_tasks_count_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_large_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_large_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_medium_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_medium_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_small_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_small_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_xsmall_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_xsmall_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_stored_logs_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_stored_logs_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forwarding_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            forwarding_events_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "heroku_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            heroku_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_monthly_active_users_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_monthly_active_users_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_events_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_events_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iot_device_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_device_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iot_device_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_device_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_lite_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_android_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_android_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_flutter_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_flutter_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_ios_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_ios_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_reactnative_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_reactnative_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_roku_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_roku_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_units_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ndm_netflow_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ndm_netflow_events_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "netflow_indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            netflow_indexed_events_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observability_pipelines_bytes_processed_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            observability_pipelines_bytes_processed_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_events_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_apm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiling_aas_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_aas_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiling_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_and_mobile_session_count" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_and_mobile_session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_total_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_total_session_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_units_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_apm_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_apm_scanned_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_events_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_events_scanned_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_logs_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_logs_scanned_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_rum_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_rum_scanned_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_total_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_total_scanned_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_azure_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_google_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_google_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_total_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_total_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_browser_check_calls_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_browser_check_calls_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_check_calls_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_check_calls_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_mobile_test_runs_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_mobile_test_runs_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_parallel_testing_max_slots_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_parallel_testing_max_slots_hwm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_search_indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_search_indexed_events_count_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twol_ingested_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            twol_ingested_events_bytes_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "universal_service_monitoring_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            universal_service_monitoring_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vsphere_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            vsphere_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_management_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_management_host_count_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow_executions_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_executions_usage_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                #[allow(deprecated)]
                let content = UsageSummaryDateOrg {
                    agent_host_top99p,
                    apm_azure_app_service_host_top99p,
                    apm_devsecops_host_top99p,
                    apm_fargate_count_avg,
                    apm_host_top99p,
                    appsec_fargate_count_avg,
                    asm_serverless_sum,
                    audit_logs_lines_indexed_sum,
                    audit_trail_enabled_hwm,
                    avg_profiled_fargate_tasks,
                    aws_host_top99p,
                    aws_lambda_func_count,
                    aws_lambda_invocations_sum,
                    azure_app_service_top99p,
                    billable_ingested_bytes_sum,
                    browser_rum_lite_session_count_sum,
                    browser_rum_replay_session_count_sum,
                    browser_rum_units_sum,
                    ci_pipeline_indexed_spans_sum,
                    ci_test_indexed_spans_sum,
                    ci_visibility_itr_committers_hwm,
                    ci_visibility_pipeline_committers_hwm,
                    ci_visibility_test_committers_hwm,
                    cloud_cost_management_aws_host_count_avg,
                    cloud_cost_management_azure_host_count_avg,
                    cloud_cost_management_gcp_host_count_avg,
                    cloud_cost_management_host_count_avg,
                    cloud_siem_events_sum,
                    container_avg,
                    container_excl_agent_avg,
                    container_hwm,
                    csm_container_enterprise_compliance_count_sum,
                    csm_container_enterprise_cws_count_sum,
                    csm_container_enterprise_total_count_sum,
                    csm_host_enterprise_aas_host_count_top99p,
                    csm_host_enterprise_aws_host_count_top99p,
                    csm_host_enterprise_azure_host_count_top99p,
                    csm_host_enterprise_compliance_host_count_top99p,
                    csm_host_enterprise_cws_host_count_top99p,
                    csm_host_enterprise_gcp_host_count_top99p,
                    csm_host_enterprise_total_host_count_top99p,
                    cspm_aas_host_top99p,
                    cspm_aws_host_top99p,
                    cspm_azure_host_top99p,
                    cspm_container_avg,
                    cspm_container_hwm,
                    cspm_gcp_host_top99p,
                    cspm_host_top99p,
                    custom_historical_ts_avg,
                    custom_live_ts_avg,
                    custom_ts_avg,
                    cws_container_count_avg,
                    cws_host_top99p,
                    dbm_host_top99p_sum,
                    dbm_queries_avg_sum,
                    error_tracking_events_sum,
                    fargate_tasks_count_avg,
                    fargate_tasks_count_hwm,
                    flex_logs_compute_large_avg,
                    flex_logs_compute_medium_avg,
                    flex_logs_compute_small_avg,
                    flex_logs_compute_xsmall_avg,
                    flex_stored_logs_avg,
                    forwarding_events_bytes_sum,
                    gcp_host_top99p,
                    heroku_host_top99p,
                    id,
                    incident_management_monthly_active_users_hwm,
                    indexed_events_count_sum,
                    infra_host_top99p,
                    ingested_events_bytes_sum,
                    iot_device_agg_sum,
                    iot_device_top99p_sum,
                    mobile_rum_lite_session_count_sum,
                    mobile_rum_session_count_android_sum,
                    mobile_rum_session_count_flutter_sum,
                    mobile_rum_session_count_ios_sum,
                    mobile_rum_session_count_reactnative_sum,
                    mobile_rum_session_count_roku_sum,
                    mobile_rum_session_count_sum,
                    mobile_rum_units_sum,
                    name,
                    ndm_netflow_events_sum,
                    netflow_indexed_events_count_sum,
                    npm_host_top99p,
                    observability_pipelines_bytes_processed_sum,
                    online_archive_events_count_sum,
                    opentelemetry_apm_host_top99p,
                    opentelemetry_host_top99p,
                    profiling_aas_count_top99p,
                    profiling_host_top99p,
                    public_id,
                    region,
                    rum_browser_and_mobile_session_count,
                    rum_session_count_sum,
                    rum_total_session_count_sum,
                    rum_units_sum,
                    sds_apm_scanned_bytes_sum,
                    sds_events_scanned_bytes_sum,
                    sds_logs_scanned_bytes_sum,
                    sds_rum_scanned_bytes_sum,
                    sds_total_scanned_bytes_sum,
                    serverless_apps_azure_count_avg,
                    serverless_apps_google_count_avg,
                    serverless_apps_total_count_avg,
                    synthetics_browser_check_calls_count_sum,
                    synthetics_check_calls_count_sum,
                    synthetics_mobile_test_runs_sum,
                    synthetics_parallel_testing_max_slots_hwm,
                    trace_search_indexed_events_count_sum,
                    twol_ingested_events_bytes_sum,
                    universal_service_monitoring_host_top99p,
                    vsphere_host_top99p,
                    vuln_management_host_count_top99p,
                    workflow_executions_usage_sum,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSummaryDateOrgVisitor)
    }
}
