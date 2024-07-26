// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fields in Usage Summary by tag(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonthlyUsageAttributionValues {
    /// The percentage of synthetic API test usage by tag(s).
    #[serde(rename = "api_percentage")]
    pub api_percentage: Option<f64>,
    /// The synthetic API test usage by tag(s).
    #[serde(rename = "api_usage")]
    pub api_usage: Option<f64>,
    /// The percentage of APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_percentage")]
    pub apm_fargate_percentage: Option<f64>,
    /// The APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_usage")]
    pub apm_fargate_usage: Option<f64>,
    /// The percentage of APM host usage by tag(s).
    #[serde(rename = "apm_host_percentage")]
    pub apm_host_percentage: Option<f64>,
    /// The APM host usage by tag(s).
    #[serde(rename = "apm_host_usage")]
    pub apm_host_usage: Option<f64>,
    /// The percentage of APM and Universal Service Monitoring host usage by tag(s).
    #[serde(rename = "apm_usm_percentage")]
    pub apm_usm_percentage: Option<f64>,
    /// The APM and Universal Service Monitoring host usage by tag(s).
    #[serde(rename = "apm_usm_usage")]
    pub apm_usm_usage: Option<f64>,
    /// The percentage of Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_percentage")]
    pub appsec_fargate_percentage: Option<f64>,
    /// The Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_usage")]
    pub appsec_fargate_usage: Option<f64>,
    /// The percentage of Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_percentage")]
    pub appsec_percentage: Option<f64>,
    /// The Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_usage")]
    pub appsec_usage: Option<f64>,
    /// The percentage of Application Security Monitoring Serverless traced invocations usage by tag(s).
    #[serde(rename = "asm_serverless_traced_invocations_percentage")]
    pub asm_serverless_traced_invocations_percentage: Option<f64>,
    /// The Application Security Monitoring Serverless traced invocations usage by tag(s).
    #[serde(rename = "asm_serverless_traced_invocations_usage")]
    pub asm_serverless_traced_invocations_usage: Option<f64>,
    /// The percentage of synthetic browser test usage by tag(s).
    #[serde(rename = "browser_percentage")]
    pub browser_percentage: Option<f64>,
    /// The synthetic browser test usage by tag(s).
    #[serde(rename = "browser_usage")]
    pub browser_usage: Option<f64>,
    /// The percentage of CI Pipeline Indexed Spans usage by tag(s).
    #[serde(rename = "ci_pipeline_indexed_spans_percentage")]
    pub ci_pipeline_indexed_spans_percentage: Option<f64>,
    /// The total CI Pipeline Indexed Spans usage by tag(s).
    #[serde(rename = "ci_pipeline_indexed_spans_usage")]
    pub ci_pipeline_indexed_spans_usage: Option<f64>,
    /// The percentage of CI Test Indexed Spans usage by tag(s).
    #[serde(rename = "ci_test_indexed_spans_percentage")]
    pub ci_test_indexed_spans_percentage: Option<f64>,
    /// The total CI Test Indexed Spans usage by tag(s).
    #[serde(rename = "ci_test_indexed_spans_usage")]
    pub ci_test_indexed_spans_usage: Option<f64>,
    /// The percentage of Git committers for Intelligent Test Runner usage by tag(s).
    #[serde(rename = "ci_visibility_itr_percentage")]
    pub ci_visibility_itr_percentage: Option<f64>,
    /// The Git committers for Intelligent Test Runner usage by tag(s).
    #[serde(rename = "ci_visibility_itr_usage")]
    pub ci_visibility_itr_usage: Option<f64>,
    /// The percentage of Cloud Security Information and Event Management usage by tag(s).
    #[serde(rename = "cloud_siem_percentage")]
    pub cloud_siem_percentage: Option<f64>,
    /// The Cloud Security Information and Event Management usage by tag(s).
    #[serde(rename = "cloud_siem_usage")]
    pub cloud_siem_usage: Option<f64>,
    /// The percentage of container usage without the Datadog Agent by tag(s).
    #[serde(rename = "container_excl_agent_percentage")]
    pub container_excl_agent_percentage: Option<f64>,
    /// The container usage without the Datadog Agent by tag(s).
    #[serde(rename = "container_excl_agent_usage")]
    pub container_excl_agent_usage: Option<f64>,
    /// The percentage of container usage by tag(s).
    #[serde(rename = "container_percentage")]
    pub container_percentage: Option<f64>,
    /// The container usage by tag(s).
    #[serde(rename = "container_usage")]
    pub container_usage: Option<f64>,
    /// The percentage of Cloud Security Management Pro container usage by tag(s).
    #[serde(rename = "cspm_containers_percentage")]
    pub cspm_containers_percentage: Option<f64>,
    /// The Cloud Security Management Pro container usage by tag(s).
    #[serde(rename = "cspm_containers_usage")]
    pub cspm_containers_usage: Option<f64>,
    /// The percentage of Cloud Security Management Pro host usage by tag(s).
    #[serde(rename = "cspm_hosts_percentage")]
    pub cspm_hosts_percentage: Option<f64>,
    /// The Cloud Security Management Pro host usage by tag(s).
    #[serde(rename = "cspm_hosts_usage")]
    pub cspm_hosts_usage: Option<f64>,
    /// The percentage of Custom Events usage by tag(s).
    #[serde(rename = "custom_event_percentage")]
    pub custom_event_percentage: Option<f64>,
    /// The total Custom Events usage by tag(s).
    #[serde(rename = "custom_event_usage")]
    pub custom_event_usage: Option<f64>,
    /// The percentage of ingested custom metrics usage by tag(s).
    #[serde(rename = "custom_ingested_timeseries_percentage")]
    pub custom_ingested_timeseries_percentage: Option<f64>,
    /// The ingested custom metrics usage by tag(s).
    #[serde(rename = "custom_ingested_timeseries_usage")]
    pub custom_ingested_timeseries_usage: Option<f64>,
    /// The percentage of indexed custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_percentage")]
    pub custom_timeseries_percentage: Option<f64>,
    /// The indexed custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_usage")]
    pub custom_timeseries_usage: Option<f64>,
    /// The percentage of Cloud Workload Security container usage by tag(s).
    #[serde(rename = "cws_containers_percentage")]
    pub cws_containers_percentage: Option<f64>,
    /// The Cloud Workload Security container usage by tag(s).
    #[serde(rename = "cws_containers_usage")]
    pub cws_containers_usage: Option<f64>,
    /// The percentage of Cloud Workload Security host usage by tag(s).
    #[serde(rename = "cws_hosts_percentage")]
    pub cws_hosts_percentage: Option<f64>,
    /// The Cloud Workload Security host usage by tag(s).
    #[serde(rename = "cws_hosts_usage")]
    pub cws_hosts_usage: Option<f64>,
    /// The percentage of Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_percentage")]
    pub dbm_hosts_percentage: Option<f64>,
    /// The Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_usage")]
    pub dbm_hosts_usage: Option<f64>,
    /// The percentage of Database Monitoring queries usage by tag(s).
    #[serde(rename = "dbm_queries_percentage")]
    pub dbm_queries_percentage: Option<f64>,
    /// The Database Monitoring queries usage by tag(s).
    #[serde(rename = "dbm_queries_usage")]
    pub dbm_queries_usage: Option<f64>,
    /// The percentage of error tracking events usage by tag(s).
    #[serde(rename = "error_tracking_percentage")]
    pub error_tracking_percentage: Option<f64>,
    /// The error tracking events usage by tag(s).
    #[serde(rename = "error_tracking_usage")]
    pub error_tracking_usage: Option<f64>,
    /// The percentage of estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_percentage")]
    pub estimated_indexed_logs_percentage: Option<f64>,
    /// The estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_usage")]
    pub estimated_indexed_logs_usage: Option<f64>,
    /// The percentage of estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_percentage")]
    pub estimated_indexed_spans_percentage: Option<f64>,
    /// The estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_usage")]
    pub estimated_indexed_spans_usage: Option<f64>,
    /// The percentage of estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_percentage")]
    pub estimated_ingested_logs_percentage: Option<f64>,
    /// The estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_usage")]
    pub estimated_ingested_logs_usage: Option<f64>,
    /// The percentage of estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_percentage")]
    pub estimated_ingested_spans_percentage: Option<f64>,
    /// The estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_usage")]
    pub estimated_ingested_spans_usage: Option<f64>,
    /// The percentage of estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_percentage")]
    pub estimated_rum_sessions_percentage: Option<f64>,
    /// The estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_usage")]
    pub estimated_rum_sessions_usage: Option<f64>,
    /// The percentage of Fargate usage by tags.
    #[serde(rename = "fargate_percentage")]
    pub fargate_percentage: Option<f64>,
    /// The Fargate usage by tags.
    #[serde(rename = "fargate_usage")]
    pub fargate_usage: Option<f64>,
    /// The percentage of Lambda function usage by tag(s).
    #[serde(rename = "functions_percentage")]
    pub functions_percentage: Option<f64>,
    /// The Lambda function usage by tag(s).
    #[serde(rename = "functions_usage")]
    pub functions_usage: Option<f64>,
    /// The percentage of Incident Management monthly active users usage by tag(s).
    #[serde(rename = "incident_management_monthly_active_users_percentage")]
    pub incident_management_monthly_active_users_percentage: Option<f64>,
    /// The Incident Management monthly active users usage by tag(s).
    #[serde(rename = "incident_management_monthly_active_users_usage")]
    pub incident_management_monthly_active_users_usage: Option<f64>,
    /// The percentage of APM Indexed Spans usage by tag(s).
    #[serde(rename = "indexed_spans_percentage")]
    pub indexed_spans_percentage: Option<f64>,
    /// The total APM Indexed Spans usage by tag(s).
    #[serde(rename = "indexed_spans_usage")]
    pub indexed_spans_usage: Option<f64>,
    /// The percentage of infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_percentage")]
    pub infra_host_percentage: Option<f64>,
    /// The infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_usage")]
    pub infra_host_usage: Option<f64>,
    /// The percentage of Ingested Logs usage by tag(s).
    #[serde(rename = "ingested_logs_bytes_percentage")]
    pub ingested_logs_bytes_percentage: Option<f64>,
    /// The total Ingested Logs usage by tag(s).
    #[serde(rename = "ingested_logs_bytes_usage")]
    pub ingested_logs_bytes_usage: Option<f64>,
    /// The percentage of APM Ingested Spans usage by tag(s).
    #[serde(rename = "ingested_spans_bytes_percentage")]
    pub ingested_spans_bytes_percentage: Option<f64>,
    /// The total APM Ingested Spans usage by tag(s).
    #[serde(rename = "ingested_spans_bytes_usage")]
    pub ingested_spans_bytes_usage: Option<f64>,
    /// The percentage of Lambda invocation usage by tag(s).
    #[serde(rename = "invocations_percentage")]
    pub invocations_percentage: Option<f64>,
    /// The Lambda invocation usage by tag(s).
    #[serde(rename = "invocations_usage")]
    pub invocations_usage: Option<f64>,
    /// The percentage of Serverless APM usage by tag(s).
    #[serde(rename = "lambda_traced_invocations_percentage")]
    pub lambda_traced_invocations_percentage: Option<f64>,
    /// The Serverless APM usage by tag(s).
    #[serde(rename = "lambda_traced_invocations_usage")]
    pub lambda_traced_invocations_usage: Option<f64>,
    /// The percentage of Indexed Logs (15-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_15day_percentage")]
    pub logs_indexed_15day_percentage: Option<f64>,
    /// The total Indexed Logs (15-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_15day_usage")]
    pub logs_indexed_15day_usage: Option<f64>,
    /// The percentage of Indexed Logs (180-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_180day_percentage")]
    pub logs_indexed_180day_percentage: Option<f64>,
    /// The total Indexed Logs (180-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_180day_usage")]
    pub logs_indexed_180day_usage: Option<f64>,
    /// The percentage of Indexed Logs (1-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_1day_percentage")]
    pub logs_indexed_1day_percentage: Option<f64>,
    /// The total Indexed Logs (1-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_1day_usage")]
    pub logs_indexed_1day_usage: Option<f64>,
    /// The percentage of Indexed Logs (30-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_30day_percentage")]
    pub logs_indexed_30day_percentage: Option<f64>,
    /// The total Indexed Logs (30-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_30day_usage")]
    pub logs_indexed_30day_usage: Option<f64>,
    /// The percentage of Indexed Logs (360-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_360day_percentage")]
    pub logs_indexed_360day_percentage: Option<f64>,
    /// The total Indexed Logs (360-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_360day_usage")]
    pub logs_indexed_360day_usage: Option<f64>,
    /// The percentage of Indexed Logs (3-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_3day_percentage")]
    pub logs_indexed_3day_percentage: Option<f64>,
    /// The total Indexed Logs (3-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_3day_usage")]
    pub logs_indexed_3day_usage: Option<f64>,
    /// The percentage of Indexed Logs (45-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_45day_percentage")]
    pub logs_indexed_45day_percentage: Option<f64>,
    /// The total Indexed Logs (45-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_45day_usage")]
    pub logs_indexed_45day_usage: Option<f64>,
    /// The percentage of Indexed Logs (60-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_60day_percentage")]
    pub logs_indexed_60day_percentage: Option<f64>,
    /// The total Indexed Logs (60-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_60day_usage")]
    pub logs_indexed_60day_usage: Option<f64>,
    /// The percentage of Indexed Logs (7-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_7day_percentage")]
    pub logs_indexed_7day_percentage: Option<f64>,
    /// The total Indexed Logs (7-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_7day_usage")]
    pub logs_indexed_7day_usage: Option<f64>,
    /// The percentage of Indexed Logs (90-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_90day_percentage")]
    pub logs_indexed_90day_percentage: Option<f64>,
    /// The total Indexed Logs (90-day Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_90day_usage")]
    pub logs_indexed_90day_usage: Option<f64>,
    /// The percentage of Indexed Logs (Custom Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_custom_retention_percentage")]
    pub logs_indexed_custom_retention_percentage: Option<f64>,
    /// The total Indexed Logs (Custom Retention) usage by tag(s).
    #[serde(rename = "logs_indexed_custom_retention_usage")]
    pub logs_indexed_custom_retention_usage: Option<f64>,
    /// The percentage of Synthetic mobile application test usage by tag(s).
    #[serde(rename = "mobile_app_testing_percentage")]
    pub mobile_app_testing_percentage: Option<f64>,
    /// The Synthetic mobile application test usage by tag(s).
    #[serde(rename = "mobile_app_testing_usage")]
    pub mobile_app_testing_usage: Option<f64>,
    /// The percentage of Network Device Monitoring NetFlow usage by tag(s).
    #[serde(rename = "ndm_netflow_percentage")]
    pub ndm_netflow_percentage: Option<f64>,
    /// The Network Device Monitoring NetFlow usage by tag(s).
    #[serde(rename = "ndm_netflow_usage")]
    pub ndm_netflow_usage: Option<f64>,
    /// The percentage of network host usage by tag(s).
    #[serde(rename = "npm_host_percentage")]
    pub npm_host_percentage: Option<f64>,
    /// The network host usage by tag(s).
    #[serde(rename = "npm_host_usage")]
    pub npm_host_usage: Option<f64>,
    /// The percentage of observability pipeline bytes usage by tag(s).
    #[serde(rename = "obs_pipeline_bytes_percentage")]
    pub obs_pipeline_bytes_percentage: Option<f64>,
    /// The observability pipeline bytes usage by tag(s).
    #[serde(rename = "obs_pipeline_bytes_usage")]
    pub obs_pipeline_bytes_usage: Option<f64>,
    /// The percentage of observability pipeline per core usage by tag(s).
    #[serde(rename = "obs_pipelines_vcpu_percentage")]
    pub obs_pipelines_vcpu_percentage: Option<f64>,
    /// The observability pipeline per core usage by tag(s).
    #[serde(rename = "obs_pipelines_vcpu_usage")]
    pub obs_pipelines_vcpu_usage: Option<f64>,
    /// The percentage of online archive usage by tag(s).
    #[serde(rename = "online_archive_percentage")]
    pub online_archive_percentage: Option<f64>,
    /// The online archive usage by tag(s).
    #[serde(rename = "online_archive_usage")]
    pub online_archive_usage: Option<f64>,
    /// The percentage of profiled container usage by tag(s).
    #[serde(rename = "profiled_container_percentage")]
    pub profiled_container_percentage: Option<f64>,
    /// The profiled container usage by tag(s).
    #[serde(rename = "profiled_container_usage")]
    pub profiled_container_usage: Option<f64>,
    /// The percentage of profiled Fargate task usage by tag(s).
    #[serde(rename = "profiled_fargate_percentage")]
    pub profiled_fargate_percentage: Option<f64>,
    /// The profiled Fargate task usage by tag(s).
    #[serde(rename = "profiled_fargate_usage")]
    pub profiled_fargate_usage: Option<f64>,
    /// The percentage of profiled hosts usage by tag(s).
    #[serde(rename = "profiled_host_percentage")]
    pub profiled_host_percentage: Option<f64>,
    /// The profiled hosts usage by tag(s).
    #[serde(rename = "profiled_host_usage")]
    pub profiled_host_usage: Option<f64>,
    /// The percentage of RUM Browser and Mobile usage by tag(s).
    #[serde(rename = "rum_browser_mobile_sessions_percentage")]
    pub rum_browser_mobile_sessions_percentage: Option<f64>,
    /// The total RUM Browser and Mobile usage by tag(s).
    #[serde(rename = "rum_browser_mobile_sessions_usage")]
    pub rum_browser_mobile_sessions_usage: Option<f64>,
    /// The percentage of RUM Session Replay usage by tag(s).
    #[serde(rename = "rum_replay_sessions_percentage")]
    pub rum_replay_sessions_percentage: Option<f64>,
    /// The total RUM Session Replay usage by tag(s).
    #[serde(rename = "rum_replay_sessions_usage")]
    pub rum_replay_sessions_usage: Option<f64>,
    /// The percentage of Software Composition Analysis Fargate task usage by tag(s).
    #[serde(rename = "sca_fargate_percentage")]
    pub sca_fargate_percentage: Option<f64>,
    /// The total Software Composition Analysis Fargate task usage by tag(s).
    #[serde(rename = "sca_fargate_usage")]
    pub sca_fargate_usage: Option<f64>,
    /// The percentage of Sensitive Data Scanner usage by tag(s).
    #[serde(rename = "sds_scanned_bytes_percentage")]
    pub sds_scanned_bytes_percentage: Option<f64>,
    /// The total Sensitive Data Scanner usage by tag(s).
    #[serde(rename = "sds_scanned_bytes_usage")]
    pub sds_scanned_bytes_usage: Option<f64>,
    /// The percentage of Serverless Apps usage by tag(s).
    #[serde(rename = "serverless_apps_percentage")]
    pub serverless_apps_percentage: Option<f64>,
    /// The total Serverless Apps usage by tag(s).
    #[serde(rename = "serverless_apps_usage")]
    pub serverless_apps_usage: Option<f64>,
    /// The percentage of log events analyzed by Cloud SIEM usage by tag(s).
    #[serde(rename = "siem_analyzed_logs_add_on_percentage")]
    pub siem_analyzed_logs_add_on_percentage: Option<f64>,
    /// The log events analyzed by Cloud SIEM usage by tag(s).
    #[serde(rename = "siem_analyzed_logs_add_on_usage")]
    pub siem_analyzed_logs_add_on_usage: Option<f64>,
    /// The percentage of SIEM usage by tag(s).
    #[serde(rename = "siem_ingested_bytes_percentage")]
    pub siem_ingested_bytes_percentage: Option<f64>,
    /// The total SIEM usage by tag(s).
    #[serde(rename = "siem_ingested_bytes_usage")]
    pub siem_ingested_bytes_usage: Option<f64>,
    /// The percentage of network device usage by tag(s).
    #[serde(rename = "snmp_percentage")]
    pub snmp_percentage: Option<f64>,
    /// The network device usage by tag(s).
    #[serde(rename = "snmp_usage")]
    pub snmp_usage: Option<f64>,
    /// The percentage of universal service monitoring usage by tag(s).
    #[serde(rename = "universal_service_monitoring_percentage")]
    pub universal_service_monitoring_percentage: Option<f64>,
    /// The universal service monitoring usage by tag(s).
    #[serde(rename = "universal_service_monitoring_usage")]
    pub universal_service_monitoring_usage: Option<f64>,
    /// The percentage of Application Vulnerability Management usage by tag(s).
    #[serde(rename = "vuln_management_hosts_percentage")]
    pub vuln_management_hosts_percentage: Option<f64>,
    /// The Application Vulnerability Management usage by tag(s).
    #[serde(rename = "vuln_management_hosts_usage")]
    pub vuln_management_hosts_usage: Option<f64>,
    /// The percentage of workflow executions usage by tag(s).
    #[serde(rename = "workflow_executions_percentage")]
    pub workflow_executions_percentage: Option<f64>,
    /// The total workflow executions usage by tag(s).
    #[serde(rename = "workflow_executions_usage")]
    pub workflow_executions_usage: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonthlyUsageAttributionValues {
    pub fn new() -> MonthlyUsageAttributionValues {
        MonthlyUsageAttributionValues {
            api_percentage: None,
            api_usage: None,
            apm_fargate_percentage: None,
            apm_fargate_usage: None,
            apm_host_percentage: None,
            apm_host_usage: None,
            apm_usm_percentage: None,
            apm_usm_usage: None,
            appsec_fargate_percentage: None,
            appsec_fargate_usage: None,
            appsec_percentage: None,
            appsec_usage: None,
            asm_serverless_traced_invocations_percentage: None,
            asm_serverless_traced_invocations_usage: None,
            browser_percentage: None,
            browser_usage: None,
            ci_pipeline_indexed_spans_percentage: None,
            ci_pipeline_indexed_spans_usage: None,
            ci_test_indexed_spans_percentage: None,
            ci_test_indexed_spans_usage: None,
            ci_visibility_itr_percentage: None,
            ci_visibility_itr_usage: None,
            cloud_siem_percentage: None,
            cloud_siem_usage: None,
            container_excl_agent_percentage: None,
            container_excl_agent_usage: None,
            container_percentage: None,
            container_usage: None,
            cspm_containers_percentage: None,
            cspm_containers_usage: None,
            cspm_hosts_percentage: None,
            cspm_hosts_usage: None,
            custom_event_percentage: None,
            custom_event_usage: None,
            custom_ingested_timeseries_percentage: None,
            custom_ingested_timeseries_usage: None,
            custom_timeseries_percentage: None,
            custom_timeseries_usage: None,
            cws_containers_percentage: None,
            cws_containers_usage: None,
            cws_hosts_percentage: None,
            cws_hosts_usage: None,
            dbm_hosts_percentage: None,
            dbm_hosts_usage: None,
            dbm_queries_percentage: None,
            dbm_queries_usage: None,
            error_tracking_percentage: None,
            error_tracking_usage: None,
            estimated_indexed_logs_percentage: None,
            estimated_indexed_logs_usage: None,
            estimated_indexed_spans_percentage: None,
            estimated_indexed_spans_usage: None,
            estimated_ingested_logs_percentage: None,
            estimated_ingested_logs_usage: None,
            estimated_ingested_spans_percentage: None,
            estimated_ingested_spans_usage: None,
            estimated_rum_sessions_percentage: None,
            estimated_rum_sessions_usage: None,
            fargate_percentage: None,
            fargate_usage: None,
            functions_percentage: None,
            functions_usage: None,
            incident_management_monthly_active_users_percentage: None,
            incident_management_monthly_active_users_usage: None,
            indexed_spans_percentage: None,
            indexed_spans_usage: None,
            infra_host_percentage: None,
            infra_host_usage: None,
            ingested_logs_bytes_percentage: None,
            ingested_logs_bytes_usage: None,
            ingested_spans_bytes_percentage: None,
            ingested_spans_bytes_usage: None,
            invocations_percentage: None,
            invocations_usage: None,
            lambda_traced_invocations_percentage: None,
            lambda_traced_invocations_usage: None,
            logs_indexed_15day_percentage: None,
            logs_indexed_15day_usage: None,
            logs_indexed_180day_percentage: None,
            logs_indexed_180day_usage: None,
            logs_indexed_1day_percentage: None,
            logs_indexed_1day_usage: None,
            logs_indexed_30day_percentage: None,
            logs_indexed_30day_usage: None,
            logs_indexed_360day_percentage: None,
            logs_indexed_360day_usage: None,
            logs_indexed_3day_percentage: None,
            logs_indexed_3day_usage: None,
            logs_indexed_45day_percentage: None,
            logs_indexed_45day_usage: None,
            logs_indexed_60day_percentage: None,
            logs_indexed_60day_usage: None,
            logs_indexed_7day_percentage: None,
            logs_indexed_7day_usage: None,
            logs_indexed_90day_percentage: None,
            logs_indexed_90day_usage: None,
            logs_indexed_custom_retention_percentage: None,
            logs_indexed_custom_retention_usage: None,
            mobile_app_testing_percentage: None,
            mobile_app_testing_usage: None,
            ndm_netflow_percentage: None,
            ndm_netflow_usage: None,
            npm_host_percentage: None,
            npm_host_usage: None,
            obs_pipeline_bytes_percentage: None,
            obs_pipeline_bytes_usage: None,
            obs_pipelines_vcpu_percentage: None,
            obs_pipelines_vcpu_usage: None,
            online_archive_percentage: None,
            online_archive_usage: None,
            profiled_container_percentage: None,
            profiled_container_usage: None,
            profiled_fargate_percentage: None,
            profiled_fargate_usage: None,
            profiled_host_percentage: None,
            profiled_host_usage: None,
            rum_browser_mobile_sessions_percentage: None,
            rum_browser_mobile_sessions_usage: None,
            rum_replay_sessions_percentage: None,
            rum_replay_sessions_usage: None,
            sca_fargate_percentage: None,
            sca_fargate_usage: None,
            sds_scanned_bytes_percentage: None,
            sds_scanned_bytes_usage: None,
            serverless_apps_percentage: None,
            serverless_apps_usage: None,
            siem_analyzed_logs_add_on_percentage: None,
            siem_analyzed_logs_add_on_usage: None,
            siem_ingested_bytes_percentage: None,
            siem_ingested_bytes_usage: None,
            snmp_percentage: None,
            snmp_usage: None,
            universal_service_monitoring_percentage: None,
            universal_service_monitoring_usage: None,
            vuln_management_hosts_percentage: None,
            vuln_management_hosts_usage: None,
            workflow_executions_percentage: None,
            workflow_executions_usage: None,
            _unparsed: false,
        }
    }

    pub fn api_percentage(mut self, value: f64) -> Self {
        self.api_percentage = Some(value);
        self
    }

    pub fn api_usage(mut self, value: f64) -> Self {
        self.api_usage = Some(value);
        self
    }

    pub fn apm_fargate_percentage(mut self, value: f64) -> Self {
        self.apm_fargate_percentage = Some(value);
        self
    }

    pub fn apm_fargate_usage(mut self, value: f64) -> Self {
        self.apm_fargate_usage = Some(value);
        self
    }

    pub fn apm_host_percentage(mut self, value: f64) -> Self {
        self.apm_host_percentage = Some(value);
        self
    }

    pub fn apm_host_usage(mut self, value: f64) -> Self {
        self.apm_host_usage = Some(value);
        self
    }

    pub fn apm_usm_percentage(mut self, value: f64) -> Self {
        self.apm_usm_percentage = Some(value);
        self
    }

    pub fn apm_usm_usage(mut self, value: f64) -> Self {
        self.apm_usm_usage = Some(value);
        self
    }

    pub fn appsec_fargate_percentage(mut self, value: f64) -> Self {
        self.appsec_fargate_percentage = Some(value);
        self
    }

    pub fn appsec_fargate_usage(mut self, value: f64) -> Self {
        self.appsec_fargate_usage = Some(value);
        self
    }

    pub fn appsec_percentage(mut self, value: f64) -> Self {
        self.appsec_percentage = Some(value);
        self
    }

    pub fn appsec_usage(mut self, value: f64) -> Self {
        self.appsec_usage = Some(value);
        self
    }

    pub fn asm_serverless_traced_invocations_percentage(mut self, value: f64) -> Self {
        self.asm_serverless_traced_invocations_percentage = Some(value);
        self
    }

    pub fn asm_serverless_traced_invocations_usage(mut self, value: f64) -> Self {
        self.asm_serverless_traced_invocations_usage = Some(value);
        self
    }

    pub fn browser_percentage(mut self, value: f64) -> Self {
        self.browser_percentage = Some(value);
        self
    }

    pub fn browser_usage(mut self, value: f64) -> Self {
        self.browser_usage = Some(value);
        self
    }

    pub fn ci_pipeline_indexed_spans_percentage(mut self, value: f64) -> Self {
        self.ci_pipeline_indexed_spans_percentage = Some(value);
        self
    }

    pub fn ci_pipeline_indexed_spans_usage(mut self, value: f64) -> Self {
        self.ci_pipeline_indexed_spans_usage = Some(value);
        self
    }

    pub fn ci_test_indexed_spans_percentage(mut self, value: f64) -> Self {
        self.ci_test_indexed_spans_percentage = Some(value);
        self
    }

    pub fn ci_test_indexed_spans_usage(mut self, value: f64) -> Self {
        self.ci_test_indexed_spans_usage = Some(value);
        self
    }

    pub fn ci_visibility_itr_percentage(mut self, value: f64) -> Self {
        self.ci_visibility_itr_percentage = Some(value);
        self
    }

    pub fn ci_visibility_itr_usage(mut self, value: f64) -> Self {
        self.ci_visibility_itr_usage = Some(value);
        self
    }

    pub fn cloud_siem_percentage(mut self, value: f64) -> Self {
        self.cloud_siem_percentage = Some(value);
        self
    }

    pub fn cloud_siem_usage(mut self, value: f64) -> Self {
        self.cloud_siem_usage = Some(value);
        self
    }

    pub fn container_excl_agent_percentage(mut self, value: f64) -> Self {
        self.container_excl_agent_percentage = Some(value);
        self
    }

    pub fn container_excl_agent_usage(mut self, value: f64) -> Self {
        self.container_excl_agent_usage = Some(value);
        self
    }

    pub fn container_percentage(mut self, value: f64) -> Self {
        self.container_percentage = Some(value);
        self
    }

    pub fn container_usage(mut self, value: f64) -> Self {
        self.container_usage = Some(value);
        self
    }

    pub fn cspm_containers_percentage(mut self, value: f64) -> Self {
        self.cspm_containers_percentage = Some(value);
        self
    }

    pub fn cspm_containers_usage(mut self, value: f64) -> Self {
        self.cspm_containers_usage = Some(value);
        self
    }

    pub fn cspm_hosts_percentage(mut self, value: f64) -> Self {
        self.cspm_hosts_percentage = Some(value);
        self
    }

    pub fn cspm_hosts_usage(mut self, value: f64) -> Self {
        self.cspm_hosts_usage = Some(value);
        self
    }

    pub fn custom_event_percentage(mut self, value: f64) -> Self {
        self.custom_event_percentage = Some(value);
        self
    }

    pub fn custom_event_usage(mut self, value: f64) -> Self {
        self.custom_event_usage = Some(value);
        self
    }

    pub fn custom_ingested_timeseries_percentage(mut self, value: f64) -> Self {
        self.custom_ingested_timeseries_percentage = Some(value);
        self
    }

    pub fn custom_ingested_timeseries_usage(mut self, value: f64) -> Self {
        self.custom_ingested_timeseries_usage = Some(value);
        self
    }

    pub fn custom_timeseries_percentage(mut self, value: f64) -> Self {
        self.custom_timeseries_percentage = Some(value);
        self
    }

    pub fn custom_timeseries_usage(mut self, value: f64) -> Self {
        self.custom_timeseries_usage = Some(value);
        self
    }

    pub fn cws_containers_percentage(mut self, value: f64) -> Self {
        self.cws_containers_percentage = Some(value);
        self
    }

    pub fn cws_containers_usage(mut self, value: f64) -> Self {
        self.cws_containers_usage = Some(value);
        self
    }

    pub fn cws_hosts_percentage(mut self, value: f64) -> Self {
        self.cws_hosts_percentage = Some(value);
        self
    }

    pub fn cws_hosts_usage(mut self, value: f64) -> Self {
        self.cws_hosts_usage = Some(value);
        self
    }

    pub fn dbm_hosts_percentage(mut self, value: f64) -> Self {
        self.dbm_hosts_percentage = Some(value);
        self
    }

    pub fn dbm_hosts_usage(mut self, value: f64) -> Self {
        self.dbm_hosts_usage = Some(value);
        self
    }

    pub fn dbm_queries_percentage(mut self, value: f64) -> Self {
        self.dbm_queries_percentage = Some(value);
        self
    }

    pub fn dbm_queries_usage(mut self, value: f64) -> Self {
        self.dbm_queries_usage = Some(value);
        self
    }

    pub fn error_tracking_percentage(mut self, value: f64) -> Self {
        self.error_tracking_percentage = Some(value);
        self
    }

    pub fn error_tracking_usage(mut self, value: f64) -> Self {
        self.error_tracking_usage = Some(value);
        self
    }

    pub fn estimated_indexed_logs_percentage(mut self, value: f64) -> Self {
        self.estimated_indexed_logs_percentage = Some(value);
        self
    }

    pub fn estimated_indexed_logs_usage(mut self, value: f64) -> Self {
        self.estimated_indexed_logs_usage = Some(value);
        self
    }

    pub fn estimated_indexed_spans_percentage(mut self, value: f64) -> Self {
        self.estimated_indexed_spans_percentage = Some(value);
        self
    }

    pub fn estimated_indexed_spans_usage(mut self, value: f64) -> Self {
        self.estimated_indexed_spans_usage = Some(value);
        self
    }

    pub fn estimated_ingested_logs_percentage(mut self, value: f64) -> Self {
        self.estimated_ingested_logs_percentage = Some(value);
        self
    }

    pub fn estimated_ingested_logs_usage(mut self, value: f64) -> Self {
        self.estimated_ingested_logs_usage = Some(value);
        self
    }

    pub fn estimated_ingested_spans_percentage(mut self, value: f64) -> Self {
        self.estimated_ingested_spans_percentage = Some(value);
        self
    }

    pub fn estimated_ingested_spans_usage(mut self, value: f64) -> Self {
        self.estimated_ingested_spans_usage = Some(value);
        self
    }

    pub fn estimated_rum_sessions_percentage(mut self, value: f64) -> Self {
        self.estimated_rum_sessions_percentage = Some(value);
        self
    }

    pub fn estimated_rum_sessions_usage(mut self, value: f64) -> Self {
        self.estimated_rum_sessions_usage = Some(value);
        self
    }

    pub fn fargate_percentage(mut self, value: f64) -> Self {
        self.fargate_percentage = Some(value);
        self
    }

    pub fn fargate_usage(mut self, value: f64) -> Self {
        self.fargate_usage = Some(value);
        self
    }

    pub fn functions_percentage(mut self, value: f64) -> Self {
        self.functions_percentage = Some(value);
        self
    }

    pub fn functions_usage(mut self, value: f64) -> Self {
        self.functions_usage = Some(value);
        self
    }

    pub fn incident_management_monthly_active_users_percentage(mut self, value: f64) -> Self {
        self.incident_management_monthly_active_users_percentage = Some(value);
        self
    }

    pub fn incident_management_monthly_active_users_usage(mut self, value: f64) -> Self {
        self.incident_management_monthly_active_users_usage = Some(value);
        self
    }

    pub fn indexed_spans_percentage(mut self, value: f64) -> Self {
        self.indexed_spans_percentage = Some(value);
        self
    }

    pub fn indexed_spans_usage(mut self, value: f64) -> Self {
        self.indexed_spans_usage = Some(value);
        self
    }

    pub fn infra_host_percentage(mut self, value: f64) -> Self {
        self.infra_host_percentage = Some(value);
        self
    }

    pub fn infra_host_usage(mut self, value: f64) -> Self {
        self.infra_host_usage = Some(value);
        self
    }

    pub fn ingested_logs_bytes_percentage(mut self, value: f64) -> Self {
        self.ingested_logs_bytes_percentage = Some(value);
        self
    }

    pub fn ingested_logs_bytes_usage(mut self, value: f64) -> Self {
        self.ingested_logs_bytes_usage = Some(value);
        self
    }

    pub fn ingested_spans_bytes_percentage(mut self, value: f64) -> Self {
        self.ingested_spans_bytes_percentage = Some(value);
        self
    }

    pub fn ingested_spans_bytes_usage(mut self, value: f64) -> Self {
        self.ingested_spans_bytes_usage = Some(value);
        self
    }

    pub fn invocations_percentage(mut self, value: f64) -> Self {
        self.invocations_percentage = Some(value);
        self
    }

    pub fn invocations_usage(mut self, value: f64) -> Self {
        self.invocations_usage = Some(value);
        self
    }

    pub fn lambda_traced_invocations_percentage(mut self, value: f64) -> Self {
        self.lambda_traced_invocations_percentage = Some(value);
        self
    }

    pub fn lambda_traced_invocations_usage(mut self, value: f64) -> Self {
        self.lambda_traced_invocations_usage = Some(value);
        self
    }

    pub fn logs_indexed_15day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_15day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_15day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_15day_usage = Some(value);
        self
    }

    pub fn logs_indexed_180day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_180day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_180day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_180day_usage = Some(value);
        self
    }

    pub fn logs_indexed_1day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_1day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_1day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_1day_usage = Some(value);
        self
    }

    pub fn logs_indexed_30day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_30day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_30day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_30day_usage = Some(value);
        self
    }

    pub fn logs_indexed_360day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_360day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_360day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_360day_usage = Some(value);
        self
    }

    pub fn logs_indexed_3day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_3day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_3day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_3day_usage = Some(value);
        self
    }

    pub fn logs_indexed_45day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_45day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_45day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_45day_usage = Some(value);
        self
    }

    pub fn logs_indexed_60day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_60day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_60day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_60day_usage = Some(value);
        self
    }

    pub fn logs_indexed_7day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_7day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_7day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_7day_usage = Some(value);
        self
    }

    pub fn logs_indexed_90day_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_90day_percentage = Some(value);
        self
    }

    pub fn logs_indexed_90day_usage(mut self, value: f64) -> Self {
        self.logs_indexed_90day_usage = Some(value);
        self
    }

    pub fn logs_indexed_custom_retention_percentage(mut self, value: f64) -> Self {
        self.logs_indexed_custom_retention_percentage = Some(value);
        self
    }

    pub fn logs_indexed_custom_retention_usage(mut self, value: f64) -> Self {
        self.logs_indexed_custom_retention_usage = Some(value);
        self
    }

    pub fn mobile_app_testing_percentage(mut self, value: f64) -> Self {
        self.mobile_app_testing_percentage = Some(value);
        self
    }

    pub fn mobile_app_testing_usage(mut self, value: f64) -> Self {
        self.mobile_app_testing_usage = Some(value);
        self
    }

    pub fn ndm_netflow_percentage(mut self, value: f64) -> Self {
        self.ndm_netflow_percentage = Some(value);
        self
    }

    pub fn ndm_netflow_usage(mut self, value: f64) -> Self {
        self.ndm_netflow_usage = Some(value);
        self
    }

    pub fn npm_host_percentage(mut self, value: f64) -> Self {
        self.npm_host_percentage = Some(value);
        self
    }

    pub fn npm_host_usage(mut self, value: f64) -> Self {
        self.npm_host_usage = Some(value);
        self
    }

    pub fn obs_pipeline_bytes_percentage(mut self, value: f64) -> Self {
        self.obs_pipeline_bytes_percentage = Some(value);
        self
    }

    pub fn obs_pipeline_bytes_usage(mut self, value: f64) -> Self {
        self.obs_pipeline_bytes_usage = Some(value);
        self
    }

    pub fn obs_pipelines_vcpu_percentage(mut self, value: f64) -> Self {
        self.obs_pipelines_vcpu_percentage = Some(value);
        self
    }

    pub fn obs_pipelines_vcpu_usage(mut self, value: f64) -> Self {
        self.obs_pipelines_vcpu_usage = Some(value);
        self
    }

    pub fn online_archive_percentage(mut self, value: f64) -> Self {
        self.online_archive_percentage = Some(value);
        self
    }

    pub fn online_archive_usage(mut self, value: f64) -> Self {
        self.online_archive_usage = Some(value);
        self
    }

    pub fn profiled_container_percentage(mut self, value: f64) -> Self {
        self.profiled_container_percentage = Some(value);
        self
    }

    pub fn profiled_container_usage(mut self, value: f64) -> Self {
        self.profiled_container_usage = Some(value);
        self
    }

    pub fn profiled_fargate_percentage(mut self, value: f64) -> Self {
        self.profiled_fargate_percentage = Some(value);
        self
    }

    pub fn profiled_fargate_usage(mut self, value: f64) -> Self {
        self.profiled_fargate_usage = Some(value);
        self
    }

    pub fn profiled_host_percentage(mut self, value: f64) -> Self {
        self.profiled_host_percentage = Some(value);
        self
    }

    pub fn profiled_host_usage(mut self, value: f64) -> Self {
        self.profiled_host_usage = Some(value);
        self
    }

    pub fn rum_browser_mobile_sessions_percentage(mut self, value: f64) -> Self {
        self.rum_browser_mobile_sessions_percentage = Some(value);
        self
    }

    pub fn rum_browser_mobile_sessions_usage(mut self, value: f64) -> Self {
        self.rum_browser_mobile_sessions_usage = Some(value);
        self
    }

    pub fn rum_replay_sessions_percentage(mut self, value: f64) -> Self {
        self.rum_replay_sessions_percentage = Some(value);
        self
    }

    pub fn rum_replay_sessions_usage(mut self, value: f64) -> Self {
        self.rum_replay_sessions_usage = Some(value);
        self
    }

    pub fn sca_fargate_percentage(mut self, value: f64) -> Self {
        self.sca_fargate_percentage = Some(value);
        self
    }

    pub fn sca_fargate_usage(mut self, value: f64) -> Self {
        self.sca_fargate_usage = Some(value);
        self
    }

    pub fn sds_scanned_bytes_percentage(mut self, value: f64) -> Self {
        self.sds_scanned_bytes_percentage = Some(value);
        self
    }

    pub fn sds_scanned_bytes_usage(mut self, value: f64) -> Self {
        self.sds_scanned_bytes_usage = Some(value);
        self
    }

    pub fn serverless_apps_percentage(mut self, value: f64) -> Self {
        self.serverless_apps_percentage = Some(value);
        self
    }

    pub fn serverless_apps_usage(mut self, value: f64) -> Self {
        self.serverless_apps_usage = Some(value);
        self
    }

    pub fn siem_analyzed_logs_add_on_percentage(mut self, value: f64) -> Self {
        self.siem_analyzed_logs_add_on_percentage = Some(value);
        self
    }

    pub fn siem_analyzed_logs_add_on_usage(mut self, value: f64) -> Self {
        self.siem_analyzed_logs_add_on_usage = Some(value);
        self
    }

    pub fn siem_ingested_bytes_percentage(mut self, value: f64) -> Self {
        self.siem_ingested_bytes_percentage = Some(value);
        self
    }

    pub fn siem_ingested_bytes_usage(mut self, value: f64) -> Self {
        self.siem_ingested_bytes_usage = Some(value);
        self
    }

    pub fn snmp_percentage(mut self, value: f64) -> Self {
        self.snmp_percentage = Some(value);
        self
    }

    pub fn snmp_usage(mut self, value: f64) -> Self {
        self.snmp_usage = Some(value);
        self
    }

    pub fn universal_service_monitoring_percentage(mut self, value: f64) -> Self {
        self.universal_service_monitoring_percentage = Some(value);
        self
    }

    pub fn universal_service_monitoring_usage(mut self, value: f64) -> Self {
        self.universal_service_monitoring_usage = Some(value);
        self
    }

    pub fn vuln_management_hosts_percentage(mut self, value: f64) -> Self {
        self.vuln_management_hosts_percentage = Some(value);
        self
    }

    pub fn vuln_management_hosts_usage(mut self, value: f64) -> Self {
        self.vuln_management_hosts_usage = Some(value);
        self
    }

    pub fn workflow_executions_percentage(mut self, value: f64) -> Self {
        self.workflow_executions_percentage = Some(value);
        self
    }

    pub fn workflow_executions_usage(mut self, value: f64) -> Self {
        self.workflow_executions_usage = Some(value);
        self
    }
}

