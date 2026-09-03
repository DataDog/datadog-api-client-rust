// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dataflows to configure on the Databricks integration account, keyed by dataflow id. Some dataflows and settings have prerequisites, noted on each. Those prerequisites are not checked when the request is made, so anything left enabled without them is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationDataflowsRequest {
    /// The Databricks cloud cost metrics dataflow.
    #[serde(rename = "databricks-cloud-cost-metrics")]
    pub databricks_cloud_cost_metrics:
        Option<crate::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest>,
    /// The Databricks Data Jobs Monitoring dataflow.
    #[serde(rename = "databricks-data-job-monitoring")]
    pub databricks_data_job_monitoring:
        Option<crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowRequest>,
    /// The Databricks data observability dataflow.
    #[serde(rename = "databricks-data-observability")]
    pub databricks_data_observability:
        Option<crate::datadogV2::model::DatabricksDataObservabilityIntegrationDataflowRequest>,
    /// The Databricks model serving metrics dataflow. Not supported on accounts that authenticate with `private-action-runner`; on those accounts this dataflow collects no data even when enabled.
    #[serde(rename = "databricks-model-serving-metrics")]
    pub databricks_model_serving_metrics:
        Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest>,
    /// The Databricks serverless jobs dataflow.
    #[serde(rename = "databricks-serverless-jobs")]
    pub databricks_serverless_jobs:
        Option<crate::datadogV2::model::DatabricksServerlessJobsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationDataflowsRequest {
    pub fn new() -> DatabricksIntegrationDataflowsRequest {
        DatabricksIntegrationDataflowsRequest {
            databricks_cloud_cost_metrics: None,
            databricks_data_job_monitoring: None,
            databricks_data_observability: None,
            databricks_model_serving_metrics: None,
            databricks_serverless_jobs: None,
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

    pub fn databricks_data_job_monitoring(
        mut self,
        value: crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_data_job_monitoring = Some(value);
        self
    }

    pub fn databricks_data_observability(
        mut self,
        value: crate::datadogV2::model::DatabricksDataObservabilityIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_data_observability = Some(value);
        self
    }

    pub fn databricks_model_serving_metrics(
        mut self,
        value: crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_model_serving_metrics = Some(value);
        self
    }

    pub fn databricks_serverless_jobs(
        mut self,
        value: crate::datadogV2::model::DatabricksServerlessJobsIntegrationDataflowRequest,
    ) -> Self {
        self.databricks_serverless_jobs = Some(value);
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
                let mut databricks_data_job_monitoring: Option<
                    crate::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowRequest,
                > = None;
                let mut databricks_data_observability: Option<
                    crate::datadogV2::model::DatabricksDataObservabilityIntegrationDataflowRequest,
                > = None;
                let mut databricks_model_serving_metrics: Option<crate::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest> = None;
                let mut databricks_serverless_jobs: Option<
                    crate::datadogV2::model::DatabricksServerlessJobsIntegrationDataflowRequest,
                > = None;
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
                        "databricks-data-job-monitoring" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_data_job_monitoring =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-data-observability" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_data_observability =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-model-serving-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_model_serving_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "databricks-serverless-jobs" => {
                            if v.is_null() {
                                continue;
                            }
                            databricks_serverless_jobs =
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
                    databricks_data_job_monitoring,
                    databricks_data_observability,
                    databricks_model_serving_metrics,
                    databricks_serverless_jobs,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationDataflowsRequestVisitor)
    }
}
