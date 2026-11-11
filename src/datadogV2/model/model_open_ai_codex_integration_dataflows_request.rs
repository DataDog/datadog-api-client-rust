// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from OpenAI Codex, keyed by dataflow id. Daily Codex analytics metrics are always collected; each dataflow here adds a further kind of collection on top of that baseline, so set `enabled` to start or stop it. Defaults listed on each dataflow apply when the account is created; on update, omitted fields keep their current values. Every dataflow requires an OpenAI enterprise subscription with Codex enabled; a dataflow enabled without it is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationDataflowsRequest {
    /// A daily per-engineer engagement signal that [AI Impact](<https://docs.datadoghq.com/delivery_performance/ai_impact/>) uses to attribute AI-assisted pull requests.
    #[serde(rename = "openai-codex-dora")]
    pub openai_codex_dora:
        Option<crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowRequest>,
    /// Hourly turn-level metrics from the OpenAI Compliance Logs Platform, including turn and session identity, Work Insights classification, token usage, credits, estimated cost, duration, and skill, plugin, and artifact activity. Turn cost is a provider estimate and can differ from invoiced charges. Requires the `chatgpt.enterprise.compliance_logs_platform.workspace_analytics.read` scope on the key in `authentication`; without it this dataflow collects no data.
    #[serde(rename = "openai-codex-turn-metrics")]
    pub openai_codex_turn_metrics:
        Option<crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAICodexIntegrationDataflowsRequest {
    pub fn new() -> OpenAICodexIntegrationDataflowsRequest {
        OpenAICodexIntegrationDataflowsRequest {
            openai_codex_dora: None,
            openai_codex_turn_metrics: None,
            _unparsed: false,
        }
    }

    pub fn openai_codex_dora(
        mut self,
        value: crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowRequest,
    ) -> Self {
        self.openai_codex_dora = Some(value);
        self
    }

    pub fn openai_codex_turn_metrics(
        mut self,
        value: crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.openai_codex_turn_metrics = Some(value);
        self
    }
}

impl Default for OpenAICodexIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationDataflowsRequestVisitor {
            type Value = OpenAICodexIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_codex_dora: Option<
                    crate::datadogV2::model::OpenAICodexDoraIntegrationDataflowRequest,
                > = None;
                let mut openai_codex_turn_metrics: Option<
                    crate::datadogV2::model::OpenAICodexTurnMetricsIntegrationDataflowRequest,
                > = None;
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
                        "openai-codex-turn-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_codex_turn_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAICodexIntegrationDataflowsRequest {
                    openai_codex_dora,
                    openai_codex_turn_metrics,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationDataflowsRequestVisitor)
    }
}
