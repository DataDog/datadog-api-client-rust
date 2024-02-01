// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with aggregated usage types.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryKeys {
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_fargate_average")]
    pub apm_fargate_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_fargate_sum")]
    pub apm_fargate_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_host_sum")]
    pub apm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_host_top99p")]
    pub apm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_profiler_host_sum")]
    pub apm_profiler_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_profiler_host_top99p")]
    pub apm_profiler_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "apm_trace_search_sum")]
    pub apm_trace_search_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_fargate_average")]
    pub application_security_fargate_average:
        Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_host_sum")]
    pub application_security_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "application_security_host_top99p")]
    pub application_security_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_indexed_spans_sum")]
    pub ci_pipeline_indexed_spans_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_maximum")]
    pub ci_pipeline_maximum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_pipeline_sum")]
    pub ci_pipeline_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_test_indexed_spans_sum")]
    pub ci_test_indexed_spans_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_testing_maximum")]
    pub ci_testing_maximum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ci_testing_sum")]
    pub ci_testing_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cloud_cost_management_average")]
    pub cloud_cost_management_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cloud_cost_management_sum")]
    pub cloud_cost_management_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_container_sum")]
    pub cspm_container_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_host_sum")]
    pub cspm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cspm_host_top99p")]
    pub cspm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "custom_event_sum")]
    pub custom_event_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_container_sum")]
    pub cws_container_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_host_sum")]
    pub cws_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "cws_host_top99p")]
    pub cws_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_host_sum")]
    pub dbm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_host_top99p")]
    pub dbm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_normalized_queries_average")]
    pub dbm_normalized_queries_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "dbm_normalized_queries_sum")]
    pub dbm_normalized_queries_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_apm_and_profiler_average")]
    pub fargate_container_apm_and_profiler_average:
        Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_apm_and_profiler_sum")]
    pub fargate_container_apm_and_profiler_sum:
        Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_average")]
    pub fargate_container_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_profiler_average")]
    pub fargate_container_profiler_average:
        Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_profiler_sum")]
    pub fargate_container_profiler_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "fargate_container_sum")]
    pub fargate_container_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "incident_management_maximum")]
    pub incident_management_maximum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "incident_management_sum")]
    pub incident_management_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_and_apm_host_sum")]
    pub infra_and_apm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_and_apm_host_top99p")]
    pub infra_and_apm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_container_sum")]
    pub infra_container_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_host_sum")]
    pub infra_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "infra_host_top99p")]
    pub infra_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_spans_sum")]
    pub ingested_spans_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_timeseries_average")]
    pub ingested_timeseries_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "ingested_timeseries_sum")]
    pub ingested_timeseries_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "iot_sum")]
    pub iot_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "iot_top99p")]
    pub iot_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "lambda_function_average")]
    pub lambda_function_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "lambda_function_sum")]
    pub lambda_function_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_forwarding_sum")]
    pub logs_forwarding_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_15day_sum")]
    pub logs_indexed_15day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_180day_sum")]
    pub logs_indexed_180day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_30day_sum")]
    pub logs_indexed_30day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_360day_sum")]
    pub logs_indexed_360day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_3day_sum")]
    pub logs_indexed_3day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_45day_sum")]
    pub logs_indexed_45day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_60day_sum")]
    pub logs_indexed_60day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_7day_sum")]
    pub logs_indexed_7day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_90day_sum")]
    pub logs_indexed_90day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_custom_retention_sum")]
    pub logs_indexed_custom_retention_sum:
        Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_indexed_sum")]
    pub logs_indexed_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "logs_ingested_sum")]
    pub logs_ingested_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "network_device_sum")]
    pub network_device_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "network_device_top99p")]
    pub network_device_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_flow_sum")]
    pub npm_flow_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_host_sum")]
    pub npm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "npm_host_top99p")]
    pub npm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "observability_pipeline_sum")]
    pub observability_pipeline_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "online_archive_sum")]
    pub online_archive_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_container_sum")]
    pub prof_container_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_host_sum")]
    pub prof_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "prof_host_top99p")]
    pub prof_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_lite_sum")]
    pub rum_lite_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_replay_sum")]
    pub rum_replay_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_sum")]
    pub rum_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "rum_units_sum")]
    pub rum_units_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "sensitive_data_scanner_sum")]
    pub sensitive_data_scanner_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_apm_sum")]
    pub serverless_apm_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_infra_average")]
    pub serverless_infra_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_infra_sum")]
    pub serverless_infra_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "serverless_invocation_sum")]
    pub serverless_invocation_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "siem_sum")]
    pub siem_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "standard_timeseries_average")]
    pub standard_timeseries_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_api_tests_sum")]
    pub synthetics_api_tests_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_app_testing_maximum")]
    pub synthetics_app_testing_maximum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "synthetics_browser_checks_sum")]
    pub synthetics_browser_checks_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "timeseries_average")]
    pub timeseries_average: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
    /// Response with properties for each aggregated usage type.
    #[serde(rename = "timeseries_sum")]
    pub timeseries_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
}

