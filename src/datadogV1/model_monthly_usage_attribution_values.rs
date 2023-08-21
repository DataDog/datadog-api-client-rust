// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyUsageAttributionValues {
    /// The percentage of synthetic API test usage by tag(s).
    #[serde(rename = "api_percentage", skip_serializing_if = "Option::is_none")]
    pub api_percentage: f64,
    /// The synthetic API test usage by tag(s).
    #[serde(rename = "api_usage", skip_serializing_if = "Option::is_none")]
    pub api_usage: f64,
    /// The percentage of APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_percentage", skip_serializing_if = "Option::is_none")]
    pub apm_fargate_percentage: f64,
    /// The APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_usage", skip_serializing_if = "Option::is_none")]
    pub apm_fargate_usage: f64,
    /// The percentage of APM host usage by tag(s).
    #[serde(rename = "apm_host_percentage", skip_serializing_if = "Option::is_none")]
    pub apm_host_percentage: f64,
    /// The APM host usage by tag(s).
    #[serde(rename = "apm_host_usage", skip_serializing_if = "Option::is_none")]
    pub apm_host_usage: f64,
    /// The percentage of Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_percentage", skip_serializing_if = "Option::is_none")]
    pub appsec_fargate_percentage: f64,
    /// The Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_usage", skip_serializing_if = "Option::is_none")]
    pub appsec_fargate_usage: f64,
    /// The percentage of Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_percentage", skip_serializing_if = "Option::is_none")]
    pub appsec_percentage: f64,
    /// The Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_usage", skip_serializing_if = "Option::is_none")]
    pub appsec_usage: f64,
    /// The percentage of synthetic browser test usage by tag(s).
    #[serde(rename = "browser_percentage", skip_serializing_if = "Option::is_none")]
    pub browser_percentage: f64,
    /// The synthetic browser test usage by tag(s).
    #[serde(rename = "browser_usage", skip_serializing_if = "Option::is_none")]
    pub browser_usage: f64,
    /// The percentage of Git committers for Intelligent Test Runner usage by tag(s).
    #[serde(rename = "ci_visibility_itr_percentage", skip_serializing_if = "Option::is_none")]
    pub ci_visibility_itr_percentage: f64,
    /// The Git committers for Intelligent Test Runner usage by tag(s).
    #[serde(rename = "ci_visibility_itr_usage", skip_serializing_if = "Option::is_none")]
    pub ci_visibility_itr_usage: f64,
    /// The percentage of container usage without the Datadog Agent by tag(s).
    #[serde(rename = "container_excl_agent_percentage", skip_serializing_if = "Option::is_none")]
    pub container_excl_agent_percentage: f64,
    /// The container usage without the Datadog Agent by tag(s).
    #[serde(rename = "container_excl_agent_usage", skip_serializing_if = "Option::is_none")]
    pub container_excl_agent_usage: f64,
    /// The percentage of container usage by tag(s).
    #[serde(rename = "container_percentage", skip_serializing_if = "Option::is_none")]
    pub container_percentage: f64,
    /// The container usage by tag(s).
    #[serde(rename = "container_usage", skip_serializing_if = "Option::is_none")]
    pub container_usage: f64,
    /// The percentage of CSPM container usage by tag(s).
    #[serde(rename = "cspm_containers_percentage", skip_serializing_if = "Option::is_none")]
    pub cspm_containers_percentage: f64,
    /// The CSPM container usage by tag(s).
    #[serde(rename = "cspm_containers_usage", skip_serializing_if = "Option::is_none")]
    pub cspm_containers_usage: f64,
    /// The percentage of CSPM host usage by by tag(s).
    #[serde(rename = "cspm_hosts_percentage", skip_serializing_if = "Option::is_none")]
    pub cspm_hosts_percentage: f64,
    /// The CSPM host usage by tag(s).
    #[serde(rename = "cspm_hosts_usage", skip_serializing_if = "Option::is_none")]
    pub cspm_hosts_usage: f64,
    /// The percentage of ingested custom metrics usage by tag(s).
    #[serde(rename = "custom_ingested_timeseries_percentage", skip_serializing_if = "Option::is_none")]
    pub custom_ingested_timeseries_percentage: f64,
    /// The ingested custom metrics usage by tag(s).
    #[serde(rename = "custom_ingested_timeseries_usage", skip_serializing_if = "Option::is_none")]
    pub custom_ingested_timeseries_usage: f64,
    /// The percentage of indexed custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_percentage", skip_serializing_if = "Option::is_none")]
    pub custom_timeseries_percentage: f64,
    /// The indexed custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_usage", skip_serializing_if = "Option::is_none")]
    pub custom_timeseries_usage: f64,
    /// The percentage of Cloud Workload Security container usage by tag(s).
    #[serde(rename = "cws_containers_percentage", skip_serializing_if = "Option::is_none")]
    pub cws_containers_percentage: f64,
    /// The Cloud Workload Security container usage by tag(s).
    #[serde(rename = "cws_containers_usage", skip_serializing_if = "Option::is_none")]
    pub cws_containers_usage: f64,
    /// The percentage of Cloud Workload Security host usage by tag(s).
    #[serde(rename = "cws_hosts_percentage", skip_serializing_if = "Option::is_none")]
    pub cws_hosts_percentage: f64,
    /// The Cloud Workload Security host usage by tag(s).
    #[serde(rename = "cws_hosts_usage", skip_serializing_if = "Option::is_none")]
    pub cws_hosts_usage: f64,
    /// The percentage of Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_percentage", skip_serializing_if = "Option::is_none")]
    pub dbm_hosts_percentage: f64,
    /// The Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_usage", skip_serializing_if = "Option::is_none")]
    pub dbm_hosts_usage: f64,
    /// The percentage of Database Monitoring queries usage by tag(s).
    #[serde(rename = "dbm_queries_percentage", skip_serializing_if = "Option::is_none")]
    pub dbm_queries_percentage: f64,
    /// The Database Monitoring queries usage by tag(s).
    #[serde(rename = "dbm_queries_usage", skip_serializing_if = "Option::is_none")]
    pub dbm_queries_usage: f64,
    /// The percentage of estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_percentage", skip_serializing_if = "Option::is_none")]
    pub estimated_indexed_logs_percentage: f64,
    /// The estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_usage", skip_serializing_if = "Option::is_none")]
    pub estimated_indexed_logs_usage: f64,
    /// The percentage of estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_percentage", skip_serializing_if = "Option::is_none")]
    pub estimated_indexed_spans_percentage: f64,
    /// The estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_usage", skip_serializing_if = "Option::is_none")]
    pub estimated_indexed_spans_usage: f64,
    /// The percentage of estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_percentage", skip_serializing_if = "Option::is_none")]
    pub estimated_ingested_logs_percentage: f64,
    /// The estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_usage", skip_serializing_if = "Option::is_none")]
    pub estimated_ingested_logs_usage: f64,
    /// The percentage of estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_percentage", skip_serializing_if = "Option::is_none")]
    pub estimated_ingested_spans_percentage: f64,
    /// The estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_usage", skip_serializing_if = "Option::is_none")]
    pub estimated_ingested_spans_usage: f64,
    /// The percentage of estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_percentage", skip_serializing_if = "Option::is_none")]
    pub estimated_rum_sessions_percentage: f64,
    /// The estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_usage", skip_serializing_if = "Option::is_none")]
    pub estimated_rum_sessions_usage: f64,
    /// The percentage of Fargate usage by tags.
    #[serde(rename = "fargate_percentage", skip_serializing_if = "Option::is_none")]
    pub fargate_percentage: f64,
    /// The Fargate usage by tags.
    #[serde(rename = "fargate_usage", skip_serializing_if = "Option::is_none")]
    pub fargate_usage: f64,
    /// The percentage of Lambda function usage by tag(s).
    #[serde(rename = "functions_percentage", skip_serializing_if = "Option::is_none")]
    pub functions_percentage: f64,
    /// The Lambda function usage by tag(s).
    #[serde(rename = "functions_usage", skip_serializing_if = "Option::is_none")]
    pub functions_usage: f64,
    /// The percentage of infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_percentage", skip_serializing_if = "Option::is_none")]
    pub infra_host_percentage: f64,
    /// The infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_usage", skip_serializing_if = "Option::is_none")]
    pub infra_host_usage: f64,
    /// The percentage of Lambda invocation usage by tag(s).
    #[serde(rename = "invocations_percentage", skip_serializing_if = "Option::is_none")]
    pub invocations_percentage: f64,
    /// The Lambda invocation usage by tag(s).
    #[serde(rename = "invocations_usage", skip_serializing_if = "Option::is_none")]
    pub invocations_usage: f64,
    /// The percentage of network host usage by tag(s).
    #[serde(rename = "npm_host_percentage", skip_serializing_if = "Option::is_none")]
    pub npm_host_percentage: f64,
    /// The network host usage by tag(s).
    #[serde(rename = "npm_host_usage", skip_serializing_if = "Option::is_none")]
    pub npm_host_usage: f64,
    /// The percentage of observability pipeline bytes usage by tag(s).
    #[serde(rename = "obs_pipeline_bytes_percentage", skip_serializing_if = "Option::is_none")]
    pub obs_pipeline_bytes_percentage: f64,
    /// The observability pipeline bytes usage by tag(s).
    #[serde(rename = "obs_pipeline_bytes_usage", skip_serializing_if = "Option::is_none")]
    pub obs_pipeline_bytes_usage: f64,
    /// The percentage of profiled container usage by tag(s).
    #[serde(rename = "profiled_container_percentage", skip_serializing_if = "Option::is_none")]
    pub profiled_container_percentage: f64,
    /// The profiled container usage by tag(s).
    #[serde(rename = "profiled_container_usage", skip_serializing_if = "Option::is_none")]
    pub profiled_container_usage: f64,
    /// The percentage of profiled Fargate task usage by tag(s).
    #[serde(rename = "profiled_fargate_percentage", skip_serializing_if = "Option::is_none")]
    pub profiled_fargate_percentage: f64,
    /// The profiled Fargate task usage by tag(s).
    #[serde(rename = "profiled_fargate_usage", skip_serializing_if = "Option::is_none")]
    pub profiled_fargate_usage: f64,
    /// The percentage of profiled hosts usage by tag(s).
    #[serde(rename = "profiled_host_percentage", skip_serializing_if = "Option::is_none")]
    pub profiled_host_percentage: f64,
    /// The profiled hosts usage by tag(s).
    #[serde(rename = "profiled_host_usage", skip_serializing_if = "Option::is_none")]
    pub profiled_host_usage: f64,
    /// The percentage of network device usage by tag(s).
    #[serde(rename = "snmp_percentage", skip_serializing_if = "Option::is_none")]
    pub snmp_percentage: f64,
    /// The network device usage by tag(s).
    #[serde(rename = "snmp_usage", skip_serializing_if = "Option::is_none")]
    pub snmp_usage: f64,
    /// The percentage of universal service monitoring usage by tag(s).
    #[serde(rename = "universal_service_monitoring_percentage", skip_serializing_if = "Option::is_none")]
    pub universal_service_monitoring_percentage: f64,
    /// The universal service monitoring usage by tag(s).
    #[serde(rename = "universal_service_monitoring_usage", skip_serializing_if = "Option::is_none")]
    pub universal_service_monitoring_usage: f64,
    /// The percentage of Application Vulnerability Management usage by tag(s).
    #[serde(rename = "vuln_management_hosts_percentage", skip_serializing_if = "Option::is_none")]
    pub vuln_management_hosts_percentage: f64,
    /// The Application Vulnerability Management usage by tag(s).
    #[serde(rename = "vuln_management_hosts_usage", skip_serializing_if = "Option::is_none")]
    pub vuln_management_hosts_usage: f64,
}

