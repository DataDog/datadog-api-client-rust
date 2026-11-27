// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Organization usage metrics under the `openai.*` namespace, such as `openai.completions.*`, `openai.embeddings.*`, `openai.images.*`, `openai.moderations.*`, `openai.audio_speeches.*`, `openai.audio_transcriptions.*`, `openai.code_interpreter_sessions.*`, and `openai.vector_stores.*`. This dataflow is always on and cannot be turned off, so it appears only in responses.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
    /// Whether Datadog collects this data. Always `true`, because this collection cannot be turned off.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Read-only collection status of a dataflow.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
    pub fn new() -> OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
        OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
            enabled: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountDataflowStatus,
    ) -> Self {
        self.status = Some(value);
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

impl Default for OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponseVisitor;
        impl<'a> Visitor<'a> for OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponseVisitor {
            type Value = OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponse {
                    enabled,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(OpenAIUsageAndCostsUsageMetricsIntegrationDataflowResponseVisitor)
    }
}
