// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters for an LLM inference request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsIntegrationInferenceRequest {
    /// Anthropic-specific metadata for an inference request.
    #[serde(rename = "anthropic_metadata")]
    pub anthropic_metadata: Option<crate::datadogV2::model::LLMObsAnthropicMetadata>,
    /// Azure OpenAI-specific metadata for an integration account or inference request.
    #[serde(rename = "azure_openai_metadata")]
    pub azure_openai_metadata: Option<crate::datadogV2::model::LLMObsAzureOpenAIMetadata>,
    /// Amazon Bedrock-specific metadata for an inference request.
    #[serde(rename = "bedrock_metadata")]
    pub bedrock_metadata: Option<crate::datadogV2::model::LLMObsBedrockMetadata>,
    /// Penalty for token frequency to reduce repetition.
    #[serde(
        rename = "frequency_penalty",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub frequency_penalty: Option<Option<f64>>,
    /// JSON schema for structured output, if supported by the model.
    #[serde(
        rename = "json_schema",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub json_schema: Option<Option<String>>,
    /// Maximum number of completion tokens to generate (alternative to max_tokens for some providers).
    #[serde(
        rename = "max_completion_tokens",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub max_completion_tokens: Option<Option<i64>>,
    /// Maximum number of tokens to generate.
    #[serde(
        rename = "max_tokens",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub max_tokens: Option<Option<i64>>,
    /// List of messages in an inference conversation.
    #[serde(rename = "messages")]
    pub messages: Vec<crate::datadogV2::model::LLMObsInferenceMessage>,
    /// The model identifier to use for inference.
    #[serde(rename = "model_id")]
    pub model_id: String,
    /// OpenAI-specific metadata for an inference request.
    #[serde(rename = "openai_metadata")]
    pub openai_metadata: Option<crate::datadogV2::model::LLMObsOpenAIMetadata>,
    /// Penalty for token presence to encourage topic diversity.
    #[serde(
        rename = "presence_penalty",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub presence_penalty: Option<Option<f64>>,
    /// Sampling temperature between 0 and 2. Higher values produce more random output.
    #[serde(
        rename = "temperature",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub temperature: Option<Option<f64>>,
    /// List of tools available to the model.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<crate::datadogV2::model::LLMObsInferenceTool>>,
    /// Top-K sampling parameter.
    #[serde(rename = "top_k", default, with = "::serde_with::rust::double_option")]
    pub top_k: Option<Option<i64>>,
    /// Nucleus sampling probability mass.
    #[serde(rename = "top_p", default, with = "::serde_with::rust::double_option")]
    pub top_p: Option<Option<f64>>,
    /// Vertex AI-specific metadata for an integration account or inference request.
    #[serde(rename = "vertex_ai_metadata")]
    pub vertex_ai_metadata: Option<crate::datadogV2::model::LLMObsVertexAIMetadata>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsIntegrationInferenceRequest {
    pub fn new(
        messages: Vec<crate::datadogV2::model::LLMObsInferenceMessage>,
        model_id: String,
    ) -> LLMObsIntegrationInferenceRequest {
        LLMObsIntegrationInferenceRequest {
            anthropic_metadata: None,
            azure_openai_metadata: None,
            bedrock_metadata: None,
            frequency_penalty: None,
            json_schema: None,
            max_completion_tokens: None,
            max_tokens: None,
            messages,
            model_id,
            openai_metadata: None,
            presence_penalty: None,
            temperature: None,
            tools: None,
            top_k: None,
            top_p: None,
            vertex_ai_metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn anthropic_metadata(
        mut self,
        value: crate::datadogV2::model::LLMObsAnthropicMetadata,
    ) -> Self {
        self.anthropic_metadata = Some(value);
        self
    }

    pub fn azure_openai_metadata(
        mut self,
        value: crate::datadogV2::model::LLMObsAzureOpenAIMetadata,
    ) -> Self {
        self.azure_openai_metadata = Some(value);
        self
    }

    pub fn bedrock_metadata(
        mut self,
        value: crate::datadogV2::model::LLMObsBedrockMetadata,
    ) -> Self {
        self.bedrock_metadata = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: Option<f64>) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn json_schema(mut self, value: Option<String>) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn max_completion_tokens(mut self, value: Option<i64>) -> Self {
        self.max_completion_tokens = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: Option<i64>) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn openai_metadata(mut self, value: crate::datadogV2::model::LLMObsOpenAIMetadata) -> Self {
        self.openai_metadata = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: Option<f64>) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn temperature(mut self, value: Option<f64>) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<crate::datadogV2::model::LLMObsInferenceTool>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn top_k(mut self, value: Option<i64>) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn top_p(mut self, value: Option<f64>) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn vertex_ai_metadata(
        mut self,
        value: crate::datadogV2::model::LLMObsVertexAIMetadata,
    ) -> Self {
        self.vertex_ai_metadata = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsIntegrationInferenceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsIntegrationInferenceRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsIntegrationInferenceRequestVisitor {
            type Value = LLMObsIntegrationInferenceRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anthropic_metadata: Option<
                    crate::datadogV2::model::LLMObsAnthropicMetadata,
                > = None;
                let mut azure_openai_metadata: Option<
                    crate::datadogV2::model::LLMObsAzureOpenAIMetadata,
                > = None;
                let mut bedrock_metadata: Option<crate::datadogV2::model::LLMObsBedrockMetadata> =
                    None;
                let mut frequency_penalty: Option<Option<f64>> = None;
                let mut json_schema: Option<Option<String>> = None;
                let mut max_completion_tokens: Option<Option<i64>> = None;
                let mut max_tokens: Option<Option<i64>> = None;
                let mut messages: Option<Vec<crate::datadogV2::model::LLMObsInferenceMessage>> =
                    None;
                let mut model_id: Option<String> = None;
                let mut openai_metadata: Option<crate::datadogV2::model::LLMObsOpenAIMetadata> =
                    None;
                let mut presence_penalty: Option<Option<f64>> = None;
                let mut temperature: Option<Option<f64>> = None;
                let mut tools: Option<Vec<crate::datadogV2::model::LLMObsInferenceTool>> = None;
                let mut top_k: Option<Option<i64>> = None;
                let mut top_p: Option<Option<f64>> = None;
                let mut vertex_ai_metadata: Option<
                    crate::datadogV2::model::LLMObsVertexAIMetadata,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anthropic_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            anthropic_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_openai_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_openai_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bedrock_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            bedrock_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "frequency_penalty" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            frequency_penalty =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "json_schema" => {
                            json_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_completion_tokens" => {
                            max_completion_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_tokens" => {
                            max_tokens = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "messages" => {
                            messages = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id" => {
                            model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "openai_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "presence_penalty" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            presence_penalty =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "temperature" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            temperature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tools" => {
                            if v.is_null() {
                                continue;
                            }
                            tools = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_k" => {
                            top_k = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_p" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            top_p = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vertex_ai_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            vertex_ai_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let messages = messages.ok_or_else(|| M::Error::missing_field("messages"))?;
                let model_id = model_id.ok_or_else(|| M::Error::missing_field("model_id"))?;

                let content = LLMObsIntegrationInferenceRequest {
                    anthropic_metadata,
                    azure_openai_metadata,
                    bedrock_metadata,
                    frequency_penalty,
                    json_schema,
                    max_completion_tokens,
                    max_tokens,
                    messages,
                    model_id,
                    openai_metadata,
                    presence_penalty,
                    temperature,
                    tools,
                    top_k,
                    top_p,
                    vertex_ai_metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsIntegrationInferenceRequestVisitor)
    }
}
