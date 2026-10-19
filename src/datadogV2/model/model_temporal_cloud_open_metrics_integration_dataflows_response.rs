// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Temporal Cloud OpenMetrics, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporalCloudOpenMetricsIntegrationDataflowsResponse {
    /// Temporal Cloud operational metrics for your namespaces, workflows and task queues, such as workflow and activity rates, task polling, and service request latency, collected from Temporal Cloud's OpenMetrics endpoint. This is the integration's only collection: it is always on and cannot be turned off, which is why it appears in responses only.
    #[serde(rename = "temporal-cloud-openmetrics-metrics")]
    pub temporal_cloud_openmetrics_metrics:
        Option<crate::datadogV2::model::TemporalCloudOpenMetricsMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporalCloudOpenMetricsIntegrationDataflowsResponse {
    pub fn new() -> TemporalCloudOpenMetricsIntegrationDataflowsResponse {
        TemporalCloudOpenMetricsIntegrationDataflowsResponse {
            temporal_cloud_openmetrics_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn temporal_cloud_openmetrics_metrics(
        mut self,
        value: crate::datadogV2::model::TemporalCloudOpenMetricsMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.temporal_cloud_openmetrics_metrics = Some(value);
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

impl Default for TemporalCloudOpenMetricsIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TemporalCloudOpenMetricsIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporalCloudOpenMetricsIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for TemporalCloudOpenMetricsIntegrationDataflowsResponseVisitor {
            type Value = TemporalCloudOpenMetricsIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut temporal_cloud_openmetrics_metrics: Option<crate::datadogV2::model::TemporalCloudOpenMetricsMetricsIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "temporal-cloud-openmetrics-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            temporal_cloud_openmetrics_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TemporalCloudOpenMetricsIntegrationDataflowsResponse {
                    temporal_cloud_openmetrics_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TemporalCloudOpenMetricsIntegrationDataflowsResponseVisitor)
    }
}
