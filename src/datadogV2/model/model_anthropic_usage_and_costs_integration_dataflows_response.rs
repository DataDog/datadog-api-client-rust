// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Anthropic Usage and Costs, keyed by dataflow ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnthropicUsageAndCostsIntegrationDataflowsResponse {
    /// Anthropic costs by model, workspace, and service tier, in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Requires Cloud Cost Management on your Datadog organization. Without it, this dataflow collects no data.
    #[serde(rename = "anthropic-cloud-cost-metrics")]
    pub anthropic_cloud_cost_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowResponse>,
    /// Anthropic usage metrics, such as input, output, and cache token consumption by model, workspace, and service tier. With an Analytics API key, usage is also broken down by user, product, context window, inference geography, and speed. This dataflow is always on and cannot be turned off, which is why it appears in responses only.
    #[serde(rename = "anthropic-usage-metrics")]
    pub anthropic_usage_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsUsageMetricsIntegrationDataflowResponse>,
    /// Daily Claude usage data used for DORA Metrics. This dataflow is always on and cannot be turned off, which is why it appears in responses only.
    #[serde(rename = "claude-daily-usage-dora-metrics")]
    pub claude_daily_usage_dora_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsDailyUsageDoraMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AnthropicUsageAndCostsIntegrationDataflowsResponse {
    pub fn new() -> AnthropicUsageAndCostsIntegrationDataflowsResponse {
        AnthropicUsageAndCostsIntegrationDataflowsResponse {
            anthropic_cloud_cost_metrics: None,
            anthropic_usage_metrics: None,
            claude_daily_usage_dora_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn anthropic_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.anthropic_cloud_cost_metrics = Some(value);
        self
    }

    pub fn anthropic_usage_metrics(
        mut self,
        value: crate::datadogV2::model::AnthropicUsageAndCostsUsageMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.anthropic_usage_metrics = Some(value);
        self
    }

    pub fn claude_daily_usage_dora_metrics(
        mut self,
        value: crate::datadogV2::model::AnthropicUsageAndCostsDailyUsageDoraMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.claude_daily_usage_dora_metrics = Some(value);
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

impl Default for AnthropicUsageAndCostsIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnthropicUsageAndCostsIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnthropicUsageAndCostsIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for AnthropicUsageAndCostsIntegrationDataflowsResponseVisitor {
            type Value = AnthropicUsageAndCostsIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anthropic_cloud_cost_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowResponse> = None;
                let mut anthropic_usage_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsUsageMetricsIntegrationDataflowResponse> = None;
                let mut claude_daily_usage_dora_metrics: Option<crate::datadogV2::model::AnthropicUsageAndCostsDailyUsageDoraMetricsIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "anthropic-usage-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            anthropic_usage_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "claude-daily-usage-dora-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            claude_daily_usage_dora_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AnthropicUsageAndCostsIntegrationDataflowsResponse {
                    anthropic_cloud_cost_metrics,
                    anthropic_usage_metrics,
                    claude_daily_usage_dora_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnthropicUsageAndCostsIntegrationDataflowsResponseVisitor)
    }
}
