// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryKeys {
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_fargate_average", skip_serializing_if = "Option::is_none")]
    pub apm_fargate_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_fargate_sum", skip_serializing_if = "Option::is_none")]
    pub apm_fargate_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_host_sum", skip_serializing_if = "Option::is_none")]
    pub apm_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub apm_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_profiler_host_sum", skip_serializing_if = "Option::is_none")]
    pub apm_profiler_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_profiler_host_top99p", skip_serializing_if = "Option::is_none")]
    pub apm_profiler_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_trace_search_sum", skip_serializing_if = "Option::is_none")]
    pub apm_trace_search_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_fargate_average", skip_serializing_if = "Option::is_none")]
    pub application_security_fargate_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_host_sum", skip_serializing_if = "Option::is_none")]
    pub application_security_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_host_top99p", skip_serializing_if = "Option::is_none")]
    pub application_security_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_indexed_spans_sum", skip_serializing_if = "Option::is_none")]
    pub ci_pipeline_indexed_spans_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_maximum", skip_serializing_if = "Option::is_none")]
    pub ci_pipeline_maximum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_sum", skip_serializing_if = "Option::is_none")]
    pub ci_pipeline_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_test_indexed_spans_sum", skip_serializing_if = "Option::is_none")]
    pub ci_test_indexed_spans_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_testing_maximum", skip_serializing_if = "Option::is_none")]
    pub ci_testing_maximum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_testing_sum", skip_serializing_if = "Option::is_none")]
    pub ci_testing_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cloud_cost_management_average", skip_serializing_if = "Option::is_none")]
    pub cloud_cost_management_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cloud_cost_management_sum", skip_serializing_if = "Option::is_none")]
    pub cloud_cost_management_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_container_sum", skip_serializing_if = "Option::is_none")]
    pub cspm_container_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_host_sum", skip_serializing_if = "Option::is_none")]
    pub cspm_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cspm_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "custom_event_sum", skip_serializing_if = "Option::is_none")]
    pub custom_event_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_container_sum", skip_serializing_if = "Option::is_none")]
    pub cws_container_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_host_sum", skip_serializing_if = "Option::is_none")]
    pub cws_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_host_top99p", skip_serializing_if = "Option::is_none")]
    pub cws_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_host_sum", skip_serializing_if = "Option::is_none")]
    pub dbm_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub dbm_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_normalized_queries_average", skip_serializing_if = "Option::is_none")]
    pub dbm_normalized_queries_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_normalized_queries_sum", skip_serializing_if = "Option::is_none")]
    pub dbm_normalized_queries_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_apm_and_profiler_average", skip_serializing_if = "Option::is_none")]
    pub fargate_container_apm_and_profiler_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_apm_and_profiler_sum", skip_serializing_if = "Option::is_none")]
    pub fargate_container_apm_and_profiler_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_average", skip_serializing_if = "Option::is_none")]
    pub fargate_container_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_profiler_average", skip_serializing_if = "Option::is_none")]
    pub fargate_container_profiler_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_profiler_sum", skip_serializing_if = "Option::is_none")]
    pub fargate_container_profiler_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_sum", skip_serializing_if = "Option::is_none")]
    pub fargate_container_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "incident_management_maximum", skip_serializing_if = "Option::is_none")]
    pub incident_management_maximum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "incident_management_sum", skip_serializing_if = "Option::is_none")]
    pub incident_management_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_and_apm_host_sum", skip_serializing_if = "Option::is_none")]
    pub infra_and_apm_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_and_apm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub infra_and_apm_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_container_sum", skip_serializing_if = "Option::is_none")]
    pub infra_container_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_host_sum", skip_serializing_if = "Option::is_none")]
    pub infra_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_host_top99p", skip_serializing_if = "Option::is_none")]
    pub infra_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_spans_sum", skip_serializing_if = "Option::is_none")]
    pub ingested_spans_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_timeseries_average", skip_serializing_if = "Option::is_none")]
    pub ingested_timeseries_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_timeseries_sum", skip_serializing_if = "Option::is_none")]
    pub ingested_timeseries_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "iot_sum", skip_serializing_if = "Option::is_none")]
    pub iot_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "iot_top99p", skip_serializing_if = "Option::is_none")]
    pub iot_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "lambda_function_average", skip_serializing_if = "Option::is_none")]
    pub lambda_function_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "lambda_function_sum", skip_serializing_if = "Option::is_none")]
    pub lambda_function_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_forwarding_sum", skip_serializing_if = "Option::is_none")]
    pub logs_forwarding_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_15day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_15day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_180day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_180day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_30day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_30day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_360day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_360day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_3day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_3day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_45day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_45day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_60day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_60day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_7day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_7day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_90day_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_90day_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_custom_retention_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_custom_retention_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_sum", skip_serializing_if = "Option::is_none")]
    pub logs_indexed_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_ingested_sum", skip_serializing_if = "Option::is_none")]
    pub logs_ingested_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "network_device_sum", skip_serializing_if = "Option::is_none")]
    pub network_device_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "network_device_top99p", skip_serializing_if = "Option::is_none")]
    pub network_device_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_flow_sum", skip_serializing_if = "Option::is_none")]
    pub npm_flow_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_host_sum", skip_serializing_if = "Option::is_none")]
    pub npm_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_host_top99p", skip_serializing_if = "Option::is_none")]
    pub npm_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "observability_pipeline_sum", skip_serializing_if = "Option::is_none")]
    pub observability_pipeline_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "online_archive_sum", skip_serializing_if = "Option::is_none")]
    pub online_archive_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_container_sum", skip_serializing_if = "Option::is_none")]
    pub prof_container_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_host_sum", skip_serializing_if = "Option::is_none")]
    pub prof_host_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_host_top99p", skip_serializing_if = "Option::is_none")]
    pub prof_host_top99p: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_lite_sum", skip_serializing_if = "Option::is_none")]
    pub rum_lite_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_replay_sum", skip_serializing_if = "Option::is_none")]
    pub rum_replay_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_sum", skip_serializing_if = "Option::is_none")]
    pub rum_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_units_sum", skip_serializing_if = "Option::is_none")]
    pub rum_units_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "sensitive_data_scanner_sum", skip_serializing_if = "Option::is_none")]
    pub sensitive_data_scanner_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_apm_sum", skip_serializing_if = "Option::is_none")]
    pub serverless_apm_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_infra_average", skip_serializing_if = "Option::is_none")]
    pub serverless_infra_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_infra_sum", skip_serializing_if = "Option::is_none")]
    pub serverless_infra_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_invocation_sum", skip_serializing_if = "Option::is_none")]
    pub serverless_invocation_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "siem_sum", skip_serializing_if = "Option::is_none")]
    pub siem_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "standard_timeseries_average", skip_serializing_if = "Option::is_none")]
    pub standard_timeseries_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_api_tests_sum", skip_serializing_if = "Option::is_none")]
    pub synthetics_api_tests_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_app_testing_maximum", skip_serializing_if = "Option::is_none")]
    pub synthetics_app_testing_maximum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_browser_checks_sum", skip_serializing_if = "Option::is_none")]
    pub synthetics_browser_checks_sum: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "timeseries_average", skip_serializing_if = "Option::is_none")]
    pub timeseries_average: UsageBillableSummaryBody,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "timeseries_sum", skip_serializing_if = "Option::is_none")]
    pub timeseries_sum: UsageBillableSummaryBody,
}