impl Default for MonthlyUsageAttributionValues {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonthlyUsageAttributionValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonthlyUsageAttributionValuesVisitor;
        impl<'a> Visitor<'a> for MonthlyUsageAttributionValuesVisitor {
            type Value = MonthlyUsageAttributionValues;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_percentage: Option<f64> = None;
                let mut api_usage: Option<f64> = None;
                let mut apm_fargate_percentage: Option<f64> = None;
                let mut apm_fargate_usage: Option<f64> = None;
                let mut apm_host_percentage: Option<f64> = None;
                let mut apm_host_usage: Option<f64> = None;
                let mut apm_usm_percentage: Option<f64> = None;
                let mut apm_usm_usage: Option<f64> = None;
                let mut appsec_fargate_percentage: Option<f64> = None;
                let mut appsec_fargate_usage: Option<f64> = None;
                let mut appsec_percentage: Option<f64> = None;
                let mut appsec_usage: Option<f64> = None;
                let mut asm_serverless_traced_invocations_percentage: Option<f64> = None;
                let mut asm_serverless_traced_invocations_usage: Option<f64> = None;
                let mut browser_percentage: Option<f64> = None;
                let mut browser_usage: Option<f64> = None;
                let mut ci_pipeline_indexed_spans_percentage: Option<f64> = None;
                let mut ci_pipeline_indexed_spans_usage: Option<f64> = None;
                let mut ci_test_indexed_spans_percentage: Option<f64> = None;
                let mut ci_test_indexed_spans_usage: Option<f64> = None;
                let mut ci_visibility_itr_percentage: Option<f64> = None;
                let mut ci_visibility_itr_usage: Option<f64> = None;
                let mut cloud_siem_percentage: Option<f64> = None;
                let mut cloud_siem_usage: Option<f64> = None;
                let mut container_excl_agent_percentage: Option<f64> = None;
                let mut container_excl_agent_usage: Option<f64> = None;
                let mut container_percentage: Option<f64> = None;
                let mut container_usage: Option<f64> = None;
                let mut cspm_containers_percentage: Option<f64> = None;
                let mut cspm_containers_usage: Option<f64> = None;
                let mut cspm_hosts_percentage: Option<f64> = None;
                let mut cspm_hosts_usage: Option<f64> = None;
                let mut custom_event_percentage: Option<f64> = None;
                let mut custom_event_usage: Option<f64> = None;
                let mut custom_ingested_timeseries_percentage: Option<f64> = None;
                let mut custom_ingested_timeseries_usage: Option<f64> = None;
                let mut custom_timeseries_percentage: Option<f64> = None;
                let mut custom_timeseries_usage: Option<f64> = None;
                let mut cws_containers_percentage: Option<f64> = None;
                let mut cws_containers_usage: Option<f64> = None;
                let mut cws_hosts_percentage: Option<f64> = None;
                let mut cws_hosts_usage: Option<f64> = None;
                let mut dbm_hosts_percentage: Option<f64> = None;
                let mut dbm_hosts_usage: Option<f64> = None;
                let mut dbm_queries_percentage: Option<f64> = None;
                let mut dbm_queries_usage: Option<f64> = None;
                let mut error_tracking_percentage: Option<f64> = None;
                let mut error_tracking_usage: Option<f64> = None;
                let mut estimated_indexed_logs_percentage: Option<f64> = None;
                let mut estimated_indexed_logs_usage: Option<f64> = None;
                let mut estimated_indexed_spans_percentage: Option<f64> = None;
                let mut estimated_indexed_spans_usage: Option<f64> = None;
                let mut estimated_ingested_logs_percentage: Option<f64> = None;
                let mut estimated_ingested_logs_usage: Option<f64> = None;
                let mut estimated_ingested_spans_percentage: Option<f64> = None;
                let mut estimated_ingested_spans_usage: Option<f64> = None;
                let mut estimated_rum_sessions_percentage: Option<f64> = None;
                let mut estimated_rum_sessions_usage: Option<f64> = None;
                let mut fargate_percentage: Option<f64> = None;
                let mut fargate_usage: Option<f64> = None;
                let mut functions_percentage: Option<f64> = None;
                let mut functions_usage: Option<f64> = None;
                let mut incident_management_monthly_active_users_percentage: Option<f64> = None;
                let mut incident_management_monthly_active_users_usage: Option<f64> = None;
                let mut indexed_spans_percentage: Option<f64> = None;
                let mut indexed_spans_usage: Option<f64> = None;
                let mut infra_host_percentage: Option<f64> = None;
                let mut infra_host_usage: Option<f64> = None;
                let mut ingested_logs_bytes_percentage: Option<f64> = None;
                let mut ingested_logs_bytes_usage: Option<f64> = None;
                let mut ingested_spans_bytes_percentage: Option<f64> = None;
                let mut ingested_spans_bytes_usage: Option<f64> = None;
                let mut invocations_percentage: Option<f64> = None;
                let mut invocations_usage: Option<f64> = None;
                let mut lambda_traced_invocations_percentage: Option<f64> = None;
                let mut lambda_traced_invocations_usage: Option<f64> = None;
                let mut logs_indexed_15day_percentage: Option<f64> = None;
                let mut logs_indexed_15day_usage: Option<f64> = None;
                let mut logs_indexed_180day_percentage: Option<f64> = None;
                let mut logs_indexed_180day_usage: Option<f64> = None;
                let mut logs_indexed_1day_percentage: Option<f64> = None;
                let mut logs_indexed_1day_usage: Option<f64> = None;
                let mut logs_indexed_30day_percentage: Option<f64> = None;
                let mut logs_indexed_30day_usage: Option<f64> = None;
                let mut logs_indexed_360day_percentage: Option<f64> = None;
                let mut logs_indexed_360day_usage: Option<f64> = None;
                let mut logs_indexed_3day_percentage: Option<f64> = None;
                let mut logs_indexed_3day_usage: Option<f64> = None;
                let mut logs_indexed_45day_percentage: Option<f64> = None;
                let mut logs_indexed_45day_usage: Option<f64> = None;
                let mut logs_indexed_60day_percentage: Option<f64> = None;
                let mut logs_indexed_60day_usage: Option<f64> = None;
                let mut logs_indexed_7day_percentage: Option<f64> = None;
                let mut logs_indexed_7day_usage: Option<f64> = None;
                let mut logs_indexed_90day_percentage: Option<f64> = None;
                let mut logs_indexed_90day_usage: Option<f64> = None;
                let mut logs_indexed_custom_retention_percentage: Option<f64> = None;
                let mut logs_indexed_custom_retention_usage: Option<f64> = None;
                let mut mobile_app_testing_percentage: Option<f64> = None;
                let mut mobile_app_testing_usage: Option<f64> = None;
                let mut ndm_netflow_percentage: Option<f64> = None;
                let mut ndm_netflow_usage: Option<f64> = None;
                let mut npm_host_percentage: Option<f64> = None;
                let mut npm_host_usage: Option<f64> = None;
                let mut obs_pipeline_bytes_percentage: Option<f64> = None;
                let mut obs_pipeline_bytes_usage: Option<f64> = None;
                let mut obs_pipelines_vcpu_percentage: Option<f64> = None;
                let mut obs_pipelines_vcpu_usage: Option<f64> = None;
                let mut online_archive_percentage: Option<f64> = None;
                let mut online_archive_usage: Option<f64> = None;
                let mut profiled_container_percentage: Option<f64> = None;
                let mut profiled_container_usage: Option<f64> = None;
                let mut profiled_fargate_percentage: Option<f64> = None;
                let mut profiled_fargate_usage: Option<f64> = None;
                let mut profiled_host_percentage: Option<f64> = None;
                let mut profiled_host_usage: Option<f64> = None;
                let mut rum_browser_mobile_sessions_percentage: Option<f64> = None;
                let mut rum_browser_mobile_sessions_usage: Option<f64> = None;
                let mut rum_replay_sessions_percentage: Option<f64> = None;
                let mut rum_replay_sessions_usage: Option<f64> = None;
                let mut sca_fargate_percentage: Option<f64> = None;
                let mut sca_fargate_usage: Option<f64> = None;
                let mut sds_scanned_bytes_percentage: Option<f64> = None;
                let mut sds_scanned_bytes_usage: Option<f64> = None;
                let mut serverless_apps_percentage: Option<f64> = None;
                let mut serverless_apps_usage: Option<f64> = None;
                let mut siem_analyzed_logs_add_on_percentage: Option<f64> = None;
                let mut siem_analyzed_logs_add_on_usage: Option<f64> = None;
                let mut siem_ingested_bytes_percentage: Option<f64> = None;
                let mut siem_ingested_bytes_usage: Option<f64> = None;
                let mut snmp_percentage: Option<f64> = None;
                let mut snmp_usage: Option<f64> = None;
                let mut universal_service_monitoring_percentage: Option<f64> = None;
                let mut universal_service_monitoring_usage: Option<f64> = None;
                let mut vuln_management_hosts_percentage: Option<f64> = None;
                let mut vuln_management_hosts_usage: Option<f64> = None;
                let mut workflow_executions_percentage: Option<f64> = None;
                let mut workflow_executions_usage: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            api_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            api_usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_fargate_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_fargate_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_usm_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_usm_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_usm_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_usm_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_fargate_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_fargate_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_fargate_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_fargate_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            appsec_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "asm_serverless_traced_invocations_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            asm_serverless_traced_invocations_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "asm_serverless_traced_invocations_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            asm_serverless_traced_invocations_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_indexed_spans_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_indexed_spans_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_itr_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_itr_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_itr_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_visibility_itr_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_siem_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_siem_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_siem_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_siem_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_excl_agent_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            container_excl_agent_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_excl_agent_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            container_excl_agent_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            container_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            container_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_containers_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_containers_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_containers_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_containers_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_hosts_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_hosts_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_hosts_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_hosts_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_event_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_event_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_event_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_event_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_ingested_timeseries_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_ingested_timeseries_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_ingested_timeseries_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_ingested_timeseries_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_timeseries_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_timeseries_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_timeseries_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_timeseries_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_containers_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_containers_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_containers_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_containers_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_hosts_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_hosts_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_hosts_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_hosts_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_hosts_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_hosts_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_hosts_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_hosts_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_queries_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_queries_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_queries_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_queries_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_indexed_logs_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_indexed_logs_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_indexed_logs_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_indexed_logs_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_indexed_spans_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_indexed_spans_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_indexed_spans_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_indexed_spans_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_ingested_logs_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_ingested_logs_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_ingested_logs_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_ingested_logs_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_ingested_spans_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_ingested_spans_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_ingested_spans_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_ingested_spans_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_rum_sessions_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_rum_sessions_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_rum_sessions_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_rum_sessions_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "functions_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            functions_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "functions_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            functions_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_monthly_active_users_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_monthly_active_users_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_monthly_active_users_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_monthly_active_users_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_spans_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_spans_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_spans_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_spans_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_logs_bytes_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_logs_bytes_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_logs_bytes_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_logs_bytes_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_spans_bytes_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_spans_bytes_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_spans_bytes_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_spans_bytes_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invocations_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            invocations_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invocations_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            invocations_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambda_traced_invocations_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            lambda_traced_invocations_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambda_traced_invocations_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            lambda_traced_invocations_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_15day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_15day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_15day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_15day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_180day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_180day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_180day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_180day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_1day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_1day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_1day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_1day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_30day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_30day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_30day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_30day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_360day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_360day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_360day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_360day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_3day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_3day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_3day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_3day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_45day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_45day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_45day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_45day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_60day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_60day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_60day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_60day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_7day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_7day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_7day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_7day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_90day_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_90day_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_90day_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_90day_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_custom_retention_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_custom_retention_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_custom_retention_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_custom_retention_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_app_testing_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_app_testing_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_app_testing_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_app_testing_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ndm_netflow_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            ndm_netflow_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ndm_netflow_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ndm_netflow_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "obs_pipeline_bytes_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            obs_pipeline_bytes_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "obs_pipeline_bytes_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            obs_pipeline_bytes_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "obs_pipelines_vcpu_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            obs_pipelines_vcpu_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "obs_pipelines_vcpu_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            obs_pipelines_vcpu_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_container_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_container_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_container_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_container_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_fargate_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_fargate_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_fargate_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_fargate_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_host_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_host_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiled_host_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            profiled_host_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_mobile_sessions_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_mobile_sessions_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_browser_mobile_sessions_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_browser_mobile_sessions_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_replay_sessions_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_replay_sessions_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_replay_sessions_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_replay_sessions_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sca_fargate_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sca_fargate_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            sca_fargate_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_scanned_bytes_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_scanned_bytes_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_scanned_bytes_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_scanned_bytes_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apps_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apps_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_analyzed_logs_add_on_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_analyzed_logs_add_on_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_analyzed_logs_add_on_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_analyzed_logs_add_on_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_ingested_bytes_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_ingested_bytes_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_ingested_bytes_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_ingested_bytes_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snmp_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            snmp_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snmp_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            snmp_usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "universal_service_monitoring_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            universal_service_monitoring_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "universal_service_monitoring_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            universal_service_monitoring_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_management_hosts_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_management_hosts_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_management_hosts_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_management_hosts_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow_executions_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_executions_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow_executions_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_executions_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonthlyUsageAttributionValues {
                    api_percentage,
                    api_usage,
                    apm_fargate_percentage,
                    apm_fargate_usage,
                    apm_host_percentage,
                    apm_host_usage,
                    apm_usm_percentage,
                    apm_usm_usage,
                    appsec_fargate_percentage,
                    appsec_fargate_usage,
                    appsec_percentage,
                    appsec_usage,
                    asm_serverless_traced_invocations_percentage,
                    asm_serverless_traced_invocations_usage,
                    browser_percentage,
                    browser_usage,
                    ci_pipeline_indexed_spans_percentage,
                    ci_pipeline_indexed_spans_usage,
                    ci_test_indexed_spans_percentage,
                    ci_test_indexed_spans_usage,
                    ci_visibility_itr_percentage,
                    ci_visibility_itr_usage,
                    cloud_siem_percentage,
                    cloud_siem_usage,
                    container_excl_agent_percentage,
                    container_excl_agent_usage,
                    container_percentage,
                    container_usage,
                    cspm_containers_percentage,
                    cspm_containers_usage,
                    cspm_hosts_percentage,
                    cspm_hosts_usage,
                    custom_event_percentage,
                    custom_event_usage,
                    custom_ingested_timeseries_percentage,
                    custom_ingested_timeseries_usage,
                    custom_timeseries_percentage,
                    custom_timeseries_usage,
                    cws_containers_percentage,
                    cws_containers_usage,
                    cws_hosts_percentage,
                    cws_hosts_usage,
                    dbm_hosts_percentage,
                    dbm_hosts_usage,
                    dbm_queries_percentage,
                    dbm_queries_usage,
                    error_tracking_percentage,
                    error_tracking_usage,
                    estimated_indexed_logs_percentage,
                    estimated_indexed_logs_usage,
                    estimated_indexed_spans_percentage,
                    estimated_indexed_spans_usage,
                    estimated_ingested_logs_percentage,
                    estimated_ingested_logs_usage,
                    estimated_ingested_spans_percentage,
                    estimated_ingested_spans_usage,
                    estimated_rum_sessions_percentage,
                    estimated_rum_sessions_usage,
                    fargate_percentage,
                    fargate_usage,
                    functions_percentage,
                    functions_usage,
                    incident_management_monthly_active_users_percentage,
                    incident_management_monthly_active_users_usage,
                    indexed_spans_percentage,
                    indexed_spans_usage,
                    infra_host_percentage,
                    infra_host_usage,
                    ingested_logs_bytes_percentage,
                    ingested_logs_bytes_usage,
                    ingested_spans_bytes_percentage,
                    ingested_spans_bytes_usage,
                    invocations_percentage,
                    invocations_usage,
                    lambda_traced_invocations_percentage,
                    lambda_traced_invocations_usage,
                    logs_indexed_15day_percentage,
                    logs_indexed_15day_usage,
                    logs_indexed_180day_percentage,
                    logs_indexed_180day_usage,
                    logs_indexed_1day_percentage,
                    logs_indexed_1day_usage,
                    logs_indexed_30day_percentage,
                    logs_indexed_30day_usage,
                    logs_indexed_360day_percentage,
                    logs_indexed_360day_usage,
                    logs_indexed_3day_percentage,
                    logs_indexed_3day_usage,
                    logs_indexed_45day_percentage,
                    logs_indexed_45day_usage,
                    logs_indexed_60day_percentage,
                    logs_indexed_60day_usage,
                    logs_indexed_7day_percentage,
                    logs_indexed_7day_usage,
                    logs_indexed_90day_percentage,
                    logs_indexed_90day_usage,
                    logs_indexed_custom_retention_percentage,
                    logs_indexed_custom_retention_usage,
                    mobile_app_testing_percentage,
                    mobile_app_testing_usage,
                    ndm_netflow_percentage,
                    ndm_netflow_usage,
                    npm_host_percentage,
                    npm_host_usage,
                    obs_pipeline_bytes_percentage,
                    obs_pipeline_bytes_usage,
                    obs_pipelines_vcpu_percentage,
                    obs_pipelines_vcpu_usage,
                    online_archive_percentage,
                    online_archive_usage,
                    profiled_container_percentage,
                    profiled_container_usage,
                    profiled_fargate_percentage,
                    profiled_fargate_usage,
                    profiled_host_percentage,
                    profiled_host_usage,
                    rum_browser_mobile_sessions_percentage,
                    rum_browser_mobile_sessions_usage,
                    rum_replay_sessions_percentage,
                    rum_replay_sessions_usage,
                    sca_fargate_percentage,
                    sca_fargate_usage,
                    sds_scanned_bytes_percentage,
                    sds_scanned_bytes_usage,
                    serverless_apps_percentage,
                    serverless_apps_usage,
                    siem_analyzed_logs_add_on_percentage,
                    siem_analyzed_logs_add_on_usage,
                    siem_ingested_bytes_percentage,
                    siem_ingested_bytes_usage,
                    snmp_percentage,
                    snmp_usage,
                    universal_service_monitoring_percentage,
                    universal_service_monitoring_usage,
                    vuln_management_hosts_percentage,
                    vuln_management_hosts_usage,
                    workflow_executions_percentage,
                    workflow_executions_usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonthlyUsageAttributionValuesVisitor)
    }
}
