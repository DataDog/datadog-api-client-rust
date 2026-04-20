// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// LLM provider configuration for a custom evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigLLMProvider {
    /// AWS Bedrock-specific options for LLM provider configuration.
    #[serde(rename = "bedrock")]
    pub bedrock: Option<crate::datadogV2::model::LLMObsCustomEvalConfigBedrockOptions>,
    /// Integration account identifier.
    #[serde(rename = "integration_account_id")]
    pub integration_account_id: Option<String>,
    /// Name of the LLM integration provider.
    #[serde(rename = "integration_provider")]
    pub integration_provider:
        Option<crate::datadogV2::model::LLMObsCustomEvalConfigIntegrationProvider>,
    /// Name of the LLM model.
    #[serde(rename = "model_name")]
    pub model_name: Option<String>,
    /// Google Vertex AI-specific options for LLM provider configuration.
    #[serde(rename = "vertex_ai")]
    pub vertex_ai: Option<crate::datadogV2::model::LLMObsCustomEvalConfigVertexAIOptions>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigLLMProvider {
    pub fn new() -> LLMObsCustomEvalConfigLLMProvider {
        LLMObsCustomEvalConfigLLMProvider {
            bedrock: None,
            integration_account_id: None,
            integration_provider: None,
            model_name: None,
            vertex_ai: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bedrock(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigBedrockOptions,
    ) -> Self {
        self.bedrock = Some(value);
        self
    }

    pub fn integration_account_id(mut self, value: String) -> Self {
        self.integration_account_id = Some(value);
        self
    }

    pub fn integration_provider(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigIntegrationProvider,
    ) -> Self {
        self.integration_provider = Some(value);
        self
    }

    pub fn model_name(mut self, value: String) -> Self {
        self.model_name = Some(value);
        self
    }

    pub fn vertex_ai(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigVertexAIOptions,
    ) -> Self {
        self.vertex_ai = Some(value);
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

impl Default for LLMObsCustomEvalConfigLLMProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigLLMProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigLLMProviderVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigLLMProviderVisitor {
            type Value = LLMObsCustomEvalConfigLLMProvider;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bedrock: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigBedrockOptions,
                > = None;
                let mut integration_account_id: Option<String> = None;
                let mut integration_provider: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigIntegrationProvider,
                > = None;
                let mut model_name: Option<String> = None;
                let mut vertex_ai: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigVertexAIOptions,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bedrock" => {
                            if v.is_null() {
                                continue;
                            }
                            bedrock = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration_provider) = integration_provider {
                                match _integration_provider {
                                    crate::datadogV2::model::LLMObsCustomEvalConfigIntegrationProvider::UnparsedObject(_integration_provider) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "model_name" => {
                            if v.is_null() {
                                continue;
                            }
                            model_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vertex_ai" => {
                            if v.is_null() {
                                continue;
                            }
                            vertex_ai = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsCustomEvalConfigLLMProvider {
                    bedrock,
                    integration_account_id,
                    integration_provider,
                    model_name,
                    vertex_ai,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigLLMProviderVisitor)
    }
}
