// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// LLM inference parameters for a custom evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigInferenceParams {
    /// Frequency penalty to reduce repetition.
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,
    /// Maximum number of tokens to generate.
    #[serde(rename = "max_tokens")]
    pub max_tokens: Option<i64>,
    /// Presence penalty to reduce repetition.
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f64>,
    /// Sampling temperature for the LLM.
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    /// Top-k sampling parameter.
    #[serde(rename = "top_k")]
    pub top_k: Option<i64>,
    /// Top-p (nucleus) sampling parameter.
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigInferenceParams {
    pub fn new() -> LLMObsCustomEvalConfigInferenceParams {
        LLMObsCustomEvalConfigInferenceParams {
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            temperature: None,
            top_k: None,
            top_p: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
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

impl Default for LLMObsCustomEvalConfigInferenceParams {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigInferenceParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigInferenceParamsVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigInferenceParamsVisitor {
            type Value = LLMObsCustomEvalConfigInferenceParams;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut frequency_penalty: Option<f64> = None;
                let mut max_tokens: Option<i64> = None;
                let mut presence_penalty: Option<f64> = None;
                let mut temperature: Option<f64> = None;
                let mut top_k: Option<i64> = None;
                let mut top_p: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "frequency_penalty" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            frequency_penalty =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            max_tokens = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "presence_penalty" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            presence_penalty =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "temperature" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            temperature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_k" => {
                            if v.is_null() {
                                continue;
                            }
                            top_k = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_p" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            top_p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsCustomEvalConfigInferenceParams {
                    frequency_penalty,
                    max_tokens,
                    presence_penalty,
                    temperature,
                    top_k,
                    top_p,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigInferenceParamsVisitor)
    }
}
