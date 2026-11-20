// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Anthropic Usage and Costs, keyed by dataflow ID. Datadog always collects usage metrics. Each dataflow in this object adds another type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values. If a dataflow depends on a Datadog product, its description names that product. If you enable a dataflow without that product, Datadog stores the setting but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnthropicUsageAndCostsIntegrationDataflowsRequest {
    /// Anthropic costs by model, workspace, and service tier, in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Requires Cloud Cost Management on your Datadog organization. Without it, this dataflow collects no data. Cost data appears in Cloud Cost Management within 24 to 48 hours of enabling this dataflow.
    #[serde(rename = "anthropic-cloud-cost-metrics")]
    pub anthropic_cloud_cost_metrics: Option<
        crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnthropicUsageAndCostsIntegrationDataflowsRequest {
    pub fn new() -> AnthropicUsageAndCostsIntegrationDataflowsRequest {
        AnthropicUsageAndCostsIntegrationDataflowsRequest {
            anthropic_cloud_cost_metrics: None,
            _unparsed: false,
        }
    }

    pub fn anthropic_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.anthropic_cloud_cost_metrics = Some(value);
        self
    }
}

impl Default for AnthropicUsageAndCostsIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnthropicUsageAndCostsIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnthropicUsageAndCostsIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for AnthropicUsageAndCostsIntegrationDataflowsRequestVisitor {
            type Value = AnthropicUsageAndCostsIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anthropic_cloud_cost_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anthropic-cloud-cost-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            anthropic_cloud_cost_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AnthropicUsageAndCostsIntegrationDataflowsRequest {
                    anthropic_cloud_cost_metrics,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnthropicUsageAndCostsIntegrationDataflowsRequestVisitor)
    }
}
