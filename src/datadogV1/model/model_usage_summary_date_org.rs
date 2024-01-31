// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Global hourly report of all data billed by Datadog for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSummaryDateOrg {
    /// Shows the 99th percentile of all agent hosts over all hours in the current date for the given org.
    #[serde(rename = "agent_host_top99p")]
    pub agent_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current date for the given org.
    #[serde(rename = "apm_azure_app_service_host_top99p")]
    pub apm_azure_app_service_host_top99p: Option<i64>,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current months for the given org.
    #[serde(rename = "apm_fargate_count_avg")]
    pub apm_fargate_count_avg: Option<i64>,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current date for the given org.
    #[serde(rename = "apm_host_top99p")]
    pub apm_host_top99p: Option<i64>,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current months for the given org.
    #[serde(rename = "appsec_fargate_count_avg")]
    pub appsec_fargate_count_avg: Option<i64>,
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
    /// The average task count for Fargate.
    #[serde(rename = "fargate_tasks_count_avg")]
    pub fargate_tasks_count_avg: Option<i64>,
    /// Shows the high-water mark of all Fargate tasks over all hours in the current date for the given org.
    #[serde(rename = "fargate_tasks_count_hwm")]
    pub fargate_tasks_count_hwm: Option<i64>,
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
}

impl UsageSummaryDateOrg {
    pub fn new() -> UsageSummaryDateOrg {
        #[allow(deprecated)]
        UsageSummaryDateOrg {
            agent_host_top99p: None,
            apm_azure_app_service_host_top99p: None,
            apm_fargate_count_avg: None,
            apm_host_top99p: None,
            appsec_fargate_count_avg: None,
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
            fargate_tasks_count_avg: None,
            fargate_tasks_count_hwm: None,
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
        }
    }