impl UsageBillableSummaryKeys {
    pub fn new() -> UsageBillableSummaryKeys {
        UsageBillableSummaryKeys {
            apm_fargate_average: None,
            apm_fargate_sum: None,
            apm_host_sum: None,
            apm_host_top99p: None,
            apm_profiler_host_sum: None,
            apm_profiler_host_top99p: None,
            apm_trace_search_sum: None,
            application_security_fargate_average: None,
            application_security_host_sum: None,
            application_security_host_top99p: None,
            ci_pipeline_indexed_spans_sum: None,
            ci_pipeline_maximum: None,
            ci_pipeline_sum: None,
            ci_test_indexed_spans_sum: None,
            ci_testing_maximum: None,
            ci_testing_sum: None,
            cloud_cost_management_average: None,
            cloud_cost_management_sum: None,
            cspm_container_sum: None,
            cspm_host_sum: None,
            cspm_host_top99p: None,
            custom_event_sum: None,
            cws_container_sum: None,
            cws_host_sum: None,
            cws_host_top99p: None,
            dbm_host_sum: None,
            dbm_host_top99p: None,
            dbm_normalized_queries_average: None,
            dbm_normalized_queries_sum: None,
            fargate_container_apm_and_profiler_average: None,
            fargate_container_apm_and_profiler_sum: None,
            fargate_container_average: None,
            fargate_container_profiler_average: None,
            fargate_container_profiler_sum: None,
            fargate_container_sum: None,
            incident_management_maximum: None,
            incident_management_sum: None,
            infra_and_apm_host_sum: None,
            infra_and_apm_host_top99p: None,
            infra_container_sum: None,
            infra_host_sum: None,
            infra_host_top99p: None,
            ingested_spans_sum: None,
            ingested_timeseries_average: None,
            ingested_timeseries_sum: None,
            iot_sum: None,
            iot_top99p: None,
            lambda_function_average: None,
            lambda_function_sum: None,
            logs_forwarding_sum: None,
            logs_indexed_15day_sum: None,
            logs_indexed_180day_sum: None,
            logs_indexed_30day_sum: None,
            logs_indexed_360day_sum: None,
            logs_indexed_3day_sum: None,
            logs_indexed_45day_sum: None,
            logs_indexed_60day_sum: None,
            logs_indexed_7day_sum: None,
            logs_indexed_90day_sum: None,
            logs_indexed_custom_retention_sum: None,
            logs_indexed_sum: None,
            logs_ingested_sum: None,
            network_device_sum: None,
            network_device_top99p: None,
            npm_flow_sum: None,
            npm_host_sum: None,
            npm_host_top99p: None,
            observability_pipeline_sum: None,
            online_archive_sum: None,
            prof_container_sum: None,
            prof_host_sum: None,
            prof_host_top99p: None,
            rum_lite_sum: None,
            rum_replay_sum: None,
            rum_sum: None,
            rum_units_sum: None,
            sensitive_data_scanner_sum: None,
            serverless_apm_sum: None,
            serverless_infra_average: None,
            serverless_infra_sum: None,
            serverless_invocation_sum: None,
            siem_sum: None,
            standard_timeseries_average: None,
            synthetics_api_tests_sum: None,
            synthetics_app_testing_maximum: None,
            synthetics_browser_checks_sum: None,
            timeseries_average: None,
            timeseries_sum: None,
        }
    }

