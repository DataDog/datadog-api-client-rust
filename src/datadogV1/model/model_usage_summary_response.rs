// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response summarizing all usage aggregated across the months in the request for all organizations, and broken down by month and by organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSummaryResponse {
    /// Shows the 99th percentile of all agent hosts over all hours in the current month for all organizations.
    #[serde(rename = "agent_host_top99p_sum")]
    pub agent_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current month all organizations.
    #[serde(rename = "apm_azure_app_service_host_top99p_sum")]
    pub apm_azure_app_service_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all APM DevSecOps hosts over all hours in the current month for all organizations.
    #[serde(rename = "apm_devsecops_host_top99p_sum")]
    pub apm_devsecops_host_top99p_sum: Option<i64>,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "apm_fargate_count_avg_sum")]
    pub apm_fargate_count_avg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current month for all organizations.
    #[serde(rename = "apm_host_top99p_sum")]
    pub apm_host_top99p_sum: Option<i64>,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "appsec_fargate_count_avg_sum")]
    pub appsec_fargate_count_avg_sum: Option<i64>,
    /// Shows the sum of all Application Security Monitoring Serverless invocations over all hours in the current months for all organizations.
    #[serde(rename = "asm_serverless_agg_sum")]
    pub asm_serverless_agg_sum: Option<i64>,
    /// Shows the sum of all audit logs lines indexed over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "audit_logs_lines_indexed_agg_sum")]
    pub audit_logs_lines_indexed_agg_sum: Option<i64>,
    /// Shows the total number of organizations that had Audit Trail enabled over a specific number of months.
    #[serde(rename = "audit_trail_enabled_hwm_sum")]
    pub audit_trail_enabled_hwm_sum: Option<i64>,
    /// Shows the average of all profiled Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "avg_profiled_fargate_tasks_sum")]
    pub avg_profiled_fargate_tasks_sum: Option<i64>,
    /// Shows the 99th percentile of all AWS hosts over all hours in the current month for all organizations.
    #[serde(rename = "aws_host_top99p_sum")]
    pub aws_host_top99p_sum: Option<i64>,
    /// Shows the average of the number of functions that executed 1 or more times each hour in the current month for all organizations.
    #[serde(rename = "aws_lambda_func_count")]
    pub aws_lambda_func_count: Option<i64>,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current month for all organizations.
    #[serde(rename = "aws_lambda_invocations_sum")]
    pub aws_lambda_invocations_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services over all hours in the current month for all organizations.
    #[serde(rename = "azure_app_service_top99p_sum")]
    pub azure_app_service_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure hosts over all hours in the current month for all organizations.
    #[serde(rename = "azure_host_top99p_sum")]
    pub azure_host_top99p_sum: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current month for all organizations.
    #[serde(rename = "billable_ingested_bytes_agg_sum")]
    pub billable_ingested_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all browser lite sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "browser_rum_lite_session_count_agg_sum")]
    pub browser_rum_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser replay sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[serde(rename = "browser_rum_replay_session_count_agg_sum")]
    pub browser_rum_replay_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser RUM units over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "browser_rum_units_agg_sum")]
    pub browser_rum_units_agg_sum: Option<i64>,
    /// Shows the sum of all CI pipeline indexed spans over all hours in the current month for all organizations.
    #[serde(rename = "ci_pipeline_indexed_spans_agg_sum")]
    pub ci_pipeline_indexed_spans_agg_sum: Option<i64>,
    /// Shows the sum of all CI test indexed spans over all hours in the current month for all organizations.
    #[serde(rename = "ci_test_indexed_spans_agg_sum")]
    pub ci_test_indexed_spans_agg_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility intelligent test runner committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_itr_committers_hwm_sum")]
    pub ci_visibility_itr_committers_hwm_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility pipeline committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_pipeline_committers_hwm_sum")]
    pub ci_visibility_pipeline_committers_hwm_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility test committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_test_committers_hwm_sum")]
    pub ci_visibility_test_committers_hwm_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for AWS.
    #[serde(rename = "cloud_cost_management_aws_host_count_avg_sum")]
    pub cloud_cost_management_aws_host_count_avg_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for Azure.
    #[serde(rename = "cloud_cost_management_azure_host_count_avg_sum")]
    pub cloud_cost_management_azure_host_count_avg_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for GCP.
    #[serde(rename = "cloud_cost_management_gcp_host_count_avg_sum")]
    pub cloud_cost_management_gcp_host_count_avg_sum: Option<i64>,
    /// Sum of the host count average for Cloud Cost Management for all cloud providers.
    #[serde(rename = "cloud_cost_management_host_count_avg_sum")]
    pub cloud_cost_management_host_count_avg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Information and Event Management events over all hours in the current month for all organizations.
    #[serde(rename = "cloud_siem_events_agg_sum")]
    pub cloud_siem_events_agg_sum: Option<i64>,
    /// Shows the high-water mark of all Static Analysis committers over all hours in the current month for all organizations.
    #[serde(rename = "code_analysis_sa_committers_hwm_sum")]
    pub code_analysis_sa_committers_hwm_sum: Option<i64>,
    /// Shows the high-water mark of all static Software Composition Analysis committers over all hours in the current month for all organizations.
    #[serde(rename = "code_analysis_sca_committers_hwm_sum")]
    pub code_analysis_sca_committers_hwm_sum: Option<i64>,
    /// Shows the 99th percentile of all Code Security hosts over all hours in the current month for all organizations.
    #[serde(rename = "code_security_host_top99p_sum")]
    pub code_security_host_top99p_sum: Option<i64>,
    /// Shows the average of all distinct containers over all hours in the current month for all organizations.
    #[serde(rename = "container_avg_sum")]
    pub container_avg_sum: Option<i64>,
    /// Shows the average of the containers without the Datadog Agent over all hours in the current month for all organizations.
    #[serde(rename = "container_excl_agent_avg_sum")]
    pub container_excl_agent_avg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of all distinct containers over all hours in the current month for all organizations.
    #[serde(rename = "container_hwm_sum")]
    pub container_hwm_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise compliance containers over all hours in the current month for all organizations.
    #[serde(rename = "csm_container_enterprise_compliance_count_agg_sum")]
    pub csm_container_enterprise_compliance_count_agg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise Cloud Workload Security containers over all hours in the current month for all organizations.
    #[serde(rename = "csm_container_enterprise_cws_count_agg_sum")]
    pub csm_container_enterprise_cws_count_agg_sum: Option<i64>,
    /// Shows the sum of all Cloud Security Management Enterprise containers over all hours in the current month for all organizations.
    #[serde(rename = "csm_container_enterprise_total_count_agg_sum")]
    pub csm_container_enterprise_total_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure app services hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_aas_host_count_top99p_sum")]
    pub csm_host_enterprise_aas_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise AWS hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_aws_host_count_top99p_sum")]
    pub csm_host_enterprise_aws_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Azure hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_azure_host_count_top99p_sum")]
    pub csm_host_enterprise_azure_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise compliance hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_compliance_host_count_top99p_sum")]
    pub csm_host_enterprise_compliance_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise Cloud Workload Security hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_cws_host_count_top99p_sum")]
    pub csm_host_enterprise_cws_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise GCP hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_gcp_host_count_top99p_sum")]
    pub csm_host_enterprise_gcp_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Enterprise hosts over all hours in the current month for all organizations.
    #[serde(rename = "csm_host_enterprise_total_host_count_top99p_sum")]
    pub csm_host_enterprise_total_host_count_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure app services hosts over all hours in the current month for all organizations.
    #[serde(rename = "cspm_aas_host_top99p_sum")]
    pub cspm_aas_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro AWS hosts over all hours in the current month for all organizations.
    #[serde(rename = "cspm_aws_host_top99p_sum")]
    pub cspm_aws_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure hosts over all hours in the current month for all organizations.
    #[serde(rename = "cspm_azure_host_top99p_sum")]
    pub cspm_azure_host_top99p_sum: Option<i64>,
    /// Shows the average number of Cloud Security Management Pro containers over all hours in the current month for all organizations.
    #[serde(rename = "cspm_container_avg_sum")]
    pub cspm_container_avg_sum: Option<i64>,
    /// Shows the sum of the the high-water marks of Cloud Security Management Pro containers over all hours in the current month for all organizations.
    #[serde(rename = "cspm_container_hwm_sum")]
    pub cspm_container_hwm_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro GCP hosts over all hours in the current month for all organizations.
    #[serde(rename = "cspm_gcp_host_top99p_sum")]
    pub cspm_gcp_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro hosts over all hours in the current month for all organizations.
    #[serde(rename = "cspm_host_top99p_sum")]
    pub cspm_host_top99p_sum: Option<i64>,
    /// Shows the average number of distinct historical custom metrics over all hours in the current month for all organizations.
    #[serde(rename = "custom_historical_ts_sum")]
    pub custom_historical_ts_sum: Option<i64>,
    /// Shows the average number of distinct live custom metrics over all hours in the current month for all organizations.
    #[serde(rename = "custom_live_ts_sum")]
    pub custom_live_ts_sum: Option<i64>,
    /// Shows the average number of distinct custom metrics over all hours in the current month for all organizations.
    #[serde(rename = "custom_ts_sum")]
    pub custom_ts_sum: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security containers over all hours in the current month for all organizations.
    #[serde(rename = "cws_container_avg_sum")]
    pub cws_container_avg_sum: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "cws_fargate_task_avg_sum")]
    pub cws_fargate_task_avg_sum: Option<i64>,
    /// Shows the 99th percentile of all Cloud Workload Security hosts over all hours in the current month for all organizations.
    #[serde(rename = "cws_host_top99p_sum")]
    pub cws_host_top99p_sum: Option<i64>,
    /// Shows the sum of Data Jobs Monitoring hosts over all hours in the current months for all organizations
    #[serde(rename = "data_jobs_monitoring_host_hr_agg_sum")]
    pub data_jobs_monitoring_host_hr_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all Database Monitoring hosts over all hours in the current month for all organizations.
    #[serde(rename = "dbm_host_top99p_sum")]
    pub dbm_host_top99p_sum: Option<i64>,
    /// Shows the average of all distinct Database Monitoring Normalized Queries over all hours in the current month for all organizations.
    #[serde(rename = "dbm_queries_avg_sum")]
    pub dbm_queries_avg_sum: Option<i64>,
    /// Shows the last date of usage in the current month for all organizations.
    #[serde(rename = "end_date")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Shows the sum of all Error Tracking error events over all hours in the current month for all organizations.
    #[serde(rename = "error_tracking_error_events_agg_sum")]
    pub error_tracking_error_events_agg_sum: Option<i64>,
    /// Shows the sum of all Error Tracking events over all hours in the current months for all organizations.
    #[serde(rename = "error_tracking_events_agg_sum")]
    pub error_tracking_events_agg_sum: Option<i64>,
    /// Shows the sum of all Error Tracking RUM error events over all hours in the current month for all organizations.
    #[serde(rename = "error_tracking_rum_error_events_agg_sum")]
    pub error_tracking_rum_error_events_agg_sum: Option<i64>,
    /// Shows the average of all Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "fargate_tasks_count_avg_sum")]
    pub fargate_tasks_count_avg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of all Fargate tasks over all hours in the current month for all organizations.
    #[serde(rename = "fargate_tasks_count_hwm_sum")]
    pub fargate_tasks_count_hwm_sum: Option<i64>,
    /// Shows the average number of Flex Logs Compute Large Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_compute_large_avg_sum")]
    pub flex_logs_compute_large_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Compute Medium Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_compute_medium_avg_sum")]
    pub flex_logs_compute_medium_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Compute Small Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_compute_small_avg_sum")]
    pub flex_logs_compute_small_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Compute Extra Small Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_compute_xsmall_avg_sum")]
    pub flex_logs_compute_xsmall_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Starter Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_starter_avg_sum")]
    pub flex_logs_starter_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Starter Storage Index Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_starter_storage_index_avg_sum")]
    pub flex_logs_starter_storage_index_avg_sum: Option<i64>,
    /// Shows the average number of Flex Logs Starter Storage Retention Adjustment Instances over all hours in the current months for all organizations.
    #[serde(rename = "flex_logs_starter_storage_retention_adjustment_avg_sum")]
    pub flex_logs_starter_storage_retention_adjustment_avg_sum: Option<i64>,
    /// Shows the average of all Flex Stored Logs over all hours in the current months for all organizations.
    #[serde(rename = "flex_stored_logs_avg_sum")]
    pub flex_stored_logs_avg_sum: Option<i64>,
    /// Shows the sum of all logs forwarding bytes over all hours in the current month for all organizations (data available as of April 1, 2023)
    #[serde(rename = "forwarding_events_bytes_agg_sum")]
    pub forwarding_events_bytes_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all GCP hosts over all hours in the current month for all organizations.
    #[serde(rename = "gcp_host_top99p_sum")]
    pub gcp_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Heroku dynos over all hours in the current month for all organizations.
    #[serde(rename = "heroku_host_top99p_sum")]
    pub heroku_host_top99p_sum: Option<i64>,
    /// Shows sum of the the high-water marks of incident management monthly active users in the current month for all organizations.
    #[serde(rename = "incident_management_monthly_active_users_hwm_sum")]
    pub incident_management_monthly_active_users_hwm_sum: Option<i64>,
    /// Shows the sum of all log events indexed over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "indexed_events_count_agg_sum")]
    pub indexed_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct infrastructure hosts over all hours in the current month for all organizations.
    #[serde(rename = "infra_host_top99p_sum")]
    pub infra_host_top99p_sum: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current month for all organizations.
    #[serde(rename = "ingested_events_bytes_agg_sum")]
    pub ingested_events_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all IoT devices over all hours in the current month for all organizations.
    #[serde(rename = "iot_device_agg_sum")]
    pub iot_device_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all IoT devices over all hours in the current month of all organizations.
    #[serde(rename = "iot_device_top99p_sum")]
    pub iot_device_top99p_sum: Option<i64>,
    /// Shows the the most recent hour in the current month for all organizations for which all usages were calculated.
    #[serde(rename = "last_updated")]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// Shows the sum of all live logs indexed over all hours in the current month for all organization (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "live_indexed_events_agg_sum")]
    pub live_indexed_events_agg_sum: Option<i64>,
    /// Shows the sum of all live logs bytes ingested over all hours in the current month for all organizations (data available as of December 1, 2020).
    #[serde(rename = "live_ingested_bytes_agg_sum")]
    pub live_ingested_bytes_agg_sum: Option<i64>,
    /// Object containing logs usage data broken down by retention period.
    #[serde(rename = "logs_by_retention")]
    pub logs_by_retention: Option<crate::datadogV1::model::LogsByRetention>,
    /// Shows the sum of all mobile lite sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_lite_session_count_agg_sum")]
    pub mobile_rum_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_agg_sum")]
    pub mobile_rum_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Android over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_android_agg_sum")]
    pub mobile_rum_session_count_android_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Flutter over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_flutter_agg_sum")]
    pub mobile_rum_session_count_flutter_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on iOS over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_ios_agg_sum")]
    pub mobile_rum_session_count_ios_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on React Native over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_reactnative_agg_sum")]
    pub mobile_rum_session_count_reactnative_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Roku over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_roku_agg_sum")]
    pub mobile_rum_session_count_roku_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM units over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_units_agg_sum")]
    pub mobile_rum_units_agg_sum: Option<i64>,
    /// Shows the sum of all Network Device Monitoring NetFlow events over all hours in the current month for all organizations.
    #[serde(rename = "ndm_netflow_events_agg_sum")]
    pub ndm_netflow_events_agg_sum: Option<i64>,
    /// Shows the sum of all Network flows indexed over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "netflow_indexed_events_count_agg_sum")]
    pub netflow_indexed_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct Cloud Network Monitoring hosts (formerly known as Network hosts) over all hours in the current month for all organizations.
    #[serde(rename = "npm_host_top99p_sum")]
    pub npm_host_top99p_sum: Option<i64>,
    /// Sum of all observability pipelines bytes processed over all hours in the current month for all organizations.
    #[serde(rename = "observability_pipelines_bytes_processed_agg_sum")]
    pub observability_pipelines_bytes_processed_agg_sum: Option<i64>,
    /// Shows the sum of Oracle Cloud Infrastructure hosts over all hours in the current months for all organizations
    #[serde(rename = "oci_host_agg_sum")]
    pub oci_host_agg_sum: Option<i64>,
    /// Shows the 99th percentile of Oracle Cloud Infrastructure hosts over all hours in the current months for all organizations
    #[serde(rename = "oci_host_top99p_sum")]
    pub oci_host_top99p_sum: Option<i64>,
    /// Sum of all online archived events over all hours in the current month for all organizations.
    #[serde(rename = "online_archive_events_count_agg_sum")]
    pub online_archive_events_count_agg_sum: Option<i64>,
    /// Shows the 99th percentile of APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current month for all organizations.
    #[serde(rename = "opentelemetry_apm_host_top99p_sum")]
    pub opentelemetry_apm_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current month for all organizations.
    #[serde(rename = "opentelemetry_host_top99p_sum")]
    pub opentelemetry_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all profiled Azure app services over all hours in the current month for all organizations.
    #[serde(rename = "profiling_aas_count_top99p_sum")]
    pub profiling_aas_count_top99p_sum: Option<i64>,
    /// Shows the average number of profiled containers over all hours in the current month for all organizations.
    #[serde(rename = "profiling_container_agent_count_avg")]
    pub profiling_container_agent_count_avg: Option<i64>,
    /// Shows the 99th percentile of all profiled hosts over all hours in the current month for all organizations.
    #[serde(rename = "profiling_host_count_top99p_sum")]
    pub profiling_host_count_top99p_sum: Option<i64>,
    /// Shows the sum of all rehydrated logs indexed over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "rehydrated_indexed_events_agg_sum")]
    pub rehydrated_indexed_events_agg_sum: Option<i64>,
    /// Shows the sum of all rehydrated logs bytes ingested over all hours in the current month for all organizations (data available as of December 1, 2020).
    #[serde(rename = "rehydrated_ingested_bytes_agg_sum")]
    pub rehydrated_ingested_bytes_agg_sum: Option<i64>,
    /// Shows the sum of all mobile sessions and all browser lite and legacy sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[serde(rename = "rum_browser_and_mobile_session_count")]
    pub rum_browser_and_mobile_session_count: Option<i64>,
    /// Shows the sum of all browser RUM legacy sessions over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_legacy_session_count_agg_sum")]
    pub rum_browser_legacy_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser RUM lite sessions over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_lite_session_count_agg_sum")]
    pub rum_browser_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser RUM Session Replay counts over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_replay_session_count_agg_sum")]
    pub rum_browser_replay_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all RUM lite sessions (browser and mobile) over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_lite_session_count_agg_sum")]
    pub rum_lite_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on Android over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_android_agg_sum")]
    pub rum_mobile_legacy_session_count_android_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on Flutter over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_flutter_agg_sum")]
    pub rum_mobile_legacy_session_count_flutter_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on iOS over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_ios_agg_sum")]
    pub rum_mobile_legacy_session_count_ios_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on React Native over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_reactnative_agg_sum")]
    pub rum_mobile_legacy_session_count_reactnative_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on Roku over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_roku_agg_sum")]
    pub rum_mobile_legacy_session_count_roku_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Android over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_android_agg_sum")]
    pub rum_mobile_lite_session_count_android_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Flutter over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_flutter_agg_sum")]
    pub rum_mobile_lite_session_count_flutter_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on iOS over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_ios_agg_sum")]
    pub rum_mobile_lite_session_count_ios_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on React Native over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_reactnative_agg_sum")]
    pub rum_mobile_lite_session_count_reactnative_agg_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Roku over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_roku_agg_sum")]
    pub rum_mobile_lite_session_count_roku_agg_sum: Option<i64>,
    /// Shows the sum of all RUM Session Replay counts over all hours in the current month for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_replay_session_count_agg_sum")]
    pub rum_replay_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser RUM lite sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "rum_session_count_agg_sum")]
    pub rum_session_count_agg_sum: Option<i64>,
    /// Shows the sum of RUM sessions (browser and mobile) over all hours in the current month for all organizations.
    #[serde(rename = "rum_total_session_count_agg_sum")]
    pub rum_total_session_count_agg_sum: Option<i64>,
    /// Shows the sum of all browser and mobile RUM units over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "rum_units_agg_sum")]
    pub rum_units_agg_sum: Option<i64>,
    /// Shows the average of all Software Composition Analysis Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "sca_fargate_count_avg_sum")]
    pub sca_fargate_count_avg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of all Software Composition Analysis Fargate tasks over all hours in the current months for all organizations.
    #[serde(rename = "sca_fargate_count_hwm_sum")]
    pub sca_fargate_count_hwm_sum: Option<i64>,
    /// Sum of all APM bytes scanned with sensitive data scanner in the current month for all organizations.
    #[serde(rename = "sds_apm_scanned_bytes_sum")]
    pub sds_apm_scanned_bytes_sum: Option<i64>,
    /// Sum of all event stream events bytes scanned with sensitive data scanner in the current month for all organizations.
    #[serde(rename = "sds_events_scanned_bytes_sum")]
    pub sds_events_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned of logs usage by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_logs_scanned_bytes_sum")]
    pub sds_logs_scanned_bytes_sum: Option<i64>,
    /// Sum of all RUM bytes scanned with sensitive data scanner in the current month for all organizations.
    #[serde(rename = "sds_rum_scanned_bytes_sum")]
    pub sds_rum_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned across all usage types by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_total_scanned_bytes_sum")]
    pub sds_total_scanned_bytes_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Azure in the current month for all organizations.
    #[serde(rename = "serverless_apps_azure_count_avg_sum")]
    pub serverless_apps_azure_count_avg_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Google Cloud in the current month for all organizations.
    #[serde(rename = "serverless_apps_google_count_avg_sum")]
    pub serverless_apps_google_count_avg_sum: Option<i64>,
    /// Sum of the average number of Serverless Apps for Azure and Google Cloud in the current month for all organizations.
    #[serde(rename = "serverless_apps_total_count_avg_sum")]
    pub serverless_apps_total_count_avg_sum: Option<i64>,
    /// Shows the sum of all log events analyzed by Cloud SIEM over all hours in the current month for all organizations.
    #[serde(rename = "siem_analyzed_logs_add_on_count_agg_sum")]
    pub siem_analyzed_logs_add_on_count_agg_sum: Option<i64>,
    /// Shows the first date of usage in the current month for all organizations.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Shows the sum of all Synthetic browser tests over all hours in the current month for all organizations.
    #[serde(rename = "synthetics_browser_check_calls_count_agg_sum")]
    pub synthetics_browser_check_calls_count_agg_sum: Option<i64>,
    /// Shows the sum of all Synthetic API tests over all hours in the current month for all organizations.
    #[serde(rename = "synthetics_check_calls_count_agg_sum")]
    pub synthetics_check_calls_count_agg_sum: Option<i64>,
    /// Shows the sum of Synthetic mobile application tests over all hours in the current month for all organizations.
    #[serde(rename = "synthetics_mobile_test_runs_agg_sum")]
    pub synthetics_mobile_test_runs_agg_sum: Option<i64>,
    /// Shows the sum of the high-water marks of used synthetics parallel testing slots over all hours in the current month for all organizations.
    #[serde(rename = "synthetics_parallel_testing_max_slots_hwm_sum")]
    pub synthetics_parallel_testing_max_slots_hwm_sum: Option<i64>,
    /// Shows the sum of all Indexed Spans indexed over all hours in the current month for all organizations.
    #[serde(rename = "trace_search_indexed_events_count_agg_sum")]
    pub trace_search_indexed_events_count_agg_sum: Option<i64>,
    /// Shows the sum of all ingested APM span bytes over all hours in the current month for all organizations.
    #[serde(rename = "twol_ingested_events_bytes_agg_sum")]
    pub twol_ingested_events_bytes_agg_sum: Option<i64>,
    /// Shows the 99th percentile of all Universal Service Monitoring hosts over all hours in the current month for all organizations.
    #[serde(rename = "universal_service_monitoring_host_top99p_sum")]
    pub universal_service_monitoring_host_top99p_sum: Option<i64>,
    /// An array of objects regarding hourly usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageSummaryDate>>,
    /// Shows the 99th percentile of all vSphere hosts over all hours in the current month for all organizations.
    #[serde(rename = "vsphere_host_top99p_sum")]
    pub vsphere_host_top99p_sum: Option<i64>,
    /// Shows the 99th percentile of all Application Vulnerability Management hosts over all hours in the current month for all organizations.
    #[serde(rename = "vuln_management_host_count_top99p_sum")]
    pub vuln_management_host_count_top99p_sum: Option<i64>,
    /// Sum of all workflows executed over all hours in the current month for all organizations.
    #[serde(rename = "workflow_executions_usage_agg_sum")]
    pub workflow_executions_usage_agg_sum: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSummaryResponse {
    pub fn new() -> UsageSummaryResponse {
        #[allow(deprecated)]
        UsageSummaryResponse {
            agent_host_top99p_sum: None,
            apm_azure_app_service_host_top99p_sum: None,
            apm_devsecops_host_top99p_sum: None,
            apm_fargate_count_avg_sum: None,
            apm_host_top99p_sum: None,
            appsec_fargate_count_avg_sum: None,
            asm_serverless_agg_sum: None,
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
            cloud_cost_management_gcp_host_count_avg_sum: None,
            cloud_cost_management_host_count_avg_sum: None,
            cloud_siem_events_agg_sum: None,
            code_analysis_sa_committers_hwm_sum: None,
            code_analysis_sca_committers_hwm_sum: None,
            code_security_host_top99p_sum: None,
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
            cws_container_avg_sum: None,
            cws_fargate_task_avg_sum: None,
            cws_host_top99p_sum: None,
            data_jobs_monitoring_host_hr_agg_sum: None,
            dbm_host_top99p_sum: None,
            dbm_queries_avg_sum: None,
            end_date: None,
            error_tracking_error_events_agg_sum: None,
            error_tracking_events_agg_sum: None,
            error_tracking_rum_error_events_agg_sum: None,
            fargate_tasks_count_avg_sum: None,
            fargate_tasks_count_hwm_sum: None,
            flex_logs_compute_large_avg_sum: None,
            flex_logs_compute_medium_avg_sum: None,
            flex_logs_compute_small_avg_sum: None,
            flex_logs_compute_xsmall_avg_sum: None,
            flex_logs_starter_avg_sum: None,
            flex_logs_starter_storage_index_avg_sum: None,
            flex_logs_starter_storage_retention_adjustment_avg_sum: None,
            flex_stored_logs_avg_sum: None,
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
            oci_host_agg_sum: None,
            oci_host_top99p_sum: None,
            online_archive_events_count_agg_sum: None,
            opentelemetry_apm_host_top99p_sum: None,
            opentelemetry_host_top99p_sum: None,
            profiling_aas_count_top99p_sum: None,
            profiling_container_agent_count_avg: None,
            profiling_host_count_top99p_sum: None,
            rehydrated_indexed_events_agg_sum: None,
            rehydrated_ingested_bytes_agg_sum: None,
            rum_browser_and_mobile_session_count: None,
            rum_browser_legacy_session_count_agg_sum: None,
            rum_browser_lite_session_count_agg_sum: None,
            rum_browser_replay_session_count_agg_sum: None,
            rum_lite_session_count_agg_sum: None,
            rum_mobile_legacy_session_count_android_agg_sum: None,
            rum_mobile_legacy_session_count_flutter_agg_sum: None,
            rum_mobile_legacy_session_count_ios_agg_sum: None,
            rum_mobile_legacy_session_count_reactnative_agg_sum: None,
            rum_mobile_legacy_session_count_roku_agg_sum: None,
            rum_mobile_lite_session_count_android_agg_sum: None,
            rum_mobile_lite_session_count_flutter_agg_sum: None,
            rum_mobile_lite_session_count_ios_agg_sum: None,
            rum_mobile_lite_session_count_reactnative_agg_sum: None,
            rum_mobile_lite_session_count_roku_agg_sum: None,
            rum_replay_session_count_agg_sum: None,
            rum_session_count_agg_sum: None,
            rum_total_session_count_agg_sum: None,
            rum_units_agg_sum: None,
            sca_fargate_count_avg_sum: None,
            sca_fargate_count_hwm_sum: None,
            sds_apm_scanned_bytes_sum: None,
            sds_events_scanned_bytes_sum: None,
            sds_logs_scanned_bytes_sum: None,
            sds_rum_scanned_bytes_sum: None,
            sds_total_scanned_bytes_sum: None,
            serverless_apps_azure_count_avg_sum: None,
            serverless_apps_google_count_avg_sum: None,
            serverless_apps_total_count_avg_sum: None,
            siem_analyzed_logs_add_on_count_agg_sum: None,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
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
    pub fn apm_devsecops_host_top99p_sum(mut self, value: i64) -> Self {
        self.apm_devsecops_host_top99p_sum = Some(value);
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
    pub fn asm_serverless_agg_sum(mut self, value: i64) -> Self {
        self.asm_serverless_agg_sum = Some(value);
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
    pub fn cloud_cost_management_gcp_host_count_avg_sum(mut self, value: i64) -> Self {
        self.cloud_cost_management_gcp_host_count_avg_sum = Some(value);
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
    pub fn code_analysis_sa_committers_hwm_sum(mut self, value: i64) -> Self {
        self.code_analysis_sa_committers_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn code_analysis_sca_committers_hwm_sum(mut self, value: i64) -> Self {
        self.code_analysis_sca_committers_hwm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn code_security_host_top99p_sum(mut self, value: i64) -> Self {
        self.code_security_host_top99p_sum = Some(value);
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
    pub fn cws_container_avg_sum(mut self, value: i64) -> Self {
        self.cws_container_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_fargate_task_avg_sum(mut self, value: i64) -> Self {
        self.cws_fargate_task_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_host_top99p_sum(mut self, value: i64) -> Self {
        self.cws_host_top99p_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn data_jobs_monitoring_host_hr_agg_sum(mut self, value: i64) -> Self {
        self.data_jobs_monitoring_host_hr_agg_sum = Some(value);
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
    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_error_events_agg_sum(mut self, value: i64) -> Self {
        self.error_tracking_error_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_events_agg_sum(mut self, value: i64) -> Self {
        self.error_tracking_events_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_rum_error_events_agg_sum(mut self, value: i64) -> Self {
        self.error_tracking_rum_error_events_agg_sum = Some(value);
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
    pub fn flex_logs_compute_large_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_compute_large_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_medium_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_compute_medium_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_small_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_compute_small_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_xsmall_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_compute_xsmall_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_starter_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_storage_index_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_starter_storage_index_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_storage_retention_adjustment_avg_sum(mut self, value: i64) -> Self {
        self.flex_logs_starter_storage_retention_adjustment_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_stored_logs_avg_sum(mut self, value: i64) -> Self {
        self.flex_stored_logs_avg_sum = Some(value);
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
    pub fn last_updated(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
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
    pub fn oci_host_agg_sum(mut self, value: i64) -> Self {
        self.oci_host_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn oci_host_top99p_sum(mut self, value: i64) -> Self {
        self.oci_host_top99p_sum = Some(value);
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
    pub fn rum_browser_legacy_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_browser_legacy_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_lite_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_browser_lite_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_replay_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_browser_replay_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_lite_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_lite_session_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_android_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_android_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_flutter_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_flutter_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_ios_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_ios_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_reactnative_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_reactnative_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_roku_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_roku_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_android_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_android_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_flutter_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_flutter_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_ios_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_ios_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_reactnative_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_reactnative_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_roku_agg_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_roku_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_replay_session_count_agg_sum(mut self, value: i64) -> Self {
        self.rum_replay_session_count_agg_sum = Some(value);
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
    pub fn sca_fargate_count_avg_sum(mut self, value: i64) -> Self {
        self.sca_fargate_count_avg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sca_fargate_count_hwm_sum(mut self, value: i64) -> Self {
        self.sca_fargate_count_hwm_sum = Some(value);
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
    pub fn siem_analyzed_logs_add_on_count_agg_sum(mut self, value: i64) -> Self {
        self.siem_analyzed_logs_add_on_count_agg_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for UsageSummaryResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSummaryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSummaryResponseVisitor;
        impl<'a> Visitor<'a> for UsageSummaryResponseVisitor {
            type Value = UsageSummaryResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_host_top99p_sum: Option<i64> = None;
                let mut apm_azure_app_service_host_top99p_sum: Option<i64> = None;
                let mut apm_devsecops_host_top99p_sum: Option<i64> = None;
                let mut apm_fargate_count_avg_sum: Option<i64> = None;
                let mut apm_host_top99p_sum: Option<i64> = None;
                let mut appsec_fargate_count_avg_sum: Option<i64> = None;
                let mut asm_serverless_agg_sum: Option<i64> = None;
                let mut audit_logs_lines_indexed_agg_sum: Option<i64> = None;
                let mut audit_trail_enabled_hwm_sum: Option<i64> = None;
                let mut avg_profiled_fargate_tasks_sum: Option<i64> = None;
                let mut aws_host_top99p_sum: Option<i64> = None;
                let mut aws_lambda_func_count: Option<i64> = None;
                let mut aws_lambda_invocations_sum: Option<i64> = None;
                let mut azure_app_service_top99p_sum: Option<i64> = None;
                let mut azure_host_top99p_sum: Option<i64> = None;
                let mut billable_ingested_bytes_agg_sum: Option<i64> = None;
                let mut browser_rum_lite_session_count_agg_sum: Option<i64> = None;
                let mut browser_rum_replay_session_count_agg_sum: Option<i64> = None;
                let mut browser_rum_units_agg_sum: Option<i64> = None;
                let mut ci_pipeline_indexed_spans_agg_sum: Option<i64> = None;
                let mut ci_test_indexed_spans_agg_sum: Option<i64> = None;
                let mut ci_visibility_itr_committers_hwm_sum: Option<i64> = None;
                let mut ci_visibility_pipeline_committers_hwm_sum: Option<i64> = None;
                let mut ci_visibility_test_committers_hwm_sum: Option<i64> = None;
                let mut cloud_cost_management_aws_host_count_avg_sum: Option<i64> = None;
                let mut cloud_cost_management_azure_host_count_avg_sum: Option<i64> = None;
                let mut cloud_cost_management_gcp_host_count_avg_sum: Option<i64> = None;
                let mut cloud_cost_management_host_count_avg_sum: Option<i64> = None;
                let mut cloud_siem_events_agg_sum: Option<i64> = None;
                let mut code_analysis_sa_committers_hwm_sum: Option<i64> = None;
                let mut code_analysis_sca_committers_hwm_sum: Option<i64> = None;
                let mut code_security_host_top99p_sum: Option<i64> = None;
                let mut container_avg_sum: Option<i64> = None;
                let mut container_excl_agent_avg_sum: Option<i64> = None;
                let mut container_hwm_sum: Option<i64> = None;
                let mut csm_container_enterprise_compliance_count_agg_sum: Option<i64> = None;
                let mut csm_container_enterprise_cws_count_agg_sum: Option<i64> = None;
                let mut csm_container_enterprise_total_count_agg_sum: Option<i64> = None;
                let mut csm_host_enterprise_aas_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_aws_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_azure_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_compliance_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_cws_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_gcp_host_count_top99p_sum: Option<i64> = None;
                let mut csm_host_enterprise_total_host_count_top99p_sum: Option<i64> = None;
                let mut cspm_aas_host_top99p_sum: Option<i64> = None;
                let mut cspm_aws_host_top99p_sum: Option<i64> = None;
                let mut cspm_azure_host_top99p_sum: Option<i64> = None;
                let mut cspm_container_avg_sum: Option<i64> = None;
                let mut cspm_container_hwm_sum: Option<i64> = None;
                let mut cspm_gcp_host_top99p_sum: Option<i64> = None;
                let mut cspm_host_top99p_sum: Option<i64> = None;
                let mut custom_historical_ts_sum: Option<i64> = None;
                let mut custom_live_ts_sum: Option<i64> = None;
                let mut custom_ts_sum: Option<i64> = None;
                let mut cws_container_avg_sum: Option<i64> = None;
                let mut cws_fargate_task_avg_sum: Option<i64> = None;
                let mut cws_host_top99p_sum: Option<i64> = None;
                let mut data_jobs_monitoring_host_hr_agg_sum: Option<i64> = None;
                let mut dbm_host_top99p_sum: Option<i64> = None;
                let mut dbm_queries_avg_sum: Option<i64> = None;
                let mut end_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut error_tracking_error_events_agg_sum: Option<i64> = None;
                let mut error_tracking_events_agg_sum: Option<i64> = None;
                let mut error_tracking_rum_error_events_agg_sum: Option<i64> = None;
                let mut fargate_tasks_count_avg_sum: Option<i64> = None;
                let mut fargate_tasks_count_hwm_sum: Option<i64> = None;
                let mut flex_logs_compute_large_avg_sum: Option<i64> = None;
                let mut flex_logs_compute_medium_avg_sum: Option<i64> = None;
                let mut flex_logs_compute_small_avg_sum: Option<i64> = None;
                let mut flex_logs_compute_xsmall_avg_sum: Option<i64> = None;
                let mut flex_logs_starter_avg_sum: Option<i64> = None;
                let mut flex_logs_starter_storage_index_avg_sum: Option<i64> = None;
                let mut flex_logs_starter_storage_retention_adjustment_avg_sum: Option<i64> = None;
                let mut flex_stored_logs_avg_sum: Option<i64> = None;
                let mut forwarding_events_bytes_agg_sum: Option<i64> = None;
                let mut gcp_host_top99p_sum: Option<i64> = None;
                let mut heroku_host_top99p_sum: Option<i64> = None;
                let mut incident_management_monthly_active_users_hwm_sum: Option<i64> = None;
                let mut indexed_events_count_agg_sum: Option<i64> = None;
                let mut infra_host_top99p_sum: Option<i64> = None;
                let mut ingested_events_bytes_agg_sum: Option<i64> = None;
                let mut iot_device_agg_sum: Option<i64> = None;
                let mut iot_device_top99p_sum: Option<i64> = None;
                let mut last_updated: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut live_indexed_events_agg_sum: Option<i64> = None;
                let mut live_ingested_bytes_agg_sum: Option<i64> = None;
                let mut logs_by_retention: Option<crate::datadogV1::model::LogsByRetention> = None;
                let mut mobile_rum_lite_session_count_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_android_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_flutter_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_ios_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_reactnative_agg_sum: Option<i64> = None;
                let mut mobile_rum_session_count_roku_agg_sum: Option<i64> = None;
                let mut mobile_rum_units_agg_sum: Option<i64> = None;
                let mut ndm_netflow_events_agg_sum: Option<i64> = None;
                let mut netflow_indexed_events_count_agg_sum: Option<i64> = None;
                let mut npm_host_top99p_sum: Option<i64> = None;
                let mut observability_pipelines_bytes_processed_agg_sum: Option<i64> = None;
                let mut oci_host_agg_sum: Option<i64> = None;
                let mut oci_host_top99p_sum: Option<i64> = None;
                let mut online_archive_events_count_agg_sum: Option<i64> = None;
                let mut opentelemetry_apm_host_top99p_sum: Option<i64> = None;
                let mut opentelemetry_host_top99p_sum: Option<i64> = None;
                let mut profiling_aas_count_top99p_sum: Option<i64> = None;
                let mut profiling_container_agent_count_avg: Option<i64> = None;
                let mut profiling_host_count_top99p_sum: Option<i64> = None;
                let mut rehydrated_indexed_events_agg_sum: Option<i64> = None;
                let mut rehydrated_ingested_bytes_agg_sum: Option<i64> = None;
                let mut rum_browser_and_mobile_session_count: Option<i64> = None;
                let mut rum_browser_legacy_session_count_agg_sum: Option<i64> = None;
                let mut rum_browser_lite_session_count_agg_sum: Option<i64> = None;
                let mut rum_browser_replay_session_count_agg_sum: Option<i64> = None;
                let mut rum_lite_session_count_agg_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_android_agg_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_flutter_agg_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_ios_agg_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_reactnative_agg_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_roku_agg_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_android_agg_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_flutter_agg_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_ios_agg_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_reactnative_agg_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_roku_agg_sum: Option<i64> = None;
                let mut rum_replay_session_count_agg_sum: Option<i64> = None;
                let mut rum_session_count_agg_sum: Option<i64> = None;
                let mut rum_total_session_count_agg_sum: Option<i64> = None;
                let mut rum_units_agg_sum: Option<i64> = None;
                let mut sca_fargate_count_avg_sum: Option<i64> = None;
                let mut sca_fargate_count_hwm_sum: Option<i64> = None;
                let mut sds_apm_scanned_bytes_sum: Option<i64> = None;
                let mut sds_events_scanned_bytes_sum: Option<i64> = None;
                let mut sds_logs_scanned_bytes_sum: Option<i64> = None;
                let mut sds_rum_scanned_bytes_sum: Option<i64> = None;
                let mut sds_total_scanned_bytes_sum: Option<i64> = None;
                let mut serverless_apps_azure_count_avg_sum: Option<i64> = None;
                let mut serverless_apps_google_count_avg_sum: Option<i64> = None;
                let mut serverless_apps_total_count_avg_sum: Option<i64> = None;
                let mut siem_analyzed_logs_add_on_count_agg_sum: Option<i64> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut synthetics_browser_check_calls_count_agg_sum: Option<i64> = None;
                let mut synthetics_check_calls_count_agg_sum: Option<i64> = None;
                let mut synthetics_mobile_test_runs_agg_sum: Option<i64> = None;
                let mut synthetics_parallel_testing_max_slots_hwm_sum: Option<i64> = None;
                let mut trace_search_indexed_events_count_agg_sum: Option<i64> = None;
                let mut twol_ingested_events_bytes_agg_sum: Option<i64> = None;
                let mut universal_service_monitoring_host_top99p_sum: Option<i64> = None;
                let mut usage: Option<Vec<crate::datadogV1::model::UsageSummaryDate>> = None;
                let mut vsphere_host_top99p_sum: Option<i64> = None;
                let mut vuln_management_host_count_top99p_sum: Option<i64> = None;
                let mut workflow_executions_usage_agg_sum: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_azure_app_service_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_azure_app_service_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_devsecops_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_devsecops_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_fargate_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_fargate_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_fargate_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "asm_serverless_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            asm_serverless_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audit_logs_lines_indexed_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_logs_lines_indexed_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audit_trail_enabled_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_trail_enabled_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_profiled_fargate_tasks_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            avg_profiled_fargate_tasks_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_host_top99p_sum =
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
                        "azure_app_service_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_app_service_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billable_ingested_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            billable_ingested_bytes_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_lite_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_lite_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_replay_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_replay_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_rum_units_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_units_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_indexed_spans_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_itr_committers_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_itr_committers_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_pipeline_committers_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_pipeline_committers_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_test_committers_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_test_committers_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_aws_host_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_aws_host_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_azure_host_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_azure_host_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_gcp_host_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_gcp_host_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_host_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_host_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_siem_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_siem_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_analysis_sa_committers_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis_sa_committers_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_analysis_sca_committers_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis_sca_committers_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code_security_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            code_security_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            container_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_excl_agent_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            container_excl_agent_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            container_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_compliance_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_compliance_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_cws_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_cws_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_container_enterprise_total_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_total_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_aas_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aas_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_aws_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aws_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_azure_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_azure_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_compliance_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_compliance_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_cws_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_cws_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_gcp_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_gcp_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "csm_host_enterprise_total_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_total_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_aas_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aas_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_aws_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aws_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_azure_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_azure_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_container_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_container_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_gcp_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_gcp_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_historical_ts_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_historical_ts_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_live_ts_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_live_ts_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_ts_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_ts_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_container_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_container_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_fargate_task_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_fargate_task_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_jobs_monitoring_host_hr_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            data_jobs_monitoring_host_hr_agg_sum =
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
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_error_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_error_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_rum_error_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_rum_error_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_tasks_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_tasks_count_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_large_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_large_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_medium_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_medium_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_small_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_small_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_compute_xsmall_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_xsmall_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_starter_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_starter_storage_index_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_storage_index_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_logs_starter_storage_retention_adjustment_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_storage_retention_adjustment_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flex_stored_logs_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_stored_logs_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forwarding_events_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            forwarding_events_bytes_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "heroku_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            heroku_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_monthly_active_users_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_monthly_active_users_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_events_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_events_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_events_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_events_bytes_agg_sum =
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
                        "last_updated" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live_indexed_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            live_indexed_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live_ingested_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            live_ingested_bytes_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_by_retention" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_by_retention =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_lite_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_lite_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_android_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_android_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_flutter_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_flutter_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_ios_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_ios_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_reactnative_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_reactnative_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_session_count_roku_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_roku_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_units_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_units_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ndm_netflow_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ndm_netflow_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "netflow_indexed_events_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            netflow_indexed_events_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observability_pipelines_bytes_processed_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            observability_pipelines_bytes_processed_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "oci_host_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            oci_host_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "oci_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            oci_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_events_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_events_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_apm_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_apm_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiling_aas_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_aas_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiling_container_agent_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_container_agent_count_avg =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiling_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rehydrated_indexed_events_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rehydrated_indexed_events_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rehydrated_ingested_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rehydrated_ingested_bytes_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_and_mobile_session_count" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_and_mobile_session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_legacy_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_legacy_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_lite_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_lite_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_replay_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_replay_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_lite_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_lite_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_legacy_session_count_android_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_android_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_legacy_session_count_flutter_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_flutter_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_legacy_session_count_ios_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_ios_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_legacy_session_count_reactnative_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_reactnative_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_legacy_session_count_roku_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_roku_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_lite_session_count_android_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_android_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_lite_session_count_flutter_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_flutter_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_lite_session_count_ios_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_ios_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_lite_session_count_reactnative_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_reactnative_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_mobile_lite_session_count_roku_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_roku_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_replay_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_replay_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_total_session_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_total_session_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_units_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_units_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sca_fargate_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sca_fargate_count_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_count_hwm_sum =
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
                        "serverless_apps_azure_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_google_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_google_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_total_count_avg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_total_count_avg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_analyzed_logs_add_on_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_analyzed_logs_add_on_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_browser_check_calls_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_browser_check_calls_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_check_calls_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_check_calls_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_mobile_test_runs_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_mobile_test_runs_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_parallel_testing_max_slots_hwm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_parallel_testing_max_slots_hwm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_search_indexed_events_count_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_search_indexed_events_count_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twol_ingested_events_bytes_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            twol_ingested_events_bytes_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "universal_service_monitoring_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            universal_service_monitoring_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vsphere_host_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            vsphere_host_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_management_host_count_top99p_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_management_host_count_top99p_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow_executions_usage_agg_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_executions_usage_agg_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = UsageSummaryResponse {
                    agent_host_top99p_sum,
                    apm_azure_app_service_host_top99p_sum,
                    apm_devsecops_host_top99p_sum,
                    apm_fargate_count_avg_sum,
                    apm_host_top99p_sum,
                    appsec_fargate_count_avg_sum,
                    asm_serverless_agg_sum,
                    audit_logs_lines_indexed_agg_sum,
                    audit_trail_enabled_hwm_sum,
                    avg_profiled_fargate_tasks_sum,
                    aws_host_top99p_sum,
                    aws_lambda_func_count,
                    aws_lambda_invocations_sum,
                    azure_app_service_top99p_sum,
                    azure_host_top99p_sum,
                    billable_ingested_bytes_agg_sum,
                    browser_rum_lite_session_count_agg_sum,
                    browser_rum_replay_session_count_agg_sum,
                    browser_rum_units_agg_sum,
                    ci_pipeline_indexed_spans_agg_sum,
                    ci_test_indexed_spans_agg_sum,
                    ci_visibility_itr_committers_hwm_sum,
                    ci_visibility_pipeline_committers_hwm_sum,
                    ci_visibility_test_committers_hwm_sum,
                    cloud_cost_management_aws_host_count_avg_sum,
                    cloud_cost_management_azure_host_count_avg_sum,
                    cloud_cost_management_gcp_host_count_avg_sum,
                    cloud_cost_management_host_count_avg_sum,
                    cloud_siem_events_agg_sum,
                    code_analysis_sa_committers_hwm_sum,
                    code_analysis_sca_committers_hwm_sum,
                    code_security_host_top99p_sum,
                    container_avg_sum,
                    container_excl_agent_avg_sum,
                    container_hwm_sum,
                    csm_container_enterprise_compliance_count_agg_sum,
                    csm_container_enterprise_cws_count_agg_sum,
                    csm_container_enterprise_total_count_agg_sum,
                    csm_host_enterprise_aas_host_count_top99p_sum,
                    csm_host_enterprise_aws_host_count_top99p_sum,
                    csm_host_enterprise_azure_host_count_top99p_sum,
                    csm_host_enterprise_compliance_host_count_top99p_sum,
                    csm_host_enterprise_cws_host_count_top99p_sum,
                    csm_host_enterprise_gcp_host_count_top99p_sum,
                    csm_host_enterprise_total_host_count_top99p_sum,
                    cspm_aas_host_top99p_sum,
                    cspm_aws_host_top99p_sum,
                    cspm_azure_host_top99p_sum,
                    cspm_container_avg_sum,
                    cspm_container_hwm_sum,
                    cspm_gcp_host_top99p_sum,
                    cspm_host_top99p_sum,
                    custom_historical_ts_sum,
                    custom_live_ts_sum,
                    custom_ts_sum,
                    cws_container_avg_sum,
                    cws_fargate_task_avg_sum,
                    cws_host_top99p_sum,
                    data_jobs_monitoring_host_hr_agg_sum,
                    dbm_host_top99p_sum,
                    dbm_queries_avg_sum,
                    end_date,
                    error_tracking_error_events_agg_sum,
                    error_tracking_events_agg_sum,
                    error_tracking_rum_error_events_agg_sum,
                    fargate_tasks_count_avg_sum,
                    fargate_tasks_count_hwm_sum,
                    flex_logs_compute_large_avg_sum,
                    flex_logs_compute_medium_avg_sum,
                    flex_logs_compute_small_avg_sum,
                    flex_logs_compute_xsmall_avg_sum,
                    flex_logs_starter_avg_sum,
                    flex_logs_starter_storage_index_avg_sum,
                    flex_logs_starter_storage_retention_adjustment_avg_sum,
                    flex_stored_logs_avg_sum,
                    forwarding_events_bytes_agg_sum,
                    gcp_host_top99p_sum,
                    heroku_host_top99p_sum,
                    incident_management_monthly_active_users_hwm_sum,
                    indexed_events_count_agg_sum,
                    infra_host_top99p_sum,
                    ingested_events_bytes_agg_sum,
                    iot_device_agg_sum,
                    iot_device_top99p_sum,
                    last_updated,
                    live_indexed_events_agg_sum,
                    live_ingested_bytes_agg_sum,
                    logs_by_retention,
                    mobile_rum_lite_session_count_agg_sum,
                    mobile_rum_session_count_agg_sum,
                    mobile_rum_session_count_android_agg_sum,
                    mobile_rum_session_count_flutter_agg_sum,
                    mobile_rum_session_count_ios_agg_sum,
                    mobile_rum_session_count_reactnative_agg_sum,
                    mobile_rum_session_count_roku_agg_sum,
                    mobile_rum_units_agg_sum,
                    ndm_netflow_events_agg_sum,
                    netflow_indexed_events_count_agg_sum,
                    npm_host_top99p_sum,
                    observability_pipelines_bytes_processed_agg_sum,
                    oci_host_agg_sum,
                    oci_host_top99p_sum,
                    online_archive_events_count_agg_sum,
                    opentelemetry_apm_host_top99p_sum,
                    opentelemetry_host_top99p_sum,
                    profiling_aas_count_top99p_sum,
                    profiling_container_agent_count_avg,
                    profiling_host_count_top99p_sum,
                    rehydrated_indexed_events_agg_sum,
                    rehydrated_ingested_bytes_agg_sum,
                    rum_browser_and_mobile_session_count,
                    rum_browser_legacy_session_count_agg_sum,
                    rum_browser_lite_session_count_agg_sum,
                    rum_browser_replay_session_count_agg_sum,
                    rum_lite_session_count_agg_sum,
                    rum_mobile_legacy_session_count_android_agg_sum,
                    rum_mobile_legacy_session_count_flutter_agg_sum,
                    rum_mobile_legacy_session_count_ios_agg_sum,
                    rum_mobile_legacy_session_count_reactnative_agg_sum,
                    rum_mobile_legacy_session_count_roku_agg_sum,
                    rum_mobile_lite_session_count_android_agg_sum,
                    rum_mobile_lite_session_count_flutter_agg_sum,
                    rum_mobile_lite_session_count_ios_agg_sum,
                    rum_mobile_lite_session_count_reactnative_agg_sum,
                    rum_mobile_lite_session_count_roku_agg_sum,
                    rum_replay_session_count_agg_sum,
                    rum_session_count_agg_sum,
                    rum_total_session_count_agg_sum,
                    rum_units_agg_sum,
                    sca_fargate_count_avg_sum,
                    sca_fargate_count_hwm_sum,
                    sds_apm_scanned_bytes_sum,
                    sds_events_scanned_bytes_sum,
                    sds_logs_scanned_bytes_sum,
                    sds_rum_scanned_bytes_sum,
                    sds_total_scanned_bytes_sum,
                    serverless_apps_azure_count_avg_sum,
                    serverless_apps_google_count_avg_sum,
                    serverless_apps_total_count_avg_sum,
                    siem_analyzed_logs_add_on_count_agg_sum,
                    start_date,
                    synthetics_browser_check_calls_count_agg_sum,
                    synthetics_check_calls_count_agg_sum,
                    synthetics_mobile_test_runs_agg_sum,
                    synthetics_parallel_testing_max_slots_hwm_sum,
                    trace_search_indexed_events_count_agg_sum,
                    twol_ingested_events_bytes_agg_sum,
                    universal_service_monitoring_host_top99p_sum,
                    usage,
                    vsphere_host_top99p_sum,
                    vuln_management_host_count_top99p_sum,
                    workflow_executions_usage_agg_sum,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSummaryResponseVisitor)
    }
}
