// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSummaryDateOrg {
    /// Shows the 99th percentile of all agent hosts over all hours in the current date for the given org.
    #[serde(rename = "agent_host_top99p", skip_serializing_if = "Option::is_none")]
    pub agent_host_top99p: i64,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current date for the given org.
    #[serde(rename = "apm_azure_app_service_host_top99p", skip_serializing_if = "Option::is_none")]
    pub apm_azure_app_service_host_top99p: i64,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current months for the given org.
    #[serde(rename = "apm_fargate_count_avg", skip_serializing_if = "Option::is_none")]
    pub apm_fargate_count_avg: i64,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current date for the given org.
    #[serde(rename = "apm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub apm_host_top99p: i64,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current months for the given org.
    #[serde(rename = "appsec_fargate_count_avg", skip_serializing_if = "Option::is_none")]
    pub appsec_fargate_count_avg: i64,
    /// Shows the sum of all audit logs lines indexed over all hours in the current date for the given org.
    #[serde(rename = "audit_logs_lines_indexed_sum", skip_serializing_if = "Option::is_none")]
    pub audit_logs_lines_indexed_sum: i64,
    /// Shows whether Audit Trail is enabled for the current date for the given org.
    #[serde(rename = "audit_trail_enabled_hwm", skip_serializing_if = "Option::is_none")]
    pub audit_trail_enabled_hwm: i64,
    /// The average profiled task count for Fargate Profiling.
    #[serde(rename = "avg_profiled_fargate_tasks", skip_serializing_if = "Option::is_none")]
    pub avg_profiled_fargate_tasks: i64,
    /// Shows the 99th percentile of all AWS hosts over all hours in the current date for the given org.
    #[serde(rename = "aws_host_top99p", skip_serializing_if = "Option::is_none")]
    pub aws_host_top99p: i64,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current date for the given org.
    #[serde(rename = "aws_lambda_func_count", skip_serializing_if = "Option::is_none")]
    pub aws_lambda_func_count: i64,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current date for the given org.
    #[serde(rename = "aws_lambda_invocations_sum", skip_serializing_if = "Option::is_none")]
    pub aws_lambda_invocations_sum: i64,
    /// Shows the 99th percentile of all Azure app services over all hours in the current date for the given org.
    #[serde(rename = "azure_app_service_top99p", skip_serializing_if = "Option::is_none")]
    pub azure_app_service_top99p: i64,
    /// Shows the sum of all log bytes ingested over all hours in the current date for the given org.
    #[serde(rename = "billable_ingested_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub billable_ingested_bytes_sum: i64,
    /// Shows the sum of all browser lite sessions over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_lite_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub browser_rum_lite_session_count_sum: i64,
    /// Shows the sum of all browser replay sessions over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_replay_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub browser_rum_replay_session_count_sum: i64,
    /// Shows the sum of all browser RUM units over all hours in the current date for the given org.
    #[serde(rename = "browser_rum_units_sum", skip_serializing_if = "Option::is_none")]
    pub browser_rum_units_sum: i64,
    /// Shows the sum of all CI pipeline indexed spans over all hours in the current date for the given org.
    #[serde(rename = "ci_pipeline_indexed_spans_sum", skip_serializing_if = "Option::is_none")]
    pub ci_pipeline_indexed_spans_sum: i64,
    /// Shows the sum of all CI test indexed spans over all hours in the current date for the given org.
    #[serde(rename = "ci_test_indexed_spans_sum", skip_serializing_if = "Option::is_none")]
    pub ci_test_indexed_spans_sum: i64,
    /// Shows the high-water mark of all CI visibility intelligent test runner committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_itr_committers_hwm", skip_serializing_if = "Option::is_none")]
    pub ci_visibility_itr_committers_hwm: i64,
    /// Shows the high-water mark of all CI visibility pipeline committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_pipeline_committers_hwm", skip_serializing_if = "Option::is_none")]
    pub ci_visibility_pipeline_committers_hwm: i64,
    /// Shows the high-water mark of all CI visibility test committers over all hours in the current date for the given org.
    #[serde(rename = "ci_visibility_test_committers_hwm", skip_serializing_if = "Option::is_none")]
    pub ci_visibility_test_committers_hwm: i64,
    /// Host count average of Cloud Cost Management for AWS for the given date and given org.
    #[serde(rename = "cloud_cost_management_aws_host_count_avg", skip_serializing_if = "Option::is_none")]
    pub cloud_cost_management_aws_host_count_avg: i64,
    /// Host count average of Cloud Cost Management for Azure for the given date and given org.
    #[serde(rename = "cloud_cost_management_azure_host_count_avg", skip_serializing_if = "Option::is_none")]
    pub cloud_cost_management_azure_host_count_avg: i64,
    /// Host count average of Cloud Cost Management for all cloud providers for the given date and given org.
    #[serde(rename = "cloud_cost_management_host_count_avg", skip_serializing_if = "Option::is_none")]
    pub cloud_cost_management_host_count_avg: i64,
    /// Shows the average of all distinct containers over all hours in the current date for the given org.
    #[serde(rename = "container_avg", skip_serializing_if = "Option::is_none")]
    pub container_avg: i64,
    /// Shows the average of containers without the Datadog Agent over all hours in the current date for the given organization.
    #[serde(rename = "container_excl_agent_avg", skip_serializing_if = "Option::is_none")]
    pub container_excl_agent_avg: i64,
    /// Shows the high-water mark of all distinct containers over all hours in the current date for the given org.
    #[serde(rename = "container_hwm", skip_serializing_if = "Option::is_none")]
    pub container_hwm: i64,
    /// Shows the 99th percentile of all Cloud Security Posture Management Azure app services hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_aas_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_aas_host_top99p: i64,
    /// Shows the 99th percentile of all Cloud Security Posture Management AWS hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_aws_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_aws_host_top99p: i64,
    /// Shows the 99th percentile of all Cloud Security Posture Management Azure hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_azure_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_azure_host_top99p: i64,
    /// Shows the average number of Cloud Security Posture Management containers over all hours in the current date for the given org.
    #[serde(rename = "cspm_container_avg", skip_serializing_if = "Option::is_none")]
    pub cspm_container_avg: i64,
    /// Shows the high-water mark of Cloud Security Posture Management containers over all hours in the current date for the given org.
    #[serde(rename = "cspm_container_hwm", skip_serializing_if = "Option::is_none")]
    pub cspm_container_hwm: i64,
    /// Shows the 99th percentile of all Cloud Security Posture Management GCP hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_gcp_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_gcp_host_top99p: i64,
    /// Shows the 99th percentile of all Cloud Security Posture Management hosts over all hours in the current date for the given org.
    #[serde(rename = "cspm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_host_top99p: i64,
    /// Shows the average number of distinct custom metrics over all hours in the current date for the given org.
    #[serde(rename = "custom_ts_avg", skip_serializing_if = "Option::is_none")]
    pub custom_ts_avg: i64,
    /// Shows the average of all distinct Cloud Workload Security containers over all hours in the current date for the given org.
    #[serde(rename = "cws_container_count_avg", skip_serializing_if = "Option::is_none")]
    pub cws_container_count_avg: i64,
    /// Shows the 99th percentile of all Cloud Workload Security hosts over all hours in the current date for the given org.
    #[serde(rename = "cws_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cws_host_top99p: i64,
    /// Shows the 99th percentile of all Database Monitoring hosts over all hours in the current month for the given org.
    #[serde(rename = "dbm_host_top99p_sum", skip_serializing_if = "Option::is_none")]
    pub dbm_host_top99p_sum: i64,
    /// Shows the average of all distinct Database Monitoring normalized queries over all hours in the current month for the given org.
    #[serde(rename = "dbm_queries_avg_sum", skip_serializing_if = "Option::is_none")]
    pub dbm_queries_avg_sum: i64,
    /// The average task count for Fargate.
    #[serde(rename = "fargate_tasks_count_avg", skip_serializing_if = "Option::is_none")]
    pub fargate_tasks_count_avg: i64,
    /// Shows the high-water mark of all Fargate tasks over all hours in the current date for the given org.
    #[serde(rename = "fargate_tasks_count_hwm", skip_serializing_if = "Option::is_none")]
    pub fargate_tasks_count_hwm: i64,
    /// Shows the sum of all log bytes forwarded over all hours in the current date for the given org.
    #[serde(rename = "forwarding_events_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub forwarding_events_bytes_sum: i64,
    /// Shows the 99th percentile of all GCP hosts over all hours in the current date for the given org.
    #[serde(rename = "gcp_host_top99p", skip_serializing_if = "Option::is_none")]
    pub gcp_host_top99p: i64,
    /// Shows the 99th percentile of all Heroku dynos over all hours in the current date for the given org.
    #[serde(rename = "heroku_host_top99p", skip_serializing_if = "Option::is_none")]
    pub heroku_host_top99p: i64,
    /// The organization id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Shows the high-water mark of incident management monthly active users over all hours in the current date for the given org.
    #[serde(rename = "incident_management_monthly_active_users_hwm", skip_serializing_if = "Option::is_none")]
    pub incident_management_monthly_active_users_hwm: i64,
    /// Shows the sum of all log events indexed over all hours in the current date for the given org.
    #[serde(rename = "indexed_events_count_sum", skip_serializing_if = "Option::is_none")]
    pub indexed_events_count_sum: i64,
    /// Shows the 99th percentile of all distinct infrastructure hosts over all hours in the current date for the given org.
    #[serde(rename = "infra_host_top99p", skip_serializing_if = "Option::is_none")]
    pub infra_host_top99p: i64,
    /// Shows the sum of all log bytes ingested over all hours in the current date for the given org.
    #[serde(rename = "ingested_events_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub ingested_events_bytes_sum: i64,
    /// Shows the sum of all IoT devices over all hours in the current date for the given org.
    #[serde(rename = "iot_device_agg_sum", skip_serializing_if = "Option::is_none")]
    pub iot_device_agg_sum: i64,
    /// Shows the 99th percentile of all IoT devices over all hours in the current date for the given org.
    #[serde(rename = "iot_device_top99p_sum", skip_serializing_if = "Option::is_none")]
    pub iot_device_top99p_sum: i64,
    /// Shows the sum of all mobile lite sessions over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_lite_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_lite_session_count_sum: i64,
    /// Shows the sum of all mobile RUM Sessions on Android over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_android_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_android_sum: i64,
    /// Shows the sum of all mobile RUM Sessions on Flutter over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_flutter_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_flutter_sum: i64,
    /// Shows the sum of all mobile RUM Sessions on iOS over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_ios_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_ios_sum: i64,
    /// Shows the sum of all mobile RUM Sessions on React Native over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_reactnative_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_reactnative_sum: i64,
    /// Shows the sum of all mobile RUM Sessions on Roku over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_roku_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_roku_sum: i64,
    /// Shows the sum of all mobile RUM Sessions over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_session_count_sum: i64,
    /// Shows the sum of all mobile RUM units over all hours in the current date for the given org.
    #[serde(rename = "mobile_rum_units_sum", skip_serializing_if = "Option::is_none")]
    pub mobile_rum_units_sum: i64,
    /// The organization name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Shows the sum of all Network flows indexed over all hours in the current date for the given org.
    #[serde(rename = "netflow_indexed_events_count_sum", skip_serializing_if = "Option::is_none")]
    pub netflow_indexed_events_count_sum: i64,
    /// Shows the 99th percentile of all distinct Networks hosts over all hours in the current date for the given org.
    #[serde(rename = "npm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub npm_host_top99p: i64,
    /// Sum of all observability pipelines bytes processed over all hours in the current date for the given org.
    #[serde(rename = "observability_pipelines_bytes_processed_sum", skip_serializing_if = "Option::is_none")]
    pub observability_pipelines_bytes_processed_sum: i64,
    /// Sum of all online archived events over all hours in the current date for the given org.
    #[serde(rename = "online_archive_events_count_sum", skip_serializing_if = "Option::is_none")]
    pub online_archive_events_count_sum: i64,
    /// Shows the 99th percentile of APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "opentelemetry_apm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub opentelemetry_apm_host_top99p: i64,
    /// Shows the 99th percentile of all hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "opentelemetry_host_top99p", skip_serializing_if = "Option::is_none")]
    pub opentelemetry_host_top99p: i64,
    /// Shows the 99th percentile of all profiled hosts over all hours in the current date for the given org.
    #[serde(rename = "profiling_host_top99p", skip_serializing_if = "Option::is_none")]
    pub profiling_host_top99p: i64,
    /// The organization public id.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The region of the organization.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// Shows the sum of all mobile sessions and all browser lite and legacy sessions over all hours in the current date for the given org.
    #[serde(rename = "rum_browser_and_mobile_session_count", skip_serializing_if = "Option::is_none")]
    pub rum_browser_and_mobile_session_count: i64,
    /// Shows the sum of all browser RUM Lite Sessions over all hours in the current date for the given org.
    #[serde(rename = "rum_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub rum_session_count_sum: i64,
    /// Shows the sum of RUM Sessions (browser and mobile) over all hours in the current date for the given org.
    #[serde(rename = "rum_total_session_count_sum", skip_serializing_if = "Option::is_none")]
    pub rum_total_session_count_sum: i64,
    /// Shows the sum of all browser and mobile RUM units over all hours in the current date for the given org.
    #[serde(rename = "rum_units_sum", skip_serializing_if = "Option::is_none")]
    pub rum_units_sum: i64,
    /// Sum of all APM bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_apm_scanned_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub sds_apm_scanned_bytes_sum: i64,
    /// Sum of all event stream events bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_events_scanned_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub sds_events_scanned_bytes_sum: i64,
    /// Shows the sum of all bytes scanned of logs usage by the Sensitive Data Scanner over all hours in the current month for the given org.
    #[serde(rename = "sds_logs_scanned_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub sds_logs_scanned_bytes_sum: i64,
    /// Sum of all RUM bytes scanned with sensitive data scanner over all hours in the current date for the given org.
    #[serde(rename = "sds_rum_scanned_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub sds_rum_scanned_bytes_sum: i64,
    /// Shows the sum of all bytes scanned across all usage types by the Sensitive Data Scanner over all hours in the current month for the given org.
    #[serde(rename = "sds_total_scanned_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub sds_total_scanned_bytes_sum: i64,
    /// Shows the sum of all Synthetic browser tests over all hours in the current date for the given org.
    #[serde(rename = "synthetics_browser_check_calls_count_sum", skip_serializing_if = "Option::is_none")]
    pub synthetics_browser_check_calls_count_sum: i64,
    /// Shows the sum of all Synthetic API tests over all hours in the current date for the given org.
    #[serde(rename = "synthetics_check_calls_count_sum", skip_serializing_if = "Option::is_none")]
    pub synthetics_check_calls_count_sum: i64,
    /// Shows the high-water mark of used synthetics parallel testing slots over all hours in the current date for the given org.
    #[serde(rename = "synthetics_parallel_testing_max_slots_hwm", skip_serializing_if = "Option::is_none")]
    pub synthetics_parallel_testing_max_slots_hwm: i64,
    /// Shows the sum of all Indexed Spans indexed over all hours in the current date for the given org.
    #[serde(rename = "trace_search_indexed_events_count_sum", skip_serializing_if = "Option::is_none")]
    pub trace_search_indexed_events_count_sum: i64,
    /// Shows the sum of all ingested APM span bytes over all hours in the current date for the given org.
    #[serde(rename = "twol_ingested_events_bytes_sum", skip_serializing_if = "Option::is_none")]
    pub twol_ingested_events_bytes_sum: i64,
    /// Shows the 99th percentile of all Universal Service Monitoring hosts over all hours in the current date for the given org.
    #[serde(rename = "universal_service_monitoring_host_top99p", skip_serializing_if = "Option::is_none")]
    pub universal_service_monitoring_host_top99p: i64,
    /// Shows the 99th percentile of all vSphere hosts over all hours in the current date for the given org.
    #[serde(rename = "vsphere_host_top99p", skip_serializing_if = "Option::is_none")]
    pub vsphere_host_top99p: i64,
    /// Shows the 99th percentile of all Application Vulnerability Management hosts over all hours in the current date for the given org.
    #[serde(rename = "vuln_management_host_count_top99p", skip_serializing_if = "Option::is_none")]
    pub vuln_management_host_count_top99p: i64,
    /// Sum of all workflows executed over all hours in the current date for the given org.
    #[serde(rename = "workflow_executions_usage_sum", skip_serializing_if = "Option::is_none")]
    pub workflow_executions_usage_sum: i64,
}