    #[allow(deprecated)]
    pub fn with_agent_host_top99p(&mut self, value: i64) -> &mut Self {
        self.agent_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_apm_azure_app_service_host_top99p(&mut self, value: i64) -> &mut Self {
        self.apm_azure_app_service_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_apm_fargate_count_avg(&mut self, value: i64) -> &mut Self {
        self.apm_fargate_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_apm_host_top99p(&mut self, value: i64) -> &mut Self {
        self.apm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_appsec_fargate_count_avg(&mut self, value: i64) -> &mut Self {
        self.appsec_fargate_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_audit_logs_lines_indexed_sum(&mut self, value: i64) -> &mut Self {
        self.audit_logs_lines_indexed_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_audit_trail_enabled_hwm(&mut self, value: i64) -> &mut Self {
        self.audit_trail_enabled_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_avg_profiled_fargate_tasks(&mut self, value: i64) -> &mut Self {
        self.avg_profiled_fargate_tasks = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_aws_host_top99p(&mut self, value: i64) -> &mut Self {
        self.aws_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_aws_lambda_func_count(&mut self, value: i64) -> &mut Self {
        self.aws_lambda_func_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_aws_lambda_invocations_sum(&mut self, value: i64) -> &mut Self {
        self.aws_lambda_invocations_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_azure_app_service_top99p(&mut self, value: i64) -> &mut Self {
        self.azure_app_service_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_billable_ingested_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.billable_ingested_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_browser_rum_lite_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.browser_rum_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_browser_rum_replay_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.browser_rum_replay_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_browser_rum_units_sum(&mut self, value: i64) -> &mut Self {
        self.browser_rum_units_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ci_pipeline_indexed_spans_sum(&mut self, value: i64) -> &mut Self {
        self.ci_pipeline_indexed_spans_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ci_test_indexed_spans_sum(&mut self, value: i64) -> &mut Self {
        self.ci_test_indexed_spans_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ci_visibility_itr_committers_hwm(&mut self, value: i64) -> &mut Self {
        self.ci_visibility_itr_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ci_visibility_pipeline_committers_hwm(&mut self, value: i64) -> &mut Self {
        self.ci_visibility_pipeline_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ci_visibility_test_committers_hwm(&mut self, value: i64) -> &mut Self {
        self.ci_visibility_test_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cloud_cost_management_aws_host_count_avg(&mut self, value: i64) -> &mut Self {
        self.cloud_cost_management_aws_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cloud_cost_management_azure_host_count_avg(&mut self, value: i64) -> &mut Self {
        self.cloud_cost_management_azure_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cloud_cost_management_host_count_avg(&mut self, value: i64) -> &mut Self {
        self.cloud_cost_management_host_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cloud_siem_events_sum(&mut self, value: i64) -> &mut Self {
        self.cloud_siem_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_container_avg(&mut self, value: i64) -> &mut Self {
        self.container_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_container_excl_agent_avg(&mut self, value: i64) -> &mut Self {
        self.container_excl_agent_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_container_hwm(&mut self, value: i64) -> &mut Self {
        self.container_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_container_enterprise_compliance_count_sum(&mut self, value: i64) -> &mut Self {
        self.csm_container_enterprise_compliance_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_container_enterprise_cws_count_sum(&mut self, value: i64) -> &mut Self {
        self.csm_container_enterprise_cws_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_container_enterprise_total_count_sum(&mut self, value: i64) -> &mut Self {
        self.csm_container_enterprise_total_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_aas_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_aas_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_aws_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_aws_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_azure_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_azure_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_compliance_host_count_top99p(
        &mut self,
        value: i64,
    ) -> &mut Self {
        self.csm_host_enterprise_compliance_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_cws_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_cws_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_gcp_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_gcp_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_csm_host_enterprise_total_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.csm_host_enterprise_total_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_aas_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cspm_aas_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_aws_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cspm_aws_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_azure_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cspm_azure_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_container_avg(&mut self, value: i64) -> &mut Self {
        self.cspm_container_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_container_hwm(&mut self, value: i64) -> &mut Self {
        self.cspm_container_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_gcp_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cspm_gcp_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cspm_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cspm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_custom_historical_ts_avg(&mut self, value: i64) -> &mut Self {
        self.custom_historical_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_custom_live_ts_avg(&mut self, value: i64) -> &mut Self {
        self.custom_live_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_custom_ts_avg(&mut self, value: i64) -> &mut Self {
        self.custom_ts_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cws_container_count_avg(&mut self, value: i64) -> &mut Self {
        self.cws_container_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_cws_host_top99p(&mut self, value: i64) -> &mut Self {
        self.cws_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_dbm_host_top99p_sum(&mut self, value: i64) -> &mut Self {
        self.dbm_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_dbm_queries_avg_sum(&mut self, value: i64) -> &mut Self {
        self.dbm_queries_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_fargate_tasks_count_avg(&mut self, value: i64) -> &mut Self {
        self.fargate_tasks_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_fargate_tasks_count_hwm(&mut self, value: i64) -> &mut Self {
        self.fargate_tasks_count_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_forwarding_events_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.forwarding_events_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_gcp_host_top99p(&mut self, value: i64) -> &mut Self {
        self.gcp_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_heroku_host_top99p(&mut self, value: i64) -> &mut Self {
        self.heroku_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_incident_management_monthly_active_users_hwm(&mut self, value: i64) -> &mut Self {
        self.incident_management_monthly_active_users_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_indexed_events_count_sum(&mut self, value: i64) -> &mut Self {
        self.indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_infra_host_top99p(&mut self, value: i64) -> &mut Self {
        self.infra_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ingested_events_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.ingested_events_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_iot_device_agg_sum(&mut self, value: i64) -> &mut Self {
        self.iot_device_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_iot_device_top99p_sum(&mut self, value: i64) -> &mut Self {
        self.iot_device_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_lite_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_android_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_android_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_flutter_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_flutter_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_ios_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_ios_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_reactnative_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_reactnative_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_roku_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_roku_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_mobile_rum_units_sum(&mut self, value: i64) -> &mut Self {
        self.mobile_rum_units_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_ndm_netflow_events_sum(&mut self, value: i64) -> &mut Self {
        self.ndm_netflow_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_netflow_indexed_events_count_sum(&mut self, value: i64) -> &mut Self {
        self.netflow_indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_npm_host_top99p(&mut self, value: i64) -> &mut Self {
        self.npm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_observability_pipelines_bytes_processed_sum(&mut self, value: i64) -> &mut Self {
        self.observability_pipelines_bytes_processed_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_online_archive_events_count_sum(&mut self, value: i64) -> &mut Self {
        self.online_archive_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_opentelemetry_apm_host_top99p(&mut self, value: i64) -> &mut Self {
        self.opentelemetry_apm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_opentelemetry_host_top99p(&mut self, value: i64) -> &mut Self {
        self.opentelemetry_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_profiling_aas_count_top99p(&mut self, value: i64) -> &mut Self {
        self.profiling_aas_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_profiling_host_top99p(&mut self, value: i64) -> &mut Self {
        self.profiling_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_rum_browser_and_mobile_session_count(&mut self, value: i64) -> &mut Self {
        self.rum_browser_and_mobile_session_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_rum_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.rum_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_rum_total_session_count_sum(&mut self, value: i64) -> &mut Self {
        self.rum_total_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_rum_units_sum(&mut self, value: i64) -> &mut Self {
        self.rum_units_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_sds_apm_scanned_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.sds_apm_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_sds_events_scanned_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.sds_events_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_sds_logs_scanned_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.sds_logs_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_sds_rum_scanned_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.sds_rum_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_sds_total_scanned_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.sds_total_scanned_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_serverless_apps_azure_count_avg(&mut self, value: i64) -> &mut Self {
        self.serverless_apps_azure_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_serverless_apps_google_count_avg(&mut self, value: i64) -> &mut Self {
        self.serverless_apps_google_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_serverless_apps_total_count_avg(&mut self, value: i64) -> &mut Self {
        self.serverless_apps_total_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_synthetics_browser_check_calls_count_sum(&mut self, value: i64) -> &mut Self {
        self.synthetics_browser_check_calls_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_synthetics_check_calls_count_sum(&mut self, value: i64) -> &mut Self {
        self.synthetics_check_calls_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_synthetics_mobile_test_runs_sum(&mut self, value: i64) -> &mut Self {
        self.synthetics_mobile_test_runs_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_synthetics_parallel_testing_max_slots_hwm(&mut self, value: i64) -> &mut Self {
        self.synthetics_parallel_testing_max_slots_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_trace_search_indexed_events_count_sum(&mut self, value: i64) -> &mut Self {
        self.trace_search_indexed_events_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_twol_ingested_events_bytes_sum(&mut self, value: i64) -> &mut Self {
        self.twol_ingested_events_bytes_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_universal_service_monitoring_host_top99p(&mut self, value: i64) -> &mut Self {
        self.universal_service_monitoring_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_vsphere_host_top99p(&mut self, value: i64) -> &mut Self {
        self.vsphere_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_vuln_management_host_count_top99p(&mut self, value: i64) -> &mut Self {
        self.vuln_management_host_count_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn with_workflow_executions_usage_sum(&mut self, value: i64) -> &mut Self {
        self.workflow_executions_usage_sum = Some(value);
        self
    }
}
impl Default for UsageSummaryDateOrg {
    fn default() -> Self {
        Self::new()
    }
}
