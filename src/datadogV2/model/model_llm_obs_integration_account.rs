// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A configured account for an LLM provider integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsIntegrationAccount {
    /// Provider-specific account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Human-readable name for the integration account.
    #[serde(rename = "account_name")]
    pub account_name: String,
    /// Provider region associated with the account, if applicable.
    #[serde(rename = "account_region")]
    pub account_region: Option<String>,
    /// Azure OpenAI-specific metadata for an integration account or inference request.
    #[serde(rename = "azure_openai_metadata")]
    pub azure_openai_metadata: Option<crate::datadogV2::model::LLMObsAzureOpenAIMetadata>,
    /// Unique identifier for the integration account.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the LLM provider integration.
    #[serde(rename = "integration")]
    pub integration: String,
    /// Vertex AI-specific metadata for an integration account or inference request.
    #[serde(rename = "vertex_ai_metadata")]
    pub vertex_ai_metadata: Option<crate::datadogV2::model::LLMObsVertexAIMetadata>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsIntegrationAccount {
    pub fn new(
        account_id: String,
        account_name: String,
        id: String,
        integration: String,
    ) -> LLMObsIntegrationAccount {
        LLMObsIntegrationAccount {
            account_id,
            account_name,
            account_region: None,
            azure_openai_metadata: None,
            id,
            integration,
            vertex_ai_metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_region(mut self, value: String) -> Self {
        self.account_region = Some(value);
        self
    }

    pub fn azure_openai_metadata(
        mut self,
        value: crate::datadogV2::model::LLMObsAzureOpenAIMetadata,
    ) -> Self {
        self.azure_openai_metadata = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsIntegrationAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsIntegrationAccountVisitor;
        impl<'a> Visitor<'a> for LLMObsIntegrationAccountVisitor {
            type Value = LLMObsIntegrationAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut account_name: Option<String> = None;
                let mut account_region: Option<String> = None;
                let mut azure_openai_metadata: Option<
                    crate::datadogV2::model::LLMObsAzureOpenAIMetadata,
                > = None;
                let mut id: Option<String> = None;
                let mut integration: Option<String> = None;
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
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_name" => {
                            account_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_region" => {
                            if v.is_null() {
                                continue;
                            }
                            account_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_openai_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_openai_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let account_name =
                    account_name.ok_or_else(|| M::Error::missing_field("account_name"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;

                let content = LLMObsIntegrationAccount {
                    account_id,
                    account_name,
                    account_region,
                    azure_openai_metadata,
                    id,
                    integration,
                    vertex_ai_metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsIntegrationAccountVisitor)
    }
}
