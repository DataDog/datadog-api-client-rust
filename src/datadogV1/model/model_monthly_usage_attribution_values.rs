// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Fields in Usage Summary by tag(s).
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// The percentage of RUM Replay Sessions usage by tag(s).
    #[serde(rename = "rum_replay_sessions_percentage")]
    pub rum_replay_sessions_percentage: Option<f64>,
    /// The total RUM Replay Sessions usage by tag(s).
    #[serde(rename = "rum_replay_sessions_usage")]
    pub rum_replay_sessions_usage: Option<f64>,
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
            sds_scanned_bytes_percentage: None,
            sds_scanned_bytes_usage: None,
            serverless_apps_percentage: None,
            serverless_apps_usage: None,
            siem_ingested_bytes_percentage: None,
            siem_ingested_bytes_usage: None,
            snmp_percentage: None,
            snmp_usage: None,
            universal_service_monitoring_percentage: None,
            universal_service_monitoring_usage: None,
            vuln_management_hosts_percentage: None,
            vuln_management_hosts_usage: None,
        }
    }
}
