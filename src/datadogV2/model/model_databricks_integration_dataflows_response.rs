// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Databricks, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationDataflowsResponse {
    /// Cost data collected from your Databricks system tables. Requires [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>) to be set up for your organization.
    #[serde(rename = "databricks-cloud-cost-metrics")]
    pub databricks_cloud_cost_metrics: Option<crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowResponse>,
    /// Data Jobs Monitoring, which collects performance, reliability, and cost data for your Databricks jobs.
    #[serde(rename = "databricks-data-observability-jobs-monitoring")]
    pub databricks_data_observability_jobs_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse>,
    /// Data Observability, which collects lineage and data quality information from your Databricks catalogs so you can explore how data flows and detect, resolve, and prevent quality issues.
    #[serde(rename = "databricks-data-observability-quality-monitoring")]
    pub databricks_data_observability_quality_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowResponse>,
    /// Health and usage metrics for your Databricks model serving endpoints. Not supported on accounts that authenticate with `private-action-runner`; on those accounts this dataflow collects no data even when enabled.
    #[serde(rename = "databricks-model-serving-metrics")]
    pub databricks_model_serving_metrics: Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatabricksIntegrationDataflowsResponse {
    pub fn new() -> DatabricksIntegrationDataflowsResponse {
        DatabricksIntegrationDataflowsResponse {
            databricks_cloud_cost_metrics: None,
            databricks_data_observability_jobs_monitoring: None,
            databricks_data_observability_quality_monitoring: None,
            databricks_model_serving_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn databricks_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.databricks_cloud_cost_metrics = Some(value);
        self
    }

    pub fn databricks_data_observability_jobs_monitoring(
        mut self,
        value: crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse,
    ) -> Self {
        self.databricks_data_observability_jobs_monitoring = Some(value);
        self
    }

    pub fn databricks_data_observability_quality_monitoring(
        mut self,
        value: crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowResponse,
    ) -> Self {
        self.databricks_data_observability_quality_monitoring = Some(value);
        self
    }

    pub fn databricks_model_serving_metrics(
        mut self,
        value: crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.databricks_model_serving_metrics = Some(value);
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

impl Default for DatabricksIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationDataflowsResponseVisitor {
            type Value = DatabricksIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut databricks_cloud_cost_metrics: Option<
                    crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowResponse,
                > = None;
                let mut databricks_data_observability_jobs_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowResponse> = None;
                let mut databricks_data_observability_quality_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowResponse> = None;
                let mut databricks_model_serving_metrics: Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "databricks-cloud-cost-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_cloud_cost_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-data-observability-jobs-monitoring" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_data_observability_jobs_monitoring =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-data-observability-quality-monitoring" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_data_observability_quality_monitoring =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-model-serving-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_model_serving_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DatabricksIntegrationDataflowsResponse {
                    databricks_cloud_cost_metrics,
                    databricks_data_observability_jobs_monitoring,
                    databricks_data_observability_quality_monitoring,
                    databricks_model_serving_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationDataflowsResponseVisitor)
    }
}
