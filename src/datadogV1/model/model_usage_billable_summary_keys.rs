// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with aggregated usage types.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(rename = "logs_indexed_1day_sum")]
    pub logs_indexed_1day_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody>,
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            logs_indexed_1day_sum: None,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn apm_fargate_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_fargate_average = Some(value);
        self
    }

    pub fn apm_fargate_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_fargate_sum = Some(value);
        self
    }

    pub fn apm_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_host_sum = Some(value);
        self
    }

    pub fn apm_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_host_top99p = Some(value);
        self
    }

    pub fn apm_profiler_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_profiler_host_sum = Some(value);
        self
    }

    pub fn apm_profiler_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_profiler_host_top99p = Some(value);
        self
    }

    pub fn apm_trace_search_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.apm_trace_search_sum = Some(value);
        self
    }

    pub fn application_security_fargate_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.application_security_fargate_average = Some(value);
        self
    }

    pub fn application_security_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.application_security_host_sum = Some(value);
        self
    }

    pub fn application_security_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.application_security_host_top99p = Some(value);
        self
    }

    pub fn ci_pipeline_indexed_spans_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_pipeline_indexed_spans_sum = Some(value);
        self
    }

    pub fn ci_pipeline_maximum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_pipeline_maximum = Some(value);
        self
    }

    pub fn ci_pipeline_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_pipeline_sum = Some(value);
        self
    }

    pub fn ci_test_indexed_spans_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_test_indexed_spans_sum = Some(value);
        self
    }

    pub fn ci_testing_maximum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_testing_maximum = Some(value);
        self
    }

    pub fn ci_testing_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ci_testing_sum = Some(value);
        self
    }

    pub fn cloud_cost_management_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cloud_cost_management_average = Some(value);
        self
    }

    pub fn cloud_cost_management_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cloud_cost_management_sum = Some(value);
        self
    }

    pub fn cspm_container_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cspm_container_sum = Some(value);
        self
    }

    pub fn cspm_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cspm_host_sum = Some(value);
        self
    }

    pub fn cspm_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cspm_host_top99p = Some(value);
        self
    }

    pub fn custom_event_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.custom_event_sum = Some(value);
        self
    }

    pub fn cws_container_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cws_container_sum = Some(value);
        self
    }

    pub fn cws_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cws_host_sum = Some(value);
        self
    }

    pub fn cws_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.cws_host_top99p = Some(value);
        self
    }

    pub fn dbm_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.dbm_host_sum = Some(value);
        self
    }

    pub fn dbm_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.dbm_host_top99p = Some(value);
        self
    }

    pub fn dbm_normalized_queries_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.dbm_normalized_queries_average = Some(value);
        self
    }

    pub fn dbm_normalized_queries_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.dbm_normalized_queries_sum = Some(value);
        self
    }

    pub fn fargate_container_apm_and_profiler_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_apm_and_profiler_average = Some(value);
        self
    }

    pub fn fargate_container_apm_and_profiler_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_apm_and_profiler_sum = Some(value);
        self
    }

    pub fn fargate_container_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_average = Some(value);
        self
    }

    pub fn fargate_container_profiler_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_profiler_average = Some(value);
        self
    }

    pub fn fargate_container_profiler_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_profiler_sum = Some(value);
        self
    }

    pub fn fargate_container_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.fargate_container_sum = Some(value);
        self
    }

    pub fn incident_management_maximum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.incident_management_maximum = Some(value);
        self
    }

    pub fn incident_management_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.incident_management_sum = Some(value);
        self
    }

    pub fn infra_and_apm_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.infra_and_apm_host_sum = Some(value);
        self
    }

    pub fn infra_and_apm_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.infra_and_apm_host_top99p = Some(value);
        self
    }

    pub fn infra_container_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.infra_container_sum = Some(value);
        self
    }

    pub fn infra_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.infra_host_sum = Some(value);
        self
    }

    pub fn infra_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.infra_host_top99p = Some(value);
        self
    }

    pub fn ingested_spans_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ingested_spans_sum = Some(value);
        self
    }

    pub fn ingested_timeseries_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ingested_timeseries_average = Some(value);
        self
    }

    pub fn ingested_timeseries_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.ingested_timeseries_sum = Some(value);
        self
    }

    pub fn iot_sum(mut self, value: crate::datadogV1::model::UsageBillableSummaryBody) -> Self {
        self.iot_sum = Some(value);
        self
    }

    pub fn iot_top99p(mut self, value: crate::datadogV1::model::UsageBillableSummaryBody) -> Self {
        self.iot_top99p = Some(value);
        self
    }

    pub fn lambda_function_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.lambda_function_average = Some(value);
        self
    }

    pub fn lambda_function_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.lambda_function_sum = Some(value);
        self
    }

    pub fn logs_forwarding_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_forwarding_sum = Some(value);
        self
    }

    pub fn logs_indexed_15day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_15day_sum = Some(value);
        self
    }

    pub fn logs_indexed_180day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_180day_sum = Some(value);
        self
    }

    pub fn logs_indexed_1day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_1day_sum = Some(value);
        self
    }

    pub fn logs_indexed_30day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_30day_sum = Some(value);
        self
    }

    pub fn logs_indexed_360day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_360day_sum = Some(value);
        self
    }

    pub fn logs_indexed_3day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_3day_sum = Some(value);
        self
    }

    pub fn logs_indexed_45day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_45day_sum = Some(value);
        self
    }

    pub fn logs_indexed_60day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_60day_sum = Some(value);
        self
    }

    pub fn logs_indexed_7day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_7day_sum = Some(value);
        self
    }

    pub fn logs_indexed_90day_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_90day_sum = Some(value);
        self
    }

    pub fn logs_indexed_custom_retention_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_custom_retention_sum = Some(value);
        self
    }

    pub fn logs_indexed_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_indexed_sum = Some(value);
        self
    }

    pub fn logs_ingested_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.logs_ingested_sum = Some(value);
        self
    }

    pub fn network_device_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.network_device_sum = Some(value);
        self
    }

    pub fn network_device_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.network_device_top99p = Some(value);
        self
    }

    pub fn npm_flow_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.npm_flow_sum = Some(value);
        self
    }

    pub fn npm_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.npm_host_sum = Some(value);
        self
    }

    pub fn npm_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.npm_host_top99p = Some(value);
        self
    }

    pub fn observability_pipeline_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.observability_pipeline_sum = Some(value);
        self
    }

    pub fn online_archive_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.online_archive_sum = Some(value);
        self
    }

    pub fn prof_container_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.prof_container_sum = Some(value);
        self
    }

    pub fn prof_host_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.prof_host_sum = Some(value);
        self
    }

    pub fn prof_host_top99p(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.prof_host_top99p = Some(value);
        self
    }

    pub fn rum_lite_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.rum_lite_sum = Some(value);
        self
    }

    pub fn rum_replay_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.rum_replay_sum = Some(value);
        self
    }

    pub fn rum_sum(mut self, value: crate::datadogV1::model::UsageBillableSummaryBody) -> Self {
        self.rum_sum = Some(value);
        self
    }

    pub fn rum_units_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.rum_units_sum = Some(value);
        self
    }

    pub fn sensitive_data_scanner_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.sensitive_data_scanner_sum = Some(value);
        self
    }

    pub fn serverless_apm_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.serverless_apm_sum = Some(value);
        self
    }

    pub fn serverless_infra_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.serverless_infra_average = Some(value);
        self
    }

    pub fn serverless_infra_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.serverless_infra_sum = Some(value);
        self
    }

    pub fn serverless_invocation_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.serverless_invocation_sum = Some(value);
        self
    }

    pub fn siem_sum(mut self, value: crate::datadogV1::model::UsageBillableSummaryBody) -> Self {
        self.siem_sum = Some(value);
        self
    }

    pub fn standard_timeseries_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.standard_timeseries_average = Some(value);
        self
    }

    pub fn synthetics_api_tests_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.synthetics_api_tests_sum = Some(value);
        self
    }

    pub fn synthetics_app_testing_maximum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.synthetics_app_testing_maximum = Some(value);
        self
    }

    pub fn synthetics_browser_checks_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.synthetics_browser_checks_sum = Some(value);
        self
    }

    pub fn timeseries_average(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.timeseries_average = Some(value);
        self
    }

    pub fn timeseries_sum(
        mut self,
        value: crate::datadogV1::model::UsageBillableSummaryBody,
    ) -> Self {
        self.timeseries_sum = Some(value);
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

impl Default for UsageBillableSummaryKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageBillableSummaryKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageBillableSummaryKeysVisitor;
        impl<'a> Visitor<'a> for UsageBillableSummaryKeysVisitor {
            type Value = UsageBillableSummaryKeys;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_fargate_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut apm_fargate_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut apm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut apm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut apm_profiler_host_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut apm_profiler_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut apm_trace_search_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut application_security_fargate_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut application_security_host_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut application_security_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ci_pipeline_indexed_spans_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ci_pipeline_maximum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ci_pipeline_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut ci_test_indexed_spans_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ci_testing_maximum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ci_testing_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut cloud_cost_management_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut cloud_cost_management_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut cspm_container_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut cspm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut cspm_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut custom_event_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut cws_container_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut cws_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut cws_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut dbm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut dbm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut dbm_normalized_queries_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut dbm_normalized_queries_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_apm_and_profiler_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_apm_and_profiler_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_profiler_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_profiler_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut fargate_container_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut incident_management_maximum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut incident_management_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut infra_and_apm_host_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut infra_and_apm_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut infra_container_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut infra_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut infra_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ingested_spans_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ingested_timeseries_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut ingested_timeseries_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut iot_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> = None;
                let mut iot_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut lambda_function_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut lambda_function_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_forwarding_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_15day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_180day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_1day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_30day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_360day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_3day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_45day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_60day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_7day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_90day_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_custom_retention_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_indexed_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut logs_ingested_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut network_device_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut network_device_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut npm_flow_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut npm_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut npm_host_top99p: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut observability_pipeline_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut online_archive_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut prof_container_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut prof_host_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut prof_host_top99p: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut rum_lite_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut rum_replay_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut rum_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> = None;
                let mut rum_units_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut sensitive_data_scanner_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut serverless_apm_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut serverless_infra_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut serverless_infra_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut serverless_invocation_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut siem_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> = None;
                let mut standard_timeseries_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut synthetics_api_tests_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut synthetics_app_testing_maximum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut synthetics_browser_checks_sum: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut timeseries_average: Option<
                    crate::datadogV1::model::UsageBillableSummaryBody,
                > = None;
                let mut timeseries_sum: Option<crate::datadogV1::model::UsageBillableSummaryBody> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_fargate_average" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_fargate_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_fargate_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_profiler_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_profiler_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_profiler_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_profiler_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_trace_search_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_trace_search_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_security_fargate_average" => {
                            if v.is_null() {
                                continue;
                            }
                            application_security_fargate_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_security_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            application_security_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_security_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            application_security_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_indexed_spans_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_maximum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_maximum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_pipeline_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_pipeline_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_test_indexed_spans_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_testing_maximum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_testing_maximum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_testing_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ci_testing_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_average" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_cost_management_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_cost_management_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_container_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_container_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_event_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_event_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_container_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_container_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_normalized_queries_average" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_normalized_queries_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_normalized_queries_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            dbm_normalized_queries_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_apm_and_profiler_average" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_apm_and_profiler_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_apm_and_profiler_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_apm_and_profiler_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_average" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_profiler_average" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_profiler_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_profiler_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_profiler_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fargate_container_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            fargate_container_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_maximum" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_maximum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_management_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_management_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_and_apm_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_and_apm_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_and_apm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_and_apm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_container_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_container_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            infra_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_spans_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_spans_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_timeseries_average" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_timeseries_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_timeseries_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_timeseries_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iot_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iot_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            iot_top99p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambda_function_average" => {
                            if v.is_null() {
                                continue;
                            }
                            lambda_function_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambda_function_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            lambda_function_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_forwarding_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_forwarding_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_15day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_15day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_180day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_180day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_1day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_1day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_30day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_30day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_360day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_360day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_3day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_3day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_45day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_45day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_60day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_60day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_7day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_7day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_90day_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_90day_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_custom_retention_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_custom_retention_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_indexed_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_indexed_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_ingested_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_ingested_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network_device_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            network_device_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network_device_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            network_device_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_flow_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_flow_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "npm_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            npm_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observability_pipeline_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            observability_pipeline_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            online_archive_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prof_container_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            prof_container_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prof_host_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            prof_host_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prof_host_top99p" => {
                            if v.is_null() {
                                continue;
                            }
                            prof_host_top99p =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_lite_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_lite_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_replay_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_replay_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_units_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_units_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sensitive_data_scanner_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            sensitive_data_scanner_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_apm_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_apm_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_infra_average" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_infra_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_infra_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_infra_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serverless_invocation_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            serverless_invocation_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "siem_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            siem_sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "standard_timeseries_average" => {
                            if v.is_null() {
                                continue;
                            }
                            standard_timeseries_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_api_tests_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_api_tests_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_app_testing_maximum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_app_testing_maximum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics_browser_checks_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_browser_checks_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeseries_average" => {
                            if v.is_null() {
                                continue;
                            }
                            timeseries_average =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeseries_sum" => {
                            if v.is_null() {
                                continue;
                            }
                            timeseries_sum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageBillableSummaryKeys {
                    apm_fargate_average,
                    apm_fargate_sum,
                    apm_host_sum,
                    apm_host_top99p,
                    apm_profiler_host_sum,
                    apm_profiler_host_top99p,
                    apm_trace_search_sum,
                    application_security_fargate_average,
                    application_security_host_sum,
                    application_security_host_top99p,
                    ci_pipeline_indexed_spans_sum,
                    ci_pipeline_maximum,
                    ci_pipeline_sum,
                    ci_test_indexed_spans_sum,
                    ci_testing_maximum,
                    ci_testing_sum,
                    cloud_cost_management_average,
                    cloud_cost_management_sum,
                    cspm_container_sum,
                    cspm_host_sum,
                    cspm_host_top99p,
                    custom_event_sum,
                    cws_container_sum,
                    cws_host_sum,
                    cws_host_top99p,
                    dbm_host_sum,
                    dbm_host_top99p,
                    dbm_normalized_queries_average,
                    dbm_normalized_queries_sum,
                    fargate_container_apm_and_profiler_average,
                    fargate_container_apm_and_profiler_sum,
                    fargate_container_average,
                    fargate_container_profiler_average,
                    fargate_container_profiler_sum,
                    fargate_container_sum,
                    incident_management_maximum,
                    incident_management_sum,
                    infra_and_apm_host_sum,
                    infra_and_apm_host_top99p,
                    infra_container_sum,
                    infra_host_sum,
                    infra_host_top99p,
                    ingested_spans_sum,
                    ingested_timeseries_average,
                    ingested_timeseries_sum,
                    iot_sum,
                    iot_top99p,
                    lambda_function_average,
                    lambda_function_sum,
                    logs_forwarding_sum,
                    logs_indexed_15day_sum,
                    logs_indexed_180day_sum,
                    logs_indexed_1day_sum,
                    logs_indexed_30day_sum,
                    logs_indexed_360day_sum,
                    logs_indexed_3day_sum,
                    logs_indexed_45day_sum,
                    logs_indexed_60day_sum,
                    logs_indexed_7day_sum,
                    logs_indexed_90day_sum,
                    logs_indexed_custom_retention_sum,
                    logs_indexed_sum,
                    logs_ingested_sum,
                    network_device_sum,
                    network_device_top99p,
                    npm_flow_sum,
                    npm_host_sum,
                    npm_host_top99p,
                    observability_pipeline_sum,
                    online_archive_sum,
                    prof_container_sum,
                    prof_host_sum,
                    prof_host_top99p,
                    rum_lite_sum,
                    rum_replay_sum,
                    rum_sum,
                    rum_units_sum,
                    sensitive_data_scanner_sum,
                    serverless_apm_sum,
                    serverless_infra_average,
                    serverless_infra_sum,
                    serverless_invocation_sum,
                    siem_sum,
                    standard_timeseries_average,
                    synthetics_api_tests_sum,
                    synthetics_app_testing_maximum,
                    synthetics_browser_checks_sum,
                    timeseries_average,
                    timeseries_sum,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageBillableSummaryKeysVisitor)
    }
}
