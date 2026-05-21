// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The output of a completed LLM inference call.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsInferenceRunResult {
    /// An optional assessment of the inference output quality.
    #[serialize_always]
    #[serde(rename = "assessment")]
    pub assessment: Option<String>,
    /// The text content of the model response.
    #[serde(rename = "content")]
    pub content: String,
    /// The reason the model stopped generating tokens.
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    /// List of generated code snippets for the inference configuration.
    #[serde(rename = "inference_codes")]
    pub inference_codes: Vec<crate::datadogV2::model::LLMObsInferenceCode>,
    /// Number of input tokens consumed.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i64,
    /// The model's internal reasoning or thinking output, if available.
    #[serde(rename = "internal_reasoning")]
    pub internal_reasoning: Option<crate::datadogV2::model::LLMObsInternalReasoning>,
    /// Request latency in milliseconds.
    #[serde(rename = "latency")]
    pub latency: i64,
    /// Number of output tokens generated.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i64,
    /// List of tools available to the model.
    #[serde(rename = "tools")]
    pub tools: Vec<crate::datadogV2::model::LLMObsInferenceTool>,
    /// Total tokens used (input plus output).
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsInferenceRunResult {
    pub fn new(
        assessment: Option<String>,
        content: String,
        finish_reason: String,
        inference_codes: Vec<crate::datadogV2::model::LLMObsInferenceCode>,
        input_tokens: i64,
        latency: i64,
        output_tokens: i64,
        tools: Vec<crate::datadogV2::model::LLMObsInferenceTool>,
        total_tokens: i64,
    ) -> LLMObsInferenceRunResult {
        LLMObsInferenceRunResult {
            assessment,
            content,
            finish_reason,
            inference_codes,
            input_tokens,
            internal_reasoning: None,
            latency,
            output_tokens,
            tools,
            total_tokens,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn internal_reasoning(
        mut self,
        value: crate::datadogV2::model::LLMObsInternalReasoning,
    ) -> Self {
        self.internal_reasoning = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsInferenceRunResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsInferenceRunResultVisitor;
        impl<'a> Visitor<'a> for LLMObsInferenceRunResultVisitor {
            type Value = LLMObsInferenceRunResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment: Option<Option<String>> = None;
                let mut content: Option<String> = None;
                let mut finish_reason: Option<String> = None;
                let mut inference_codes: Option<Vec<crate::datadogV2::model::LLMObsInferenceCode>> =
                    None;
                let mut input_tokens: Option<i64> = None;
                let mut internal_reasoning: Option<
                    crate::datadogV2::model::LLMObsInternalReasoning,
                > = None;
                let mut latency: Option<i64> = None;
                let mut output_tokens: Option<i64> = None;
                let mut tools: Option<Vec<crate::datadogV2::model::LLMObsInferenceTool>> = None;
                let mut total_tokens: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assessment" => {
                            assessment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finish_reason" => {
                            finish_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inference_codes" => {
                            inference_codes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens" => {
                            input_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "internal_reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            internal_reasoning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "latency" => {
                            latency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens" => {
                            output_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tools" => {
                            tools = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_tokens" => {
                            total_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let assessment = assessment.ok_or_else(|| M::Error::missing_field("assessment"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let finish_reason =
                    finish_reason.ok_or_else(|| M::Error::missing_field("finish_reason"))?;
                let inference_codes =
                    inference_codes.ok_or_else(|| M::Error::missing_field("inference_codes"))?;
                let input_tokens =
                    input_tokens.ok_or_else(|| M::Error::missing_field("input_tokens"))?;
                let latency = latency.ok_or_else(|| M::Error::missing_field("latency"))?;
                let output_tokens =
                    output_tokens.ok_or_else(|| M::Error::missing_field("output_tokens"))?;
                let tools = tools.ok_or_else(|| M::Error::missing_field("tools"))?;
                let total_tokens =
                    total_tokens.ok_or_else(|| M::Error::missing_field("total_tokens"))?;

                let content = LLMObsInferenceRunResult {
                    assessment,
                    content,
                    finish_reason,
                    inference_codes,
                    input_tokens,
                    internal_reasoning,
                    latency,
                    output_tokens,
                    tools,
                    total_tokens,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsInferenceRunResultVisitor)
    }
}
