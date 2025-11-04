// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with hourly report of all data billed by Datadog all organizations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSummaryDate {
    /// Shows the 99th percentile of all agent hosts over all hours in the current date for all organizations.
    #[serde(rename = "agent_host_top99p")]
    pub agent_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Azure app services using APM over all hours in the current date all organizations.
    #[serde(rename = "apm_azure_app_service_host_top99p")]
    pub apm_azure_app_service_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all APM DevSecOps hosts over all hours in the current date for the given org.
    #[serde(rename = "apm_devsecops_host_top99p")]
    pub apm_devsecops_host_top99p: Option<i64>,
    /// Shows the average of all APM ECS Fargate tasks over all hours in the current date for all organizations.
    #[serde(rename = "apm_fargate_count_avg")]
    pub apm_fargate_count_avg: Option<i64>,
    /// Shows the 99th percentile of all distinct APM hosts over all hours in the current date for all organizations.
    #[serde(rename = "apm_host_top99p")]
    pub apm_host_top99p: Option<i64>,
    /// Shows the average of all Application Security Monitoring ECS Fargate tasks over all hours in the current date for all organizations.
    #[serde(rename = "appsec_fargate_count_avg")]
    pub appsec_fargate_count_avg: Option<i64>,
    /// Shows the sum of all Application Security Monitoring Serverless invocations over all hours in the current date for all organizations.
    #[serde(rename = "asm_serverless_sum")]
    pub asm_serverless_sum: Option<i64>,
    /// Shows the sum of audit logs lines indexed over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "audit_logs_lines_indexed_sum")]
    pub audit_logs_lines_indexed_sum: Option<i64>,
    /// Shows the number of organizations that had Audit Trail enabled in the current date.
    #[serde(rename = "audit_trail_enabled_hwm")]
    pub audit_trail_enabled_hwm: Option<i64>,
    /// The average total count for Fargate Container Profiler over all hours in the current date for all organizations.
    #[serde(rename = "avg_profiled_fargate_tasks")]
    pub avg_profiled_fargate_tasks: Option<i64>,
    /// Shows the 99th percentile of all AWS hosts over all hours in the current date for all organizations.
    #[serde(rename = "aws_host_top99p")]
    pub aws_host_top99p: Option<i64>,
    /// Shows the average of the number of functions that executed 1 or more times each hour in the current date for all organizations.
    #[serde(rename = "aws_lambda_func_count")]
    pub aws_lambda_func_count: Option<i64>,
    /// Shows the sum of all AWS Lambda invocations over all hours in the current date for all organizations.
    #[serde(rename = "aws_lambda_invocations_sum")]
    pub aws_lambda_invocations_sum: Option<i64>,
    /// Shows the 99th percentile of all Azure app services over all hours in the current date for all organizations.
    #[serde(rename = "azure_app_service_top99p")]
    pub azure_app_service_top99p: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current date for all organizations.
    #[serde(rename = "billable_ingested_bytes_sum")]
    pub billable_ingested_bytes_sum: Option<i64>,
    /// Shows the sum of all Bits AI Investigations over all hours in the current date for all organizations.
    #[serde(rename = "bits_ai_investigations_sum")]
    pub bits_ai_investigations_sum: Option<i64>,
    /// Shows the sum of all browser lite sessions over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "browser_rum_lite_session_count_sum")]
    pub browser_rum_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all browser replay sessions over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[serde(rename = "browser_rum_replay_session_count_sum")]
    pub browser_rum_replay_session_count_sum: Option<i64>,
    /// Shows the sum of all browser RUM units over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "browser_rum_units_sum")]
    pub browser_rum_units_sum: Option<i64>,
    /// Shows the sum of all CI pipeline indexed spans over all hours in the current month for all organizations.
    #[serde(rename = "ci_pipeline_indexed_spans_sum")]
    pub ci_pipeline_indexed_spans_sum: Option<i64>,
    /// Shows the sum of all CI test indexed spans over all hours in the current month for all organizations.
    #[serde(rename = "ci_test_indexed_spans_sum")]
    pub ci_test_indexed_spans_sum: Option<i64>,
    /// Shows the high-water mark of all CI visibility intelligent test runner committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_itr_committers_hwm")]
    pub ci_visibility_itr_committers_hwm: Option<i64>,
    /// Shows the high-water mark of all CI visibility pipeline committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_pipeline_committers_hwm")]
    pub ci_visibility_pipeline_committers_hwm: Option<i64>,
    /// Shows the high-water mark of all CI visibility test committers over all hours in the current month for all organizations.
    #[serde(rename = "ci_visibility_test_committers_hwm")]
    pub ci_visibility_test_committers_hwm: Option<i64>,
    /// Host count average of Cloud Cost Management for AWS for the given date and given organization.
    #[serde(rename = "cloud_cost_management_aws_host_count_avg")]
    pub cloud_cost_management_aws_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for Azure for the given date and given organization.
    #[serde(rename = "cloud_cost_management_azure_host_count_avg")]
    pub cloud_cost_management_azure_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for GCP for the given date and given organization.
    #[serde(rename = "cloud_cost_management_gcp_host_count_avg")]
    pub cloud_cost_management_gcp_host_count_avg: Option<i64>,
    /// Host count average of Cloud Cost Management for all cloud providers for the given date and given organization.
    #[serde(rename = "cloud_cost_management_host_count_avg")]
    pub cloud_cost_management_host_count_avg: Option<i64>,
    /// Shows the sum of all Cloud Security Information and Event Management events over all hours in the current date for the given org.
    #[serde(rename = "cloud_siem_events_sum")]
    pub cloud_siem_events_sum: Option<i64>,
    /// Shows the high-water mark of all Static Analysis committers over all hours in the current date for the given org.
    #[serde(rename = "code_analysis_sa_committers_hwm")]
    pub code_analysis_sa_committers_hwm: Option<i64>,
    /// Shows the high-water mark of all static Software Composition Analysis committers over all hours in the current date for the given org.
    #[serde(rename = "code_analysis_sca_committers_hwm")]
    pub code_analysis_sca_committers_hwm: Option<i64>,
    /// Shows the 99th percentile of all Code Security hosts over all hours in the current date for the given org.
    #[serde(rename = "code_security_host_top99p")]
    pub code_security_host_top99p: Option<i64>,
    /// Shows the average of all distinct containers over all hours in the current date for all organizations.
    #[serde(rename = "container_avg")]
    pub container_avg: Option<i64>,
    /// Shows the average of containers without the Datadog Agent over all hours in the current date for all organizations.
    #[serde(rename = "container_excl_agent_avg")]
    pub container_excl_agent_avg: Option<i64>,
    /// Shows the high-water mark of all distinct containers over all hours in the current date for all organizations.
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
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure app services hosts over all hours in the current date for all organizations.
    #[serde(rename = "cspm_aas_host_top99p")]
    pub cspm_aas_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro AWS hosts over all hours in the current date for all organizations.
    #[serde(rename = "cspm_aws_host_top99p")]
    pub cspm_aws_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro Azure hosts over all hours in the current date for all organizations.
    #[serde(rename = "cspm_azure_host_top99p")]
    pub cspm_azure_host_top99p: Option<i64>,
    /// Shows the average number of Cloud Security Management Pro containers over all hours in the current date for all organizations.
    #[serde(rename = "cspm_container_avg")]
    pub cspm_container_avg: Option<i64>,
    /// Shows the high-water mark of Cloud Security Management Pro containers over all hours in the current date for all organizations.
    #[serde(rename = "cspm_container_hwm")]
    pub cspm_container_hwm: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro GCP hosts over all hours in the current date for all organizations.
    #[serde(rename = "cspm_gcp_host_top99p")]
    pub cspm_gcp_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Cloud Security Management Pro hosts over all hours in the current date for all organizations.
    #[serde(rename = "cspm_host_top99p")]
    pub cspm_host_top99p: Option<i64>,
    /// Shows the average number of distinct custom metrics over all hours in the current date for all organizations.
    #[serde(rename = "custom_ts_avg")]
    pub custom_ts_avg: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security containers over all hours in the current date for all organizations.
    #[serde(rename = "cws_container_count_avg")]
    pub cws_container_count_avg: Option<i64>,
    /// Shows the average of all distinct Cloud Workload Security Fargate tasks over all hours in the current date for all organizations.
    #[serde(rename = "cws_fargate_task_avg")]
    pub cws_fargate_task_avg: Option<i64>,
    /// Shows the 99th percentile of all Cloud Workload Security hosts over all hours in the current date for all organizations.
    #[serde(rename = "cws_host_top99p")]
    pub cws_host_top99p: Option<i64>,
    /// Shows the sum of all Data Jobs Monitoring hosts over all hours in the current date for the given org.
    #[serde(rename = "data_jobs_monitoring_host_hr_sum")]
    pub data_jobs_monitoring_host_hr_sum: Option<i64>,
    /// The date for the usage.
    #[serde(rename = "date")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    /// Shows the 99th percentile of all Database Monitoring hosts over all hours in the current date for all organizations.
    #[serde(rename = "dbm_host_top99p")]
    pub dbm_host_top99p: Option<i64>,
    /// Shows the average of all normalized Database Monitoring queries over all hours in the current date for all organizations.
    #[serde(rename = "dbm_queries_count_avg")]
    pub dbm_queries_count_avg: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts with the Datadog Agent over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_agent_sum")]
    pub eph_infra_host_agent_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts on Alibaba over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_alibaba_sum")]
    pub eph_infra_host_alibaba_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts on AWS over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_aws_sum")]
    pub eph_infra_host_aws_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts on Azure over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_azure_sum")]
    pub eph_infra_host_azure_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts for Enterprise over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_ent_sum")]
    pub eph_infra_host_ent_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts on GCP over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_gcp_sum")]
    pub eph_infra_host_gcp_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts on Heroku over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_heroku_sum")]
    pub eph_infra_host_heroku_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts with only Azure App Services over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_only_aas_sum")]
    pub eph_infra_host_only_aas_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts with only vSphere over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_only_vsphere_sum")]
    pub eph_infra_host_only_vsphere_sum: Option<i64>,
    /// Shows the sum of all ephemeral APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_opentelemetry_apm_sum")]
    pub eph_infra_host_opentelemetry_apm_sum: Option<i64>,
    /// Shows the sum of all ephemeral hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_opentelemetry_sum")]
    pub eph_infra_host_opentelemetry_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts for Pro over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_pro_sum")]
    pub eph_infra_host_pro_sum: Option<i64>,
    /// Shows the sum of all ephemeral infrastructure hosts for Pro Plus over all hours in the current date for the given org.
    #[serde(rename = "eph_infra_host_proplus_sum")]
    pub eph_infra_host_proplus_sum: Option<i64>,
    /// Shows the sum of all Error Tracking APM error events over all hours in the current date for the given org.
    #[serde(rename = "error_tracking_apm_error_events_sum")]
    pub error_tracking_apm_error_events_sum: Option<i64>,
    /// Shows the sum of all Error Tracking error events over all hours in the current date for the given org.
    #[serde(rename = "error_tracking_error_events_sum")]
    pub error_tracking_error_events_sum: Option<i64>,
    /// Shows the sum of all Error Tracking events over all hours in the current date for the given org.
    #[serde(rename = "error_tracking_events_sum")]
    pub error_tracking_events_sum: Option<i64>,
    /// Shows the sum of all Error Tracking RUM error events over all hours in the current date for the given org.
    #[serde(rename = "error_tracking_rum_error_events_sum")]
    pub error_tracking_rum_error_events_sum: Option<i64>,
    /// Shows the sum of all Event Management correlated events over all hours in the current date for all organizations.
    #[serde(rename = "event_management_correlation_correlated_events_sum")]
    pub event_management_correlation_correlated_events_sum: Option<i64>,
    /// Shows the sum of all Event Management correlated related events over all hours in the current date for all organizations.
    #[serde(rename = "event_management_correlation_correlated_related_events_sum")]
    pub event_management_correlation_correlated_related_events_sum: Option<i64>,
    /// Shows the sum of all Event Management correlations over all hours in the current date for all organizations.
    #[serde(rename = "event_management_correlation_sum")]
    pub event_management_correlation_sum: Option<i64>,
    /// The average number of Profiling Fargate tasks over all hours in the current date for all organizations.
    #[serde(rename = "fargate_container_profiler_profiling_fargate_avg")]
    pub fargate_container_profiler_profiling_fargate_avg: Option<i64>,
    /// The average number of Profiling Fargate Elastic Kubernetes Service tasks over all hours in the current date for all organizations.
    #[serde(rename = "fargate_container_profiler_profiling_fargate_eks_avg")]
    pub fargate_container_profiler_profiling_fargate_eks_avg: Option<i64>,
    /// Shows the high-watermark of all Fargate tasks over all hours in the current date for all organizations.
    #[serde(rename = "fargate_tasks_count_avg")]
    pub fargate_tasks_count_avg: Option<i64>,
    /// Shows the average of all Fargate tasks over all hours in the current date for all organizations.
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
    /// Shows the average number of Flex Logs Compute Extra Large Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_xlarge_avg")]
    pub flex_logs_compute_xlarge_avg: Option<i64>,
    /// Shows the average number of Flex Logs Compute Extra Small Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_compute_xsmall_avg")]
    pub flex_logs_compute_xsmall_avg: Option<i64>,
    /// Shows the average number of Flex Logs Starter Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_starter_avg")]
    pub flex_logs_starter_avg: Option<i64>,
    /// Shows the average number of Flex Logs Starter Storage Index Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_starter_storage_index_avg")]
    pub flex_logs_starter_storage_index_avg: Option<i64>,
    /// Shows the average number of Flex Logs Starter Storage Retention Adjustment Instances over all hours in the current date for the given org.
    #[serde(rename = "flex_logs_starter_storage_retention_adjustment_avg")]
    pub flex_logs_starter_storage_retention_adjustment_avg: Option<i64>,
    /// Shows the average of all Flex Stored Logs over all hours in the current date for the given org.
    #[serde(rename = "flex_stored_logs_avg")]
    pub flex_stored_logs_avg: Option<i64>,
    /// Shows the sum of all log bytes forwarded over all hours in the current date for all organizations.
    #[serde(rename = "forwarding_events_bytes_sum")]
    pub forwarding_events_bytes_sum: Option<i64>,
    /// Shows the 99th percentile of all GCP hosts over all hours in the current date for all organizations.
    #[serde(rename = "gcp_host_top99p")]
    pub gcp_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Heroku dynos over all hours in the current date for all organizations.
    #[serde(rename = "heroku_host_top99p")]
    pub heroku_host_top99p: Option<i64>,
    /// Shows the high-water mark of incident management monthly active users over all hours in the current date for all organizations.
    #[serde(rename = "incident_management_monthly_active_users_hwm")]
    pub incident_management_monthly_active_users_hwm: Option<i64>,
    /// Shows the sum of all log events indexed over all hours in the current date for all organizations.
    #[serde(rename = "indexed_events_count_sum")]
    pub indexed_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of all distinct infrastructure hosts over all hours in the current date for all organizations.
    #[serde(rename = "infra_host_top99p")]
    pub infra_host_top99p: Option<i64>,
    /// Shows the sum of all log bytes ingested over all hours in the current date for all organizations.
    #[serde(rename = "ingested_events_bytes_sum")]
    pub ingested_events_bytes_sum: Option<i64>,
    /// Shows the sum of all IoT devices over all hours in the current date for all organizations.
    #[serde(rename = "iot_device_sum")]
    pub iot_device_sum: Option<i64>,
    /// Shows the 99th percentile of all IoT devices over all hours in the current date all organizations.
    #[serde(rename = "iot_device_top99p")]
    pub iot_device_top99p: Option<i64>,
    /// Sum of all LLM observability minimum spend over all hours in the current date for all organizations.
    #[serde(rename = "llm_observability_min_spend_sum")]
    pub llm_observability_min_spend_sum: Option<i64>,
    /// Sum of all LLM observability sessions over all hours in the current date for all organizations.
    #[serde(rename = "llm_observability_sum")]
    pub llm_observability_sum: Option<i64>,
    /// Shows the sum of all mobile lite sessions over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_lite_session_count_sum")]
    pub mobile_rum_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Android over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_android_sum")]
    pub mobile_rum_session_count_android_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Flutter over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_flutter_sum")]
    pub mobile_rum_session_count_flutter_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on iOS over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_ios_sum")]
    pub mobile_rum_session_count_ios_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on React Native over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_reactnative_sum")]
    pub mobile_rum_session_count_reactnative_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions on Roku over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_roku_sum")]
    pub mobile_rum_session_count_roku_sum: Option<i64>,
    /// Shows the sum of all mobile RUM sessions over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_session_count_sum")]
    pub mobile_rum_session_count_sum: Option<i64>,
    /// Shows the sum of all mobile RUM units over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "mobile_rum_units_sum")]
    pub mobile_rum_units_sum: Option<i64>,
    /// Shows the sum of all Network Device Monitoring NetFlow events over all hours in the current date for the given org.
    #[serde(rename = "ndm_netflow_events_sum")]
    pub ndm_netflow_events_sum: Option<i64>,
    /// Shows the sum of all Network flows indexed over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "netflow_indexed_events_count_sum")]
    pub netflow_indexed_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of all Network Device Monitoring wireless devices over all hours in the current date for all organizations.
    #[serde(rename = "network_device_wireless_top99p")]
    pub network_device_wireless_top99p: Option<i64>,
    /// Shows the 99th percentile of all distinct Cloud Network Monitoring hosts (formerly known as Network hosts) over all hours in the current date for all organizations.
    #[serde(rename = "npm_host_top99p")]
    pub npm_host_top99p: Option<i64>,
    /// Sum of all observability pipelines bytes processed over all hours in the current date for the given org.
    #[serde(rename = "observability_pipelines_bytes_processed_sum")]
    pub observability_pipelines_bytes_processed_sum: Option<i64>,
    /// Shows the sum of all Oracle Cloud Infrastructure hosts over all hours in the current date for the given org.
    #[serde(rename = "oci_host_sum")]
    pub oci_host_sum: Option<i64>,
    /// Shows the 99th percentile of all Oracle Cloud Infrastructure hosts over all hours in the current date for the given org.
    #[serde(rename = "oci_host_top99p")]
    pub oci_host_top99p: Option<i64>,
    /// Shows the high-water mark of On-Call seats over all hours in the current date for all organizations.
    #[serde(rename = "on_call_seat_hwm")]
    pub on_call_seat_hwm: Option<i64>,
    /// Sum of all online archived events over all hours in the current date for all organizations.
    #[serde(rename = "online_archive_events_count_sum")]
    pub online_archive_events_count_sum: Option<i64>,
    /// Shows the 99th percentile of APM hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for all organizations.
    #[serde(rename = "opentelemetry_apm_host_top99p")]
    pub opentelemetry_apm_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all hosts reported by the Datadog exporter for the OpenTelemetry Collector over all hours in the current date for all organizations.
    #[serde(rename = "opentelemetry_host_top99p")]
    pub opentelemetry_host_top99p: Option<i64>,
    /// Organizations associated with a user.
    #[serde(rename = "orgs")]
    pub orgs: Option<Vec<crate::datadogV1::model::UsageSummaryDateOrg>>,
    /// Sum of all product analytics sessions over all hours in the current date for all organizations.
    #[serde(rename = "product_analytics_sum")]
    pub product_analytics_sum: Option<i64>,
    /// Shows the 99th percentile of all profiled Azure app services over all hours in the current date for all organizations.
    #[serde(rename = "profiling_aas_count_top99p")]
    pub profiling_aas_count_top99p: Option<i64>,
    /// Shows the 99th percentile of all profiled hosts over all hours within the current date for all organizations.
    #[serde(rename = "profiling_host_top99p")]
    pub profiling_host_top99p: Option<i64>,
    /// Shows the high-water mark of all published applications over all hours in the current date for all organizations.
    #[serde(rename = "published_app_hwm")]
    pub published_app_hwm: Option<i64>,
    /// Shows the sum of all mobile sessions and all browser lite and legacy sessions over all hours in the current month for all organizations (To be deprecated on October 1st, 2024).
    #[serde(rename = "rum_browser_and_mobile_session_count")]
    pub rum_browser_and_mobile_session_count: Option<i64>,
    /// Shows the sum of all browser RUM legacy sessions over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_legacy_session_count_sum")]
    pub rum_browser_legacy_session_count_sum: Option<i64>,
    /// Shows the sum of all browser RUM lite sessions over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_lite_session_count_sum")]
    pub rum_browser_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all browser RUM Session Replay counts over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_browser_replay_session_count_sum")]
    pub rum_browser_replay_session_count_sum: Option<i64>,
    /// Sum of all RUM indexed sessions over all hours in the current date for all organizations.
    #[serde(rename = "rum_indexed_sessions_sum")]
    pub rum_indexed_sessions_sum: Option<i64>,
    /// Sum of all RUM ingested sessions over all hours in the current date for all organizations.
    #[serde(rename = "rum_ingested_sessions_sum")]
    pub rum_ingested_sessions_sum: Option<i64>,
    /// Shows the sum of all RUM lite sessions (browser and mobile) over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_lite_session_count_sum")]
    pub rum_lite_session_count_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on Android over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_android_sum")]
    pub rum_mobile_legacy_session_count_android_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy Sessions on Flutter over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_flutter_sum")]
    pub rum_mobile_legacy_session_count_flutter_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on iOS over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_ios_sum")]
    pub rum_mobile_legacy_session_count_ios_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on React Native over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_reactnative_sum")]
    pub rum_mobile_legacy_session_count_reactnative_sum: Option<i64>,
    /// Shows the sum of all mobile RUM legacy sessions on Roku over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_legacy_session_count_roku_sum")]
    pub rum_mobile_legacy_session_count_roku_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Android over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_android_sum")]
    pub rum_mobile_lite_session_count_android_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Flutter over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_flutter_sum")]
    pub rum_mobile_lite_session_count_flutter_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on iOS over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_ios_sum")]
    pub rum_mobile_lite_session_count_ios_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Kotlin Multiplatform over all hours within the current date for all organizations.
    #[serde(rename = "rum_mobile_lite_session_count_kotlinmultiplatform_sum")]
    pub rum_mobile_lite_session_count_kotlinmultiplatform_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on React Native over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_reactnative_sum")]
    pub rum_mobile_lite_session_count_reactnative_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Roku over all hours within the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_mobile_lite_session_count_roku_sum")]
    pub rum_mobile_lite_session_count_roku_sum: Option<i64>,
    /// Shows the sum of all mobile RUM lite sessions on Unity over all hours within the current date for all organizations.
    #[serde(rename = "rum_mobile_lite_session_count_unity_sum")]
    pub rum_mobile_lite_session_count_unity_sum: Option<i64>,
    /// Shows the sum of all mobile RUM replay sessions on Android over all hours within the current date for the given org.
    #[serde(rename = "rum_mobile_replay_session_count_android_sum")]
    pub rum_mobile_replay_session_count_android_sum: Option<i64>,
    /// Shows the sum of all mobile RUM replay sessions on iOS over all hours within the current date for the given org.
    #[serde(rename = "rum_mobile_replay_session_count_ios_sum")]
    pub rum_mobile_replay_session_count_ios_sum: Option<i64>,
    /// Shows the sum of all mobile RUM replay sessions on Kotlin Multiplatform over all hours within the current date for all organizations.
    #[serde(rename = "rum_mobile_replay_session_count_kotlinmultiplatform_sum")]
    pub rum_mobile_replay_session_count_kotlinmultiplatform_sum: Option<i64>,
    /// Shows the sum of all mobile RUM replay sessions on React Native over all hours within the current date for the given org.
    #[serde(rename = "rum_mobile_replay_session_count_reactnative_sum")]
    pub rum_mobile_replay_session_count_reactnative_sum: Option<i64>,
    /// Shows the sum of all RUM Session Replay counts over all hours in the current date for all organizations (To be introduced on October 1st, 2024).
    #[serde(rename = "rum_replay_session_count_sum")]
    pub rum_replay_session_count_sum: Option<i64>,
    /// Shows the sum of all browser RUM lite sessions over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "rum_session_count_sum")]
    pub rum_session_count_sum: Option<i64>,
    /// Sum of all RUM session replay add-on sessions over all hours in the current date for all organizations.
    #[serde(rename = "rum_session_replay_add_on_sum")]
    pub rum_session_replay_add_on_sum: Option<i64>,
    /// Shows the sum of RUM sessions (browser and mobile) over all hours in the current date for all organizations.
    #[serde(rename = "rum_total_session_count_sum")]
    pub rum_total_session_count_sum: Option<i64>,
    /// Shows the sum of all browser and mobile RUM units over all hours in the current date for all organizations (To be deprecated on October 1st, 2024).
    #[deprecated]
    #[serde(rename = "rum_units_sum")]
    pub rum_units_sum: Option<i64>,
    /// Shows the average of all Software Composition Analysis Fargate tasks over all hours in the current date for the given org.
    #[serde(rename = "sca_fargate_count_avg")]
    pub sca_fargate_count_avg: Option<i64>,
    /// Shows the sum of the high-water marks of all Software Composition Analysis Fargate tasks over all hours in the current date for the given org.
    #[serde(rename = "sca_fargate_count_hwm")]
    pub sca_fargate_count_hwm: Option<i64>,
    /// Sum of all APM bytes scanned with sensitive data scanner over all hours in the current date for all organizations.
    #[serde(rename = "sds_apm_scanned_bytes_sum")]
    pub sds_apm_scanned_bytes_sum: Option<i64>,
    /// Sum of all event stream events bytes scanned with sensitive data scanner over all hours in the current date for all organizations.
    #[serde(rename = "sds_events_scanned_bytes_sum")]
    pub sds_events_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned of logs usage by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_logs_scanned_bytes_sum")]
    pub sds_logs_scanned_bytes_sum: Option<i64>,
    /// Sum of all RUM bytes scanned with sensitive data scanner over all hours in the current date for all organizations.
    #[serde(rename = "sds_rum_scanned_bytes_sum")]
    pub sds_rum_scanned_bytes_sum: Option<i64>,
    /// Shows the sum of all bytes scanned across all usage types by the Sensitive Data Scanner over all hours in the current month for all organizations.
    #[serde(rename = "sds_total_scanned_bytes_sum")]
    pub sds_total_scanned_bytes_sum: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Azure App Service instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_azure_appservice_instances_avg")]
    pub serverless_apps_apm_apm_azure_appservice_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Azure Function instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_azure_azurefunction_instances_avg")]
    pub serverless_apps_apm_apm_azure_azurefunction_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Azure Container App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_azure_containerapp_instances_avg")]
    pub serverless_apps_apm_apm_azure_containerapp_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Fargate Elastic Container Service tasks for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_fargate_ecs_tasks_avg")]
    pub serverless_apps_apm_apm_fargate_ecs_tasks_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Google Cloud Platform Cloud Function instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_gcp_cloudfunction_instances_avg")]
    pub serverless_apps_apm_apm_gcp_cloudfunction_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for Google Cloud Platform Cloud Run instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_apm_gcp_cloudrun_instances_avg")]
    pub serverless_apps_apm_apm_gcp_cloudrun_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_avg")]
    pub serverless_apps_apm_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for Azure App Service instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg")]
    pub serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for Azure Function instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg")]
    pub serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for Azure Container App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg")]
    pub serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for Google Cloud Platform Cloud Function instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg")]
    pub serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for Google Cloud Platform Cloud Run instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg")]
    pub serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps with Application Performance Monitoring excluding Fargate for the current date for all organizations.
    #[serde(rename = "serverless_apps_apm_excl_fargate_avg")]
    pub serverless_apps_apm_excl_fargate_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Azure Container App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_azure_container_app_instances_avg")]
    pub serverless_apps_azure_container_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Azure for the given date and given org.
    #[serde(rename = "serverless_apps_azure_count_avg")]
    pub serverless_apps_azure_count_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Azure Function App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_azure_function_app_instances_avg")]
    pub serverless_apps_azure_function_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Azure Web App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_azure_web_app_instances_avg")]
    pub serverless_apps_azure_web_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Elastic Container Service for the current date for all organizations.
    #[serde(rename = "serverless_apps_ecs_avg")]
    pub serverless_apps_ecs_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Elastic Kubernetes Service for the current date for all organizations.
    #[serde(rename = "serverless_apps_eks_avg")]
    pub serverless_apps_eks_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_avg")]
    pub serverless_apps_excl_fargate_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for Azure Container App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_azure_container_app_instances_avg")]
    pub serverless_apps_excl_fargate_azure_container_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for Azure Function App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_azure_function_app_instances_avg")]
    pub serverless_apps_excl_fargate_azure_function_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for Azure Web App instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_azure_web_app_instances_avg")]
    pub serverless_apps_excl_fargate_azure_web_app_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for Google Cloud Platform Cloud Functions instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_google_cloud_functions_instances_avg")]
    pub serverless_apps_excl_fargate_google_cloud_functions_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps excluding Fargate for Google Cloud Platform Cloud Run instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_excl_fargate_google_cloud_run_instances_avg")]
    pub serverless_apps_excl_fargate_google_cloud_run_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Google Cloud Platform Cloud Functions instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_google_cloud_functions_instances_avg")]
    pub serverless_apps_google_cloud_functions_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Google Cloud Platform Cloud Run instances for the current date for all organizations.
    #[serde(rename = "serverless_apps_google_cloud_run_instances_avg")]
    pub serverless_apps_google_cloud_run_instances_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Google Cloud for the given date and given org.
    #[serde(rename = "serverless_apps_google_count_avg")]
    pub serverless_apps_google_count_avg: Option<i64>,
    /// Shows the average number of Serverless Apps for Azure and Google Cloud for the given date and given org.
    #[serde(rename = "serverless_apps_total_count_avg")]
    pub serverless_apps_total_count_avg: Option<i64>,
    /// Shows the sum of all log events analyzed by Cloud SIEM over all hours in the current date for the given org.
    #[serde(rename = "siem_analyzed_logs_add_on_count_sum")]
    pub siem_analyzed_logs_add_on_count_sum: Option<i64>,
    /// Shows the sum of all Synthetic browser tests over all hours in the current date for all organizations.
    #[serde(rename = "synthetics_browser_check_calls_count_sum")]
    pub synthetics_browser_check_calls_count_sum: Option<i64>,
    /// Shows the sum of all Synthetic API tests over all hours in the current date for all organizations.
    #[serde(rename = "synthetics_check_calls_count_sum")]
    pub synthetics_check_calls_count_sum: Option<i64>,
    /// Shows the sum of all Synthetic mobile application tests over all hours in the current date for all organizations.
    #[serde(rename = "synthetics_mobile_test_runs_sum")]
    pub synthetics_mobile_test_runs_sum: Option<i64>,
    /// Shows the high-water mark of used synthetics parallel testing slots over all hours in the current date for all organizations.
    #[serde(rename = "synthetics_parallel_testing_max_slots_hwm")]
    pub synthetics_parallel_testing_max_slots_hwm: Option<i64>,
    /// Shows the sum of all Indexed Spans indexed over all hours in the current date for all organizations.
    #[serde(rename = "trace_search_indexed_events_count_sum")]
    pub trace_search_indexed_events_count_sum: Option<i64>,
    /// Shows the sum of all ingested APM span bytes over all hours in the current date for all organizations.
    #[serde(rename = "twol_ingested_events_bytes_sum")]
    pub twol_ingested_events_bytes_sum: Option<i64>,
    /// Shows the 99th percentile of all universal service management hosts over all hours in the current date for the given org.
    #[serde(rename = "universal_service_monitoring_host_top99p")]
    pub universal_service_monitoring_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all vSphere hosts over all hours in the current date for all organizations.
    #[serde(rename = "vsphere_host_top99p")]
    pub vsphere_host_top99p: Option<i64>,
    /// Shows the 99th percentile of all Application Vulnerability Management hosts over all hours in the current date for the given org.
    #[serde(rename = "vuln_management_host_count_top99p")]
    pub vuln_management_host_count_top99p: Option<i64>,
    /// Sum of all workflows executed over all hours in the current date for all organizations.
    #[serde(rename = "workflow_executions_usage_sum")]
    pub workflow_executions_usage_sum: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSummaryDate {
    pub fn new() -> UsageSummaryDate {
        #[allow(deprecated)]
        UsageSummaryDate {
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
            bits_ai_investigations_sum: None,
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
            code_analysis_sa_committers_hwm: None,
            code_analysis_sca_committers_hwm: None,
            code_security_host_top99p: None,
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
            custom_ts_avg: None,
            cws_container_count_avg: None,
            cws_fargate_task_avg: None,
            cws_host_top99p: None,
            data_jobs_monitoring_host_hr_sum: None,
            date: None,
            dbm_host_top99p: None,
            dbm_queries_count_avg: None,
            eph_infra_host_agent_sum: None,
            eph_infra_host_alibaba_sum: None,
            eph_infra_host_aws_sum: None,
            eph_infra_host_azure_sum: None,
            eph_infra_host_ent_sum: None,
            eph_infra_host_gcp_sum: None,
            eph_infra_host_heroku_sum: None,
            eph_infra_host_only_aas_sum: None,
            eph_infra_host_only_vsphere_sum: None,
            eph_infra_host_opentelemetry_apm_sum: None,
            eph_infra_host_opentelemetry_sum: None,
            eph_infra_host_pro_sum: None,
            eph_infra_host_proplus_sum: None,
            error_tracking_apm_error_events_sum: None,
            error_tracking_error_events_sum: None,
            error_tracking_events_sum: None,
            error_tracking_rum_error_events_sum: None,
            event_management_correlation_correlated_events_sum: None,
            event_management_correlation_correlated_related_events_sum: None,
            event_management_correlation_sum: None,
            fargate_container_profiler_profiling_fargate_avg: None,
            fargate_container_profiler_profiling_fargate_eks_avg: None,
            fargate_tasks_count_avg: None,
            fargate_tasks_count_hwm: None,
            flex_logs_compute_large_avg: None,
            flex_logs_compute_medium_avg: None,
            flex_logs_compute_small_avg: None,
            flex_logs_compute_xlarge_avg: None,
            flex_logs_compute_xsmall_avg: None,
            flex_logs_starter_avg: None,
            flex_logs_starter_storage_index_avg: None,
            flex_logs_starter_storage_retention_adjustment_avg: None,
            flex_stored_logs_avg: None,
            forwarding_events_bytes_sum: None,
            gcp_host_top99p: None,
            heroku_host_top99p: None,
            incident_management_monthly_active_users_hwm: None,
            indexed_events_count_sum: None,
            infra_host_top99p: None,
            ingested_events_bytes_sum: None,
            iot_device_sum: None,
            iot_device_top99p: None,
            llm_observability_min_spend_sum: None,
            llm_observability_sum: None,
            mobile_rum_lite_session_count_sum: None,
            mobile_rum_session_count_android_sum: None,
            mobile_rum_session_count_flutter_sum: None,
            mobile_rum_session_count_ios_sum: None,
            mobile_rum_session_count_reactnative_sum: None,
            mobile_rum_session_count_roku_sum: None,
            mobile_rum_session_count_sum: None,
            mobile_rum_units_sum: None,
            ndm_netflow_events_sum: None,
            netflow_indexed_events_count_sum: None,
            network_device_wireless_top99p: None,
            npm_host_top99p: None,
            observability_pipelines_bytes_processed_sum: None,
            oci_host_sum: None,
            oci_host_top99p: None,
            on_call_seat_hwm: None,
            online_archive_events_count_sum: None,
            opentelemetry_apm_host_top99p: None,
            opentelemetry_host_top99p: None,
            orgs: None,
            product_analytics_sum: None,
            profiling_aas_count_top99p: None,
            profiling_host_top99p: None,
            published_app_hwm: None,
            rum_browser_and_mobile_session_count: None,
            rum_browser_legacy_session_count_sum: None,
            rum_browser_lite_session_count_sum: None,
            rum_browser_replay_session_count_sum: None,
            rum_indexed_sessions_sum: None,
            rum_ingested_sessions_sum: None,
            rum_lite_session_count_sum: None,
            rum_mobile_legacy_session_count_android_sum: None,
            rum_mobile_legacy_session_count_flutter_sum: None,
            rum_mobile_legacy_session_count_ios_sum: None,
            rum_mobile_legacy_session_count_reactnative_sum: None,
            rum_mobile_legacy_session_count_roku_sum: None,
            rum_mobile_lite_session_count_android_sum: None,
            rum_mobile_lite_session_count_flutter_sum: None,
            rum_mobile_lite_session_count_ios_sum: None,
            rum_mobile_lite_session_count_kotlinmultiplatform_sum: None,
            rum_mobile_lite_session_count_reactnative_sum: None,
            rum_mobile_lite_session_count_roku_sum: None,
            rum_mobile_lite_session_count_unity_sum: None,
            rum_mobile_replay_session_count_android_sum: None,
            rum_mobile_replay_session_count_ios_sum: None,
            rum_mobile_replay_session_count_kotlinmultiplatform_sum: None,
            rum_mobile_replay_session_count_reactnative_sum: None,
            rum_replay_session_count_sum: None,
            rum_session_count_sum: None,
            rum_session_replay_add_on_sum: None,
            rum_total_session_count_sum: None,
            rum_units_sum: None,
            sca_fargate_count_avg: None,
            sca_fargate_count_hwm: None,
            sds_apm_scanned_bytes_sum: None,
            sds_events_scanned_bytes_sum: None,
            sds_logs_scanned_bytes_sum: None,
            sds_rum_scanned_bytes_sum: None,
            sds_total_scanned_bytes_sum: None,
            serverless_apps_apm_apm_azure_appservice_instances_avg: None,
            serverless_apps_apm_apm_azure_azurefunction_instances_avg: None,
            serverless_apps_apm_apm_azure_containerapp_instances_avg: None,
            serverless_apps_apm_apm_fargate_ecs_tasks_avg: None,
            serverless_apps_apm_apm_gcp_cloudfunction_instances_avg: None,
            serverless_apps_apm_apm_gcp_cloudrun_instances_avg: None,
            serverless_apps_apm_avg: None,
            serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg: None,
            serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg: None,
            serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg: None,
            serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg: None,
            serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg: None,
            serverless_apps_apm_excl_fargate_avg: None,
            serverless_apps_azure_container_app_instances_avg: None,
            serverless_apps_azure_count_avg: None,
            serverless_apps_azure_function_app_instances_avg: None,
            serverless_apps_azure_web_app_instances_avg: None,
            serverless_apps_ecs_avg: None,
            serverless_apps_eks_avg: None,
            serverless_apps_excl_fargate_avg: None,
            serverless_apps_excl_fargate_azure_container_app_instances_avg: None,
            serverless_apps_excl_fargate_azure_function_app_instances_avg: None,
            serverless_apps_excl_fargate_azure_web_app_instances_avg: None,
            serverless_apps_excl_fargate_google_cloud_functions_instances_avg: None,
            serverless_apps_excl_fargate_google_cloud_run_instances_avg: None,
            serverless_apps_google_cloud_functions_instances_avg: None,
            serverless_apps_google_cloud_run_instances_avg: None,
            serverless_apps_google_count_avg: None,
            serverless_apps_total_count_avg: None,
            siem_analyzed_logs_add_on_count_sum: None,
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
            additional_properties: std::collections::BTreeMap::new(),
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
    pub fn bits_ai_investigations_sum(mut self, value: i64) -> Self {
        self.bits_ai_investigations_sum = Some(value);
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
    pub fn code_analysis_sa_committers_hwm(mut self, value: i64) -> Self {
        self.code_analysis_sa_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn code_analysis_sca_committers_hwm(mut self, value: i64) -> Self {
        self.code_analysis_sca_committers_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn code_security_host_top99p(mut self, value: i64) -> Self {
        self.code_security_host_top99p = Some(value);
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
    pub fn cws_fargate_task_avg(mut self, value: i64) -> Self {
        self.cws_fargate_task_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cws_host_top99p(mut self, value: i64) -> Self {
        self.cws_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn data_jobs_monitoring_host_hr_sum(mut self, value: i64) -> Self {
        self.data_jobs_monitoring_host_hr_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.date = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn dbm_host_top99p(mut self, value: i64) -> Self {
        self.dbm_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn dbm_queries_count_avg(mut self, value: i64) -> Self {
        self.dbm_queries_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_agent_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_agent_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_alibaba_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_alibaba_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_aws_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_aws_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_azure_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_azure_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_ent_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_ent_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_gcp_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_gcp_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_heroku_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_heroku_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_only_aas_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_only_aas_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_only_vsphere_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_only_vsphere_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_opentelemetry_apm_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_opentelemetry_apm_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_opentelemetry_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_opentelemetry_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_pro_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_pro_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn eph_infra_host_proplus_sum(mut self, value: i64) -> Self {
        self.eph_infra_host_proplus_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_apm_error_events_sum(mut self, value: i64) -> Self {
        self.error_tracking_apm_error_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_error_events_sum(mut self, value: i64) -> Self {
        self.error_tracking_error_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_events_sum(mut self, value: i64) -> Self {
        self.error_tracking_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_tracking_rum_error_events_sum(mut self, value: i64) -> Self {
        self.error_tracking_rum_error_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn event_management_correlation_correlated_events_sum(mut self, value: i64) -> Self {
        self.event_management_correlation_correlated_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn event_management_correlation_correlated_related_events_sum(
        mut self,
        value: i64,
    ) -> Self {
        self.event_management_correlation_correlated_related_events_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn event_management_correlation_sum(mut self, value: i64) -> Self {
        self.event_management_correlation_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_container_profiler_profiling_fargate_avg(mut self, value: i64) -> Self {
        self.fargate_container_profiler_profiling_fargate_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fargate_container_profiler_profiling_fargate_eks_avg(mut self, value: i64) -> Self {
        self.fargate_container_profiler_profiling_fargate_eks_avg = Some(value);
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
    pub fn flex_logs_compute_xlarge_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_xlarge_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_compute_xsmall_avg(mut self, value: i64) -> Self {
        self.flex_logs_compute_xsmall_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_avg(mut self, value: i64) -> Self {
        self.flex_logs_starter_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_storage_index_avg(mut self, value: i64) -> Self {
        self.flex_logs_starter_storage_index_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn flex_logs_starter_storage_retention_adjustment_avg(mut self, value: i64) -> Self {
        self.flex_logs_starter_storage_retention_adjustment_avg = Some(value);
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
    pub fn iot_device_sum(mut self, value: i64) -> Self {
        self.iot_device_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn iot_device_top99p(mut self, value: i64) -> Self {
        self.iot_device_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn llm_observability_min_spend_sum(mut self, value: i64) -> Self {
        self.llm_observability_min_spend_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn llm_observability_sum(mut self, value: i64) -> Self {
        self.llm_observability_sum = Some(value);
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
    pub fn network_device_wireless_top99p(mut self, value: i64) -> Self {
        self.network_device_wireless_top99p = Some(value);
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
    pub fn oci_host_sum(mut self, value: i64) -> Self {
        self.oci_host_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn oci_host_top99p(mut self, value: i64) -> Self {
        self.oci_host_top99p = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn on_call_seat_hwm(mut self, value: i64) -> Self {
        self.on_call_seat_hwm = Some(value);
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
    pub fn orgs(mut self, value: Vec<crate::datadogV1::model::UsageSummaryDateOrg>) -> Self {
        self.orgs = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn product_analytics_sum(mut self, value: i64) -> Self {
        self.product_analytics_sum = Some(value);
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
    pub fn published_app_hwm(mut self, value: i64) -> Self {
        self.published_app_hwm = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_and_mobile_session_count(mut self, value: i64) -> Self {
        self.rum_browser_and_mobile_session_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_legacy_session_count_sum(mut self, value: i64) -> Self {
        self.rum_browser_legacy_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_lite_session_count_sum(mut self, value: i64) -> Self {
        self.rum_browser_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_browser_replay_session_count_sum(mut self, value: i64) -> Self {
        self.rum_browser_replay_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_indexed_sessions_sum(mut self, value: i64) -> Self {
        self.rum_indexed_sessions_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_ingested_sessions_sum(mut self, value: i64) -> Self {
        self.rum_ingested_sessions_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_lite_session_count_sum(mut self, value: i64) -> Self {
        self.rum_lite_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_android_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_android_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_flutter_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_flutter_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_ios_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_ios_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_reactnative_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_reactnative_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_legacy_session_count_roku_sum(mut self, value: i64) -> Self {
        self.rum_mobile_legacy_session_count_roku_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_android_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_android_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_flutter_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_flutter_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_ios_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_ios_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_kotlinmultiplatform_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_kotlinmultiplatform_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_reactnative_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_reactnative_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_roku_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_roku_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_lite_session_count_unity_sum(mut self, value: i64) -> Self {
        self.rum_mobile_lite_session_count_unity_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_replay_session_count_android_sum(mut self, value: i64) -> Self {
        self.rum_mobile_replay_session_count_android_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_replay_session_count_ios_sum(mut self, value: i64) -> Self {
        self.rum_mobile_replay_session_count_ios_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_replay_session_count_kotlinmultiplatform_sum(mut self, value: i64) -> Self {
        self.rum_mobile_replay_session_count_kotlinmultiplatform_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_mobile_replay_session_count_reactnative_sum(mut self, value: i64) -> Self {
        self.rum_mobile_replay_session_count_reactnative_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_replay_session_count_sum(mut self, value: i64) -> Self {
        self.rum_replay_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_session_count_sum(mut self, value: i64) -> Self {
        self.rum_session_count_sum = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_session_replay_add_on_sum(mut self, value: i64) -> Self {
        self.rum_session_replay_add_on_sum = Some(value);
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
    pub fn sca_fargate_count_avg(mut self, value: i64) -> Self {
        self.sca_fargate_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sca_fargate_count_hwm(mut self, value: i64) -> Self {
        self.sca_fargate_count_hwm = Some(value);
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
    pub fn serverless_apps_apm_apm_azure_appservice_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_azure_appservice_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_apm_azure_azurefunction_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_azure_azurefunction_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_apm_azure_containerapp_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_azure_containerapp_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_apm_fargate_ecs_tasks_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_fargate_ecs_tasks_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_apm_gcp_cloudfunction_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_gcp_cloudfunction_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_apm_gcp_cloudrun_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_apm_gcp_cloudrun_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_apm_excl_fargate_avg(mut self, value: i64) -> Self {
        self.serverless_apps_apm_excl_fargate_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_azure_container_app_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_azure_container_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_azure_count_avg(mut self, value: i64) -> Self {
        self.serverless_apps_azure_count_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_azure_function_app_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_azure_function_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_azure_web_app_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_azure_web_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_ecs_avg(mut self, value: i64) -> Self {
        self.serverless_apps_ecs_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_eks_avg(mut self, value: i64) -> Self {
        self.serverless_apps_eks_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_avg(mut self, value: i64) -> Self {
        self.serverless_apps_excl_fargate_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_azure_container_app_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_excl_fargate_azure_container_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_azure_function_app_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_excl_fargate_azure_function_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_azure_web_app_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_excl_fargate_azure_web_app_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_google_cloud_functions_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_excl_fargate_google_cloud_functions_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_excl_fargate_google_cloud_run_instances_avg(
        mut self,
        value: i64,
    ) -> Self {
        self.serverless_apps_excl_fargate_google_cloud_run_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_google_cloud_functions_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_google_cloud_functions_instances_avg = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn serverless_apps_google_cloud_run_instances_avg(mut self, value: i64) -> Self {
        self.serverless_apps_google_cloud_run_instances_avg = Some(value);
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
    pub fn siem_analyzed_logs_add_on_count_sum(mut self, value: i64) -> Self {
        self.siem_analyzed_logs_add_on_count_sum = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for UsageSummaryDate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSummaryDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSummaryDateVisitor;
        impl<'a> Visitor<'a> for UsageSummaryDateVisitor {
            type Value = UsageSummaryDate;

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
                let mut bits_ai_investigations_sum: Option<i64> = None;
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
                let mut code_analysis_sa_committers_hwm: Option<i64> = None;
                let mut code_analysis_sca_committers_hwm: Option<i64> = None;
                let mut code_security_host_top99p: Option<i64> = None;
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
                let mut custom_ts_avg: Option<i64> = None;
                let mut cws_container_count_avg: Option<i64> = None;
                let mut cws_fargate_task_avg: Option<i64> = None;
                let mut cws_host_top99p: Option<i64> = None;
                let mut data_jobs_monitoring_host_hr_sum: Option<i64> = None;
                let mut date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dbm_host_top99p: Option<i64> = None;
                let mut dbm_queries_count_avg: Option<i64> = None;
                let mut eph_infra_host_agent_sum: Option<i64> = None;
                let mut eph_infra_host_alibaba_sum: Option<i64> = None;
                let mut eph_infra_host_aws_sum: Option<i64> = None;
                let mut eph_infra_host_azure_sum: Option<i64> = None;
                let mut eph_infra_host_ent_sum: Option<i64> = None;
                let mut eph_infra_host_gcp_sum: Option<i64> = None;
                let mut eph_infra_host_heroku_sum: Option<i64> = None;
                let mut eph_infra_host_only_aas_sum: Option<i64> = None;
                let mut eph_infra_host_only_vsphere_sum: Option<i64> = None;
                let mut eph_infra_host_opentelemetry_apm_sum: Option<i64> = None;
                let mut eph_infra_host_opentelemetry_sum: Option<i64> = None;
                let mut eph_infra_host_pro_sum: Option<i64> = None;
                let mut eph_infra_host_proplus_sum: Option<i64> = None;
                let mut error_tracking_apm_error_events_sum: Option<i64> = None;
                let mut error_tracking_error_events_sum: Option<i64> = None;
                let mut error_tracking_events_sum: Option<i64> = None;
                let mut error_tracking_rum_error_events_sum: Option<i64> = None;
                let mut event_management_correlation_correlated_events_sum: Option<i64> = None;
                let mut event_management_correlation_correlated_related_events_sum: Option<i64> =
                    None;
                let mut event_management_correlation_sum: Option<i64> = None;
                let mut fargate_container_profiler_profiling_fargate_avg: Option<i64> = None;
                let mut fargate_container_profiler_profiling_fargate_eks_avg: Option<i64> = None;
                let mut fargate_tasks_count_avg: Option<i64> = None;
                let mut fargate_tasks_count_hwm: Option<i64> = None;
                let mut flex_logs_compute_large_avg: Option<i64> = None;
                let mut flex_logs_compute_medium_avg: Option<i64> = None;
                let mut flex_logs_compute_small_avg: Option<i64> = None;
                let mut flex_logs_compute_xlarge_avg: Option<i64> = None;
                let mut flex_logs_compute_xsmall_avg: Option<i64> = None;
                let mut flex_logs_starter_avg: Option<i64> = None;
                let mut flex_logs_starter_storage_index_avg: Option<i64> = None;
                let mut flex_logs_starter_storage_retention_adjustment_avg: Option<i64> = None;
                let mut flex_stored_logs_avg: Option<i64> = None;
                let mut forwarding_events_bytes_sum: Option<i64> = None;
                let mut gcp_host_top99p: Option<i64> = None;
                let mut heroku_host_top99p: Option<i64> = None;
                let mut incident_management_monthly_active_users_hwm: Option<i64> = None;
                let mut indexed_events_count_sum: Option<i64> = None;
                let mut infra_host_top99p: Option<i64> = None;
                let mut ingested_events_bytes_sum: Option<i64> = None;
                let mut iot_device_sum: Option<i64> = None;
                let mut iot_device_top99p: Option<i64> = None;
                let mut llm_observability_min_spend_sum: Option<i64> = None;
                let mut llm_observability_sum: Option<i64> = None;
                let mut mobile_rum_lite_session_count_sum: Option<i64> = None;
                let mut mobile_rum_session_count_android_sum: Option<i64> = None;
                let mut mobile_rum_session_count_flutter_sum: Option<i64> = None;
                let mut mobile_rum_session_count_ios_sum: Option<i64> = None;
                let mut mobile_rum_session_count_reactnative_sum: Option<i64> = None;
                let mut mobile_rum_session_count_roku_sum: Option<i64> = None;
                let mut mobile_rum_session_count_sum: Option<i64> = None;
                let mut mobile_rum_units_sum: Option<i64> = None;
                let mut ndm_netflow_events_sum: Option<i64> = None;
                let mut netflow_indexed_events_count_sum: Option<i64> = None;
                let mut network_device_wireless_top99p: Option<i64> = None;
                let mut npm_host_top99p: Option<i64> = None;
                let mut observability_pipelines_bytes_processed_sum: Option<i64> = None;
                let mut oci_host_sum: Option<i64> = None;
                let mut oci_host_top99p: Option<i64> = None;
                let mut on_call_seat_hwm: Option<i64> = None;
                let mut online_archive_events_count_sum: Option<i64> = None;
                let mut opentelemetry_apm_host_top99p: Option<i64> = None;
                let mut opentelemetry_host_top99p: Option<i64> = None;
                let mut orgs: Option<Vec<crate::datadogV1::model::UsageSummaryDateOrg>> = None;
                let mut product_analytics_sum: Option<i64> = None;
                let mut profiling_aas_count_top99p: Option<i64> = None;
                let mut profiling_host_top99p: Option<i64> = None;
                let mut published_app_hwm: Option<i64> = None;
                let mut rum_browser_and_mobile_session_count: Option<i64> = None;
                let mut rum_browser_legacy_session_count_sum: Option<i64> = None;
                let mut rum_browser_lite_session_count_sum: Option<i64> = None;
                let mut rum_browser_replay_session_count_sum: Option<i64> = None;
                let mut rum_indexed_sessions_sum: Option<i64> = None;
                let mut rum_ingested_sessions_sum: Option<i64> = None;
                let mut rum_lite_session_count_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_android_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_flutter_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_ios_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_reactnative_sum: Option<i64> = None;
                let mut rum_mobile_legacy_session_count_roku_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_android_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_flutter_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_ios_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_kotlinmultiplatform_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_reactnative_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_roku_sum: Option<i64> = None;
                let mut rum_mobile_lite_session_count_unity_sum: Option<i64> = None;
                let mut rum_mobile_replay_session_count_android_sum: Option<i64> = None;
                let mut rum_mobile_replay_session_count_ios_sum: Option<i64> = None;
                let mut rum_mobile_replay_session_count_kotlinmultiplatform_sum: Option<i64> = None;
                let mut rum_mobile_replay_session_count_reactnative_sum: Option<i64> = None;
                let mut rum_replay_session_count_sum: Option<i64> = None;
                let mut rum_session_count_sum: Option<i64> = None;
                let mut rum_session_replay_add_on_sum: Option<i64> = None;
                let mut rum_total_session_count_sum: Option<i64> = None;
                let mut rum_units_sum: Option<i64> = None;
                let mut sca_fargate_count_avg: Option<i64> = None;
                let mut sca_fargate_count_hwm: Option<i64> = None;
                let mut sds_apm_scanned_bytes_sum: Option<i64> = None;
                let mut sds_events_scanned_bytes_sum: Option<i64> = None;
                let mut sds_logs_scanned_bytes_sum: Option<i64> = None;
                let mut sds_rum_scanned_bytes_sum: Option<i64> = None;
                let mut sds_total_scanned_bytes_sum: Option<i64> = None;
                let mut serverless_apps_apm_apm_azure_appservice_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_apm_azure_azurefunction_instances_avg: Option<i64> =
                    None;
                let mut serverless_apps_apm_apm_azure_containerapp_instances_avg: Option<i64> =
                    None;
                let mut serverless_apps_apm_apm_fargate_ecs_tasks_avg: Option<i64> = None;
                let mut serverless_apps_apm_apm_gcp_cloudfunction_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_apm_gcp_cloudrun_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_avg: Option<i64> = None;
                let mut serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg: Option<i64> = None;
                let mut serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg: Option<
                    i64,
                > = None;
                let mut serverless_apps_apm_excl_fargate_avg: Option<i64> = None;
                let mut serverless_apps_azure_container_app_instances_avg: Option<i64> = None;
                let mut serverless_apps_azure_count_avg: Option<i64> = None;
                let mut serverless_apps_azure_function_app_instances_avg: Option<i64> = None;
                let mut serverless_apps_azure_web_app_instances_avg: Option<i64> = None;
                let mut serverless_apps_ecs_avg: Option<i64> = None;
                let mut serverless_apps_eks_avg: Option<i64> = None;
                let mut serverless_apps_excl_fargate_avg: Option<i64> = None;
                let mut serverless_apps_excl_fargate_azure_container_app_instances_avg: Option<
                    i64,
                > = None;
                let mut serverless_apps_excl_fargate_azure_function_app_instances_avg: Option<i64> =
                    None;
                let mut serverless_apps_excl_fargate_azure_web_app_instances_avg: Option<i64> =
                    None;
                let mut serverless_apps_excl_fargate_google_cloud_functions_instances_avg: Option<
                    i64,
                > = None;
                let mut serverless_apps_excl_fargate_google_cloud_run_instances_avg: Option<i64> =
                    None;
                let mut serverless_apps_google_cloud_functions_instances_avg: Option<i64> = None;
                let mut serverless_apps_google_cloud_run_instances_avg: Option<i64> = None;
                let mut serverless_apps_google_count_avg: Option<i64> = None;
                let mut serverless_apps_total_count_avg: Option<i64> = None;
                let mut siem_analyzed_logs_add_on_count_sum: Option<i64> = None;
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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "apm_azure_app_service_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_azure_app_service_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "apm_devsecops_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_devsecops_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "apm_fargate_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "appsec_fargate_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_fargate_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "asm_serverless_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            asm_serverless_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "audit_logs_lines_indexed_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_logs_lines_indexed_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "audit_trail_enabled_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_trail_enabled_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "avg_profiled_fargate_tasks" => {
                            if v.is_null() {
                                continue;
                            }
                            avg_profiled_fargate_tasks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "aws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "aws_lambda_func_count" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_lambda_func_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "aws_lambda_invocations_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_lambda_invocations_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "azure_app_service_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_app_service_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "billable_ingested_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            billable_ingested_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "bits_ai_investigations_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            bits_ai_investigations_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "browser_rum_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_lite_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "browser_rum_replay_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_replay_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "browser_rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_rum_units_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ci_pipeline_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ci_test_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ci_visibility_itr_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_itr_committers_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ci_visibility_pipeline_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_pipeline_committers_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ci_visibility_test_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_test_committers_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cloud_cost_management_aws_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_aws_host_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cloud_cost_management_azure_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_azure_host_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cloud_cost_management_gcp_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_gcp_host_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cloud_cost_management_host_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_host_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cloud_siem_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_siem_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "code_analysis_sa_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis_sa_committers_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "code_analysis_sca_committers_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis_sca_committers_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "code_security_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            code_security_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "container_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            container_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "container_excl_agent_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            container_excl_agent_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "container_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            container_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_container_enterprise_compliance_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_compliance_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_container_enterprise_cws_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_cws_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_container_enterprise_total_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_container_enterprise_total_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_aas_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aas_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_aws_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_aws_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_azure_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_azure_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_compliance_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_compliance_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_cws_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_cws_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_gcp_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_gcp_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "csm_host_enterprise_total_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            csm_host_enterprise_total_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_aas_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aas_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_aws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_aws_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_azure_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_azure_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_container_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_container_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_gcp_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_gcp_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cspm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "custom_ts_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_ts_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cws_container_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_container_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cws_fargate_task_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_fargate_task_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_jobs_monitoring_host_hr_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            data_jobs_monitoring_host_hr_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "date" => {
                            if v.is_null() {
                                continue;
                            }
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "dbm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "dbm_queries_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_queries_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_agent_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_agent_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_alibaba_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_alibaba_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_aws_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_aws_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_azure_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_azure_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_ent_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_ent_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_gcp_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_gcp_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_heroku_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_heroku_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_only_aas_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_only_aas_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_only_vsphere_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_only_vsphere_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_opentelemetry_apm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_opentelemetry_apm_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_opentelemetry_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_opentelemetry_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_pro_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_pro_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "eph_infra_host_proplus_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            eph_infra_host_proplus_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "error_tracking_apm_error_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_apm_error_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "error_tracking_error_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_error_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "error_tracking_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "error_tracking_rum_error_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_rum_error_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "event_management_correlation_correlated_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            event_management_correlation_correlated_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "event_management_correlation_correlated_related_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            event_management_correlation_correlated_related_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "event_management_correlation_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            event_management_correlation_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fargate_container_profiler_profiling_fargate_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_profiler_profiling_fargate_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fargate_container_profiler_profiling_fargate_eks_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_profiler_profiling_fargate_eks_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fargate_tasks_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fargate_tasks_count_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_tasks_count_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_compute_large_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_large_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_compute_medium_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_medium_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_compute_small_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_small_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_compute_xlarge_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_xlarge_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_compute_xsmall_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_compute_xsmall_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_starter_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_starter_storage_index_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_storage_index_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_logs_starter_storage_retention_adjustment_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_logs_starter_storage_retention_adjustment_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "flex_stored_logs_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            flex_stored_logs_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "forwarding_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            forwarding_events_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "gcp_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "heroku_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            heroku_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "incident_management_monthly_active_users_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_monthly_active_users_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_events_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "infra_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ingested_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_events_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "iot_device_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_device_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "iot_device_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_device_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "llm_observability_min_spend_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            llm_observability_min_spend_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "llm_observability_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            llm_observability_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_lite_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_android_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_android_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_flutter_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_flutter_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_ios_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_ios_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_reactnative_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_reactnative_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_roku_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_roku_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mobile_rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_rum_units_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ndm_netflow_events_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ndm_netflow_events_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "netflow_indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            netflow_indexed_events_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "network_device_wireless_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            network_device_wireless_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "npm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "observability_pipelines_bytes_processed_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            observability_pipelines_bytes_processed_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "oci_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            oci_host_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "oci_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            oci_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "on_call_seat_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            on_call_seat_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "online_archive_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_events_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "opentelemetry_apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_apm_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "opentelemetry_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            opentelemetry_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "orgs" => {
                            if v.is_null() {
                                continue;
                            }
                            orgs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "product_analytics_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "profiling_aas_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_aas_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "profiling_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            profiling_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "published_app_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            published_app_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_browser_and_mobile_session_count" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_and_mobile_session_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_browser_legacy_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_legacy_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_browser_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_lite_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_browser_replay_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_replay_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_indexed_sessions_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_indexed_sessions_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_ingested_sessions_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_ingested_sessions_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_lite_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_lite_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_legacy_session_count_android_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_android_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_legacy_session_count_flutter_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_flutter_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_legacy_session_count_ios_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_ios_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_legacy_session_count_reactnative_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_reactnative_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_legacy_session_count_roku_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_legacy_session_count_roku_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_android_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_android_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_flutter_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_flutter_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_ios_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_ios_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_kotlinmultiplatform_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_kotlinmultiplatform_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_reactnative_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_reactnative_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_roku_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_roku_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_lite_session_count_unity_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_lite_session_count_unity_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_replay_session_count_android_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_replay_session_count_android_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_replay_session_count_ios_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_replay_session_count_ios_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_replay_session_count_kotlinmultiplatform_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_replay_session_count_kotlinmultiplatform_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_mobile_replay_session_count_reactnative_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_mobile_replay_session_count_reactnative_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_replay_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_replay_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_session_replay_add_on_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_session_replay_add_on_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_total_session_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_total_session_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_units_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sca_fargate_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sca_fargate_count_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_count_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sds_apm_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_apm_scanned_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sds_events_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_events_scanned_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sds_logs_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_logs_scanned_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sds_rum_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_rum_scanned_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sds_total_scanned_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_total_scanned_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_azure_appservice_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_azure_appservice_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_azure_azurefunction_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_azure_azurefunction_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_azure_containerapp_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_azure_containerapp_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_fargate_ecs_tasks_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_fargate_ecs_tasks_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_gcp_cloudfunction_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_gcp_cloudfunction_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_apm_gcp_cloudrun_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_apm_gcp_cloudrun_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_apm_excl_fargate_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_apm_excl_fargate_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_azure_container_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_container_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_azure_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_azure_function_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_function_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_azure_web_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_azure_web_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_ecs_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_ecs_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_eks_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_eks_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_azure_container_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_azure_container_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_azure_function_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_azure_function_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_azure_web_app_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_azure_web_app_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_google_cloud_functions_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_google_cloud_functions_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_excl_fargate_google_cloud_run_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_excl_fargate_google_cloud_run_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_google_cloud_functions_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_google_cloud_functions_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_google_cloud_run_instances_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_google_cloud_run_instances_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_google_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_google_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "serverless_apps_total_count_avg" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_total_count_avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "siem_analyzed_logs_add_on_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_analyzed_logs_add_on_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_browser_check_calls_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_browser_check_calls_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_check_calls_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_check_calls_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_mobile_test_runs_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_mobile_test_runs_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_parallel_testing_max_slots_hwm" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_parallel_testing_max_slots_hwm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "trace_search_indexed_events_count_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_search_indexed_events_count_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "twol_ingested_events_bytes_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            twol_ingested_events_bytes_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "universal_service_monitoring_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            universal_service_monitoring_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "vsphere_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            vsphere_host_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "vuln_management_host_count_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_management_host_count_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "workflow_executions_usage_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_executions_usage_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                #[allow(deprecated)]
                let content = UsageSummaryDate {
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
                    bits_ai_investigations_sum,
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
                    code_analysis_sa_committers_hwm,
                    code_analysis_sca_committers_hwm,
                    code_security_host_top99p,
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
                    custom_ts_avg,
                    cws_container_count_avg,
                    cws_fargate_task_avg,
                    cws_host_top99p,
                    data_jobs_monitoring_host_hr_sum,
                    date,
                    dbm_host_top99p,
                    dbm_queries_count_avg,
                    eph_infra_host_agent_sum,
                    eph_infra_host_alibaba_sum,
                    eph_infra_host_aws_sum,
                    eph_infra_host_azure_sum,
                    eph_infra_host_ent_sum,
                    eph_infra_host_gcp_sum,
                    eph_infra_host_heroku_sum,
                    eph_infra_host_only_aas_sum,
                    eph_infra_host_only_vsphere_sum,
                    eph_infra_host_opentelemetry_apm_sum,
                    eph_infra_host_opentelemetry_sum,
                    eph_infra_host_pro_sum,
                    eph_infra_host_proplus_sum,
                    error_tracking_apm_error_events_sum,
                    error_tracking_error_events_sum,
                    error_tracking_events_sum,
                    error_tracking_rum_error_events_sum,
                    event_management_correlation_correlated_events_sum,
                    event_management_correlation_correlated_related_events_sum,
                    event_management_correlation_sum,
                    fargate_container_profiler_profiling_fargate_avg,
                    fargate_container_profiler_profiling_fargate_eks_avg,
                    fargate_tasks_count_avg,
                    fargate_tasks_count_hwm,
                    flex_logs_compute_large_avg,
                    flex_logs_compute_medium_avg,
                    flex_logs_compute_small_avg,
                    flex_logs_compute_xlarge_avg,
                    flex_logs_compute_xsmall_avg,
                    flex_logs_starter_avg,
                    flex_logs_starter_storage_index_avg,
                    flex_logs_starter_storage_retention_adjustment_avg,
                    flex_stored_logs_avg,
                    forwarding_events_bytes_sum,
                    gcp_host_top99p,
                    heroku_host_top99p,
                    incident_management_monthly_active_users_hwm,
                    indexed_events_count_sum,
                    infra_host_top99p,
                    ingested_events_bytes_sum,
                    iot_device_sum,
                    iot_device_top99p,
                    llm_observability_min_spend_sum,
                    llm_observability_sum,
                    mobile_rum_lite_session_count_sum,
                    mobile_rum_session_count_android_sum,
                    mobile_rum_session_count_flutter_sum,
                    mobile_rum_session_count_ios_sum,
                    mobile_rum_session_count_reactnative_sum,
                    mobile_rum_session_count_roku_sum,
                    mobile_rum_session_count_sum,
                    mobile_rum_units_sum,
                    ndm_netflow_events_sum,
                    netflow_indexed_events_count_sum,
                    network_device_wireless_top99p,
                    npm_host_top99p,
                    observability_pipelines_bytes_processed_sum,
                    oci_host_sum,
                    oci_host_top99p,
                    on_call_seat_hwm,
                    online_archive_events_count_sum,
                    opentelemetry_apm_host_top99p,
                    opentelemetry_host_top99p,
                    orgs,
                    product_analytics_sum,
                    profiling_aas_count_top99p,
                    profiling_host_top99p,
                    published_app_hwm,
                    rum_browser_and_mobile_session_count,
                    rum_browser_legacy_session_count_sum,
                    rum_browser_lite_session_count_sum,
                    rum_browser_replay_session_count_sum,
                    rum_indexed_sessions_sum,
                    rum_ingested_sessions_sum,
                    rum_lite_session_count_sum,
                    rum_mobile_legacy_session_count_android_sum,
                    rum_mobile_legacy_session_count_flutter_sum,
                    rum_mobile_legacy_session_count_ios_sum,
                    rum_mobile_legacy_session_count_reactnative_sum,
                    rum_mobile_legacy_session_count_roku_sum,
                    rum_mobile_lite_session_count_android_sum,
                    rum_mobile_lite_session_count_flutter_sum,
                    rum_mobile_lite_session_count_ios_sum,
                    rum_mobile_lite_session_count_kotlinmultiplatform_sum,
                    rum_mobile_lite_session_count_reactnative_sum,
                    rum_mobile_lite_session_count_roku_sum,
                    rum_mobile_lite_session_count_unity_sum,
                    rum_mobile_replay_session_count_android_sum,
                    rum_mobile_replay_session_count_ios_sum,
                    rum_mobile_replay_session_count_kotlinmultiplatform_sum,
                    rum_mobile_replay_session_count_reactnative_sum,
                    rum_replay_session_count_sum,
                    rum_session_count_sum,
                    rum_session_replay_add_on_sum,
                    rum_total_session_count_sum,
                    rum_units_sum,
                    sca_fargate_count_avg,
                    sca_fargate_count_hwm,
                    sds_apm_scanned_bytes_sum,
                    sds_events_scanned_bytes_sum,
                    sds_logs_scanned_bytes_sum,
                    sds_rum_scanned_bytes_sum,
                    sds_total_scanned_bytes_sum,
                    serverless_apps_apm_apm_azure_appservice_instances_avg,
                    serverless_apps_apm_apm_azure_azurefunction_instances_avg,
                    serverless_apps_apm_apm_azure_containerapp_instances_avg,
                    serverless_apps_apm_apm_fargate_ecs_tasks_avg,
                    serverless_apps_apm_apm_gcp_cloudfunction_instances_avg,
                    serverless_apps_apm_apm_gcp_cloudrun_instances_avg,
                    serverless_apps_apm_avg,
                    serverless_apps_apm_excl_fargate_apm_azure_appservice_instances_avg,
                    serverless_apps_apm_excl_fargate_apm_azure_azurefunction_instances_avg,
                    serverless_apps_apm_excl_fargate_apm_azure_containerapp_instances_avg,
                    serverless_apps_apm_excl_fargate_apm_gcp_cloudfunction_instances_avg,
                    serverless_apps_apm_excl_fargate_apm_gcp_cloudrun_instances_avg,
                    serverless_apps_apm_excl_fargate_avg,
                    serverless_apps_azure_container_app_instances_avg,
                    serverless_apps_azure_count_avg,
                    serverless_apps_azure_function_app_instances_avg,
                    serverless_apps_azure_web_app_instances_avg,
                    serverless_apps_ecs_avg,
                    serverless_apps_eks_avg,
                    serverless_apps_excl_fargate_avg,
                    serverless_apps_excl_fargate_azure_container_app_instances_avg,
                    serverless_apps_excl_fargate_azure_function_app_instances_avg,
                    serverless_apps_excl_fargate_azure_web_app_instances_avg,
                    serverless_apps_excl_fargate_google_cloud_functions_instances_avg,
                    serverless_apps_excl_fargate_google_cloud_run_instances_avg,
                    serverless_apps_google_cloud_functions_instances_avg,
                    serverless_apps_google_cloud_run_instances_avg,
                    serverless_apps_google_count_avg,
                    serverless_apps_total_count_avg,
                    siem_analyzed_logs_add_on_count_sum,
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
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSummaryDateVisitor)
    }
}