    pub fn apm_fargate_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_fargate_average = Some(value);
        self
    }

    pub fn apm_fargate_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_fargate_sum = Some(value);
        self
    }

    pub fn apm_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_host_sum = Some(value);
        self
    }

    pub fn apm_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_host_top99p = Some(value);
        self
    }

    pub fn apm_profiler_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_profiler_host_sum = Some(value);
        self
    }

    pub fn apm_profiler_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_profiler_host_top99p = Some(value);
        self
    }

    pub fn apm_trace_search_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.apm_trace_search_sum = Some(value);
        self
    }

    pub fn application_security_fargate_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.application_security_fargate_average = Some(value);
        self
    }

    pub fn application_security_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.application_security_host_sum = Some(value);
        self
    }

    pub fn application_security_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.application_security_host_top99p = Some(value);
        self
    }

    pub fn ci_pipeline_indexed_spans_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_pipeline_indexed_spans_sum = Some(value);
        self
    }

    pub fn ci_pipeline_maximum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_pipeline_maximum = Some(value);
        self
    }

    pub fn ci_pipeline_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_pipeline_sum = Some(value);
        self
    }

    pub fn ci_test_indexed_spans_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_test_indexed_spans_sum = Some(value);
        self
    }

    pub fn ci_testing_maximum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_testing_maximum = Some(value);
        self
    }

    pub fn ci_testing_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ci_testing_sum = Some(value);
        self
    }

    pub fn cloud_cost_management_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cloud_cost_management_average = Some(value);
        self
    }

    pub fn cloud_cost_management_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cloud_cost_management_sum = Some(value);
        self
    }

    pub fn cspm_container_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cspm_container_sum = Some(value);
        self
    }

    pub fn cspm_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cspm_host_sum = Some(value);
        self
    }

    pub fn cspm_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cspm_host_top99p = Some(value);
        self
    }

    pub fn custom_event_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.custom_event_sum = Some(value);
        self
    }

    pub fn cws_container_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cws_container_sum = Some(value);
        self
    }

    pub fn cws_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cws_host_sum = Some(value);
        self
    }

    pub fn cws_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.cws_host_top99p = Some(value);
        self
    }

    pub fn dbm_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.dbm_host_sum = Some(value);
        self
    }

    pub fn dbm_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.dbm_host_top99p = Some(value);
        self
    }

    pub fn dbm_normalized_queries_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.dbm_normalized_queries_average = Some(value);
        self
    }

    pub fn dbm_normalized_queries_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.dbm_normalized_queries_sum = Some(value);
        self
    }

    pub fn fargate_container_apm_and_profiler_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_apm_and_profiler_average = Some(value);
        self
    }

    pub fn fargate_container_apm_and_profiler_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_apm_and_profiler_sum = Some(value);
        self
    }

    pub fn fargate_container_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_average = Some(value);
        self
    }

    pub fn fargate_container_profiler_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_profiler_average = Some(value);
        self
    }

    pub fn fargate_container_profiler_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_profiler_sum = Some(value);
        self
    }

    pub fn fargate_container_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.fargate_container_sum = Some(value);
        self
    }

    pub fn incident_management_maximum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.incident_management_maximum = Some(value);
        self
    }

    pub fn incident_management_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.incident_management_sum = Some(value);
        self
    }

    pub fn infra_and_apm_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.infra_and_apm_host_sum = Some(value);
        self
    }

    pub fn infra_and_apm_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.infra_and_apm_host_top99p = Some(value);
        self
    }

    pub fn infra_container_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.infra_container_sum = Some(value);
        self
    }

    pub fn infra_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.infra_host_sum = Some(value);
        self
    }

    pub fn infra_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.infra_host_top99p = Some(value);
        self
    }

    pub fn ingested_spans_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ingested_spans_sum = Some(value);
        self
    }

    pub fn ingested_timeseries_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ingested_timeseries_average = Some(value);
        self
    }

    pub fn ingested_timeseries_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.ingested_timeseries_sum = Some(value);
        self
    }

    pub fn iot_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.iot_sum = Some(value);
        self
    }

    pub fn iot_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.iot_top99p = Some(value);
        self
    }

    pub fn lambda_function_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.lambda_function_average = Some(value);
        self
    }

    pub fn lambda_function_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.lambda_function_sum = Some(value);
        self
    }

    pub fn logs_forwarding_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_forwarding_sum = Some(value);
        self
    }

    pub fn logs_indexed_15day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_15day_sum = Some(value);
        self
    }

    pub fn logs_indexed_180day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_180day_sum = Some(value);
        self
    }

    pub fn logs_indexed_30day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_30day_sum = Some(value);
        self
    }

    pub fn logs_indexed_360day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_360day_sum = Some(value);
        self
    }

    pub fn logs_indexed_3day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_3day_sum = Some(value);
        self
    }

    pub fn logs_indexed_45day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_45day_sum = Some(value);
        self
    }

    pub fn logs_indexed_60day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_60day_sum = Some(value);
        self
    }

    pub fn logs_indexed_7day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_7day_sum = Some(value);
        self
    }

    pub fn logs_indexed_90day_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_90day_sum = Some(value);
        self
    }

    pub fn logs_indexed_custom_retention_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_custom_retention_sum = Some(value);
        self
    }

    pub fn logs_indexed_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_indexed_sum = Some(value);
        self
    }

    pub fn logs_ingested_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.logs_ingested_sum = Some(value);
        self
    }

    pub fn network_device_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.network_device_sum = Some(value);
        self
    }

    pub fn network_device_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.network_device_top99p = Some(value);
        self
    }

    pub fn npm_flow_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.npm_flow_sum = Some(value);
        self
    }

    pub fn npm_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.npm_host_sum = Some(value);
        self
    }

    pub fn npm_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.npm_host_top99p = Some(value);
        self
    }

    pub fn observability_pipeline_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.observability_pipeline_sum = Some(value);
        self
    }

    pub fn online_archive_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.online_archive_sum = Some(value);
        self
    }

    pub fn prof_container_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.prof_container_sum = Some(value);
        self
    }

    pub fn prof_host_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.prof_host_sum = Some(value);
        self
    }

    pub fn prof_host_top99p(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.prof_host_top99p = Some(value);
        self
    }

    pub fn rum_lite_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.rum_lite_sum = Some(value);
        self
    }

    pub fn rum_replay_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.rum_replay_sum = Some(value);
        self
    }

    pub fn rum_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.rum_sum = Some(value);
        self
    }

    pub fn rum_units_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.rum_units_sum = Some(value);
        self
    }

    pub fn sensitive_data_scanner_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.sensitive_data_scanner_sum = Some(value);
        self
    }

    pub fn serverless_apm_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.serverless_apm_sum = Some(value);
        self
    }

    pub fn serverless_infra_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.serverless_infra_average = Some(value);
        self
    }

    pub fn serverless_infra_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.serverless_infra_sum = Some(value);
        self
    }

    pub fn serverless_invocation_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.serverless_invocation_sum = Some(value);
        self
    }

    pub fn siem_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.siem_sum = Some(value);
        self
    }

    pub fn standard_timeseries_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.standard_timeseries_average = Some(value);
        self
    }

    pub fn synthetics_api_tests_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.synthetics_api_tests_sum = Some(value);
        self
    }

    pub fn synthetics_app_testing_maximum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.synthetics_app_testing_maximum = Some(value);
        self
    }

    pub fn synthetics_browser_checks_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.synthetics_browser_checks_sum = Some(value);
        self
    }

    pub fn timeseries_average(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.timeseries_average = Some(value);
        self
    }

    pub fn timeseries_sum(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> &mut Self {
        self.timeseries_sum = Some(value);
        self
    }
}

impl Default for UsageBillableSummaryKeys {
    fn default() -> Self {
        Self::new()
    }
}
