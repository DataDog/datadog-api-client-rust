// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Codex, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationDataflowsResponse {
    /// A daily per-engineer engagement signal that [AI Impact](<https://docs.datadoghq.com/delivery_performance/ai_impact/>) uses to attribute AI-assisted pull requests.
    #[serde(rename = "openai-codex-dora")]
    pub openai_codex_dora:
        Option<crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowResponse>,
    /// Daily Codex analytics metrics under the `openai.codex.*` namespace: usage totals, token counts by model, thread and turn counts by client, lines of code attributable to Codex, and code reviews. This dataflow is always on and cannot be turned off, which is why it appears in responses only.
    #[serde(rename = "openai-codex-metrics")]
    pub openai_codex_metrics:
        Option<crate::datadogV2::model::OpenAICodexMetricsIntegrationDataflowResponse>,
    /// Hourly turn-level metrics from the OpenAI Compliance Logs Platform, including turn and session identity, Work Insights classification, token usage, credits, estimated cost, duration, and skill, plugin, and artifact activity. Turn cost is a provider estimate and can differ from invoiced charges. Requires the `chatgpt.enterprise.compliance_logs_platform.workspace_analytics.read` scope on the key in `authentication`; without it this dataflow collects no data.
    #[serde(rename = "openai-codex-turn-metrics")]
    pub openai_codex_turn_metrics:
        Option<crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexIntegrationDataflowsResponse {
    pub fn new() -> OpenAICodexIntegrationDataflowsResponse {
        OpenAICodexIntegrationDataflowsResponse {
            openai_codex_dora: None,
            openai_codex_metrics: None,
            openai_codex_turn_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn openai_codex_dora(
        mut self,
        value: crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowResponse,
    ) -> Self {
        self.openai_codex_dora = Some(value);
        self
    }

    pub fn openai_codex_metrics(
        mut self,
        value: crate::datadogV2::model::OpenAICodexMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.openai_codex_metrics = Some(value);
        self
    }

    pub fn openai_codex_turn_metrics(
        mut self,
        value: crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.openai_codex_turn_metrics = Some(value);
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

impl Default for OpenAICodexIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationDataflowsResponseVisitor {
            type Value = OpenAICodexIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_codex_dora: Option<
                    crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowResponse,
                > = None;
                let mut openai_codex_metrics: Option<
                    crate::datadogV2::model::OpenAICodexMetricsIntegrationDataflowResponse,
                > = None;
                let mut openai_codex_turn_metrics: Option<
                    crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowResponse,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openai-codex-dora" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_codex_dora =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "openai-codex-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_codex_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "openai-codex-turn-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_codex_turn_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OpenAICodexIntegrationDataflowsResponse {
                    openai_codex_dora,
                    openai_codex_metrics,
                    openai_codex_turn_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationDataflowsResponseVisitor)
    }
}
