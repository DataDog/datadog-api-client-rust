// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Databricks, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to start or stop it, and use `settings` to tune what it gathers. The defaults noted below apply when the account is created; on update, anything left out keeps its current value. Some dataflows have prerequisites, noted on each; unless one is documented as rejecting the request, it is not verified, so a dataflow enabled without it is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationDataflowsRequest {
    /// Cost data collected from your Databricks system tables. [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>) must be enabled for your organization while this dataflow is on; any request that leaves it enabled without that is rejected with a `422` response.
    #[serde(rename = "databricks-cloud-cost-metrics")]
    pub databricks_cloud_cost_metrics: Option<crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest>,
    /// Data Jobs Monitoring, which collects performance, reliability, and cost data for your Databricks jobs.
    #[serde(rename = "databricks-data-observability-jobs-monitoring")]
    pub databricks_data_observability_jobs_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowRequest>,
    /// Data Observability, which collects lineage and data quality information from your Databricks catalogs so you can explore how data flows and detect, resolve, and prevent quality issues.
    #[serde(rename = "databricks-data-observability-quality-monitoring")]
    pub databricks_data_observability_quality_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowRequest>,
    /// Health and usage metrics for your Databricks model serving endpoints. Not supported on accounts that authenticate with `private-action-runner`; on those accounts this dataflow collects no data even when enabled.
    #[serde(rename = "databricks-model-serving-metrics")]
    pub databricks_model_serving_metrics: Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatabricksIntegrationDataflowsRequest {
    pub fn new() -> DatabricksIntegrationDataflowsRequest {
        DatabricksIntegrationDataflowsRequest {
            databricks_cloud_cost_metrics: None,
            databricks_data_observability_jobs_monitoring: None,
            databricks_data_observability_quality_monitoring: None,
            databricks_model_serving_metrics: None,
            _unparsed: false,
        }
    }

    pub fn databricks_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_cloud_cost_metrics = Some(value);
        self
    }

    pub fn databricks_data_observability_jobs_monitoring(
        mut self,
        value: crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_data_observability_jobs_monitoring = Some(value);
        self
    }

    pub fn databricks_data_observability_quality_monitoring(
        mut self,
        value: crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_data_observability_quality_monitoring = Some(value);
        self
    }

    pub fn databricks_model_serving_metrics(
        mut self,
        value: crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_model_serving_metrics = Some(value);
        self
    }
}

impl Default for DatabricksIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationDataflowsRequestVisitor {
            type Value = DatabricksIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut databricks_cloud_cost_metrics: Option<
                    crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest,
                > = None;
                let mut databricks_data_observability_jobs_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowRequest> = None;
                let mut databricks_data_observability_quality_monitoring: Option<crate::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowRequest> = None;
                let mut databricks_model_serving_metrics: Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest> = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = DatabricksIntegrationDataflowsRequest {
                    databricks_cloud_cost_metrics,
                    databricks_data_observability_jobs_monitoring,
                    databricks_data_observability_quality_monitoring,
                    databricks_model_serving_metrics,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationDataflowsRequestVisitor)
    }
}
