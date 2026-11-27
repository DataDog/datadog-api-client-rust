// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Usage and Costs, keyed by dataflow ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIUsageAndCostsIntegrationDataflowsResponse {
    /// OpenAI organization costs, which you can analyze in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). See [collected data](<https://docs.datadoghq.com/cloud_cost_management/saas_costs/?tab=openai#data-collected>) for what is included.
    #[serde(rename = "openai-cloud-cost-data")]
    pub openai_cloud_cost_data: Option<
        crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowResponse,
    >,
    /// Organization usage metrics under the `openai.*` namespace, such as `openai.completions.*`, `openai.embeddings.*`, `openai.images.*`, `openai.moderations.*`, `openai.audio_speeches.*`, `openai.audio_transcriptions.*`, `openai.code_interpreter_sessions.*`, and `openai.vector_stores.*`. This dataflow is always on and cannot be turned off, so it appears only in responses.
    #[serde(rename = "openai-usage-metrics")]
    pub openai_usage_metrics:
        Option<crate::datadogV2::model::OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIUsageAndCostsIntegrationDataflowsResponse {
    pub fn new() -> OpenAIUsageAndCostsIntegrationDataflowsResponse {
        OpenAIUsageAndCostsIntegrationDataflowsResponse {
            openai_cloud_cost_data: None,
            openai_usage_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn openai_cloud_cost_data(
        mut self,
        value: crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowResponse,
    ) -> Self {
        self.openai_cloud_cost_data = Some(value);
        self
    }

    pub fn openai_usage_metrics(
        mut self,
        value: crate::datadogV2::model::OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.openai_usage_metrics = Some(value);
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

impl Default for OpenAIUsageAndCostsIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIUsageAndCostsIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIUsageAndCostsIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for OpenAIUsageAndCostsIntegrationDataflowsResponseVisitor {
            type Value = OpenAIUsageAndCostsIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_cloud_cost_data: Option<crate::datadogV2::model::OpenAIUsageAndCostsCloudCostDataIntegrationDataflowResponse> = None;
                let mut openai_usage_metrics: Option<crate::datadogV2::model::OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openai-cloud-cost-data" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_cloud_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "openai-usage-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_usage_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OpenAIUsageAndCostsIntegrationDataflowsResponse {
                    openai_cloud_cost_data,
                    openai_usage_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAIUsageAndCostsIntegrationDataflowsResponseVisitor)
    }
}
