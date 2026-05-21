// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A model available for a given LLM provider integration and account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsIntegrationModel {
    /// Whether the account has access to this model.
    #[serde(rename = "has_access")]
    pub has_access: bool,
    /// Unique identifier for the model entry.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the LLM provider integration.
    #[serde(rename = "integration")]
    pub integration: String,
    /// Human-readable name of the LLM provider integration.
    #[serde(rename = "integration_display_name")]
    pub integration_display_name: String,
    /// Whether the model supports structured output via JSON schema.
    #[serde(rename = "json_schema")]
    pub json_schema: bool,
    /// Human-readable model name.
    #[serde(rename = "model_display_name")]
    pub model_display_name: String,
    /// Provider-specific model identifier used in inference calls.
    #[serde(rename = "model_id")]
    pub model_id: String,
    /// The underlying model provider.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Human-readable name of the underlying model provider.
    #[serde(rename = "provider_display_name")]
    pub provider_display_name: String,
    /// Map of region-specific model ID prefix overrides.
    #[serde(rename = "region_prefix_overrides")]
    pub region_prefix_overrides: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsIntegrationModel {
    pub fn new(
        has_access: bool,
        id: String,
        integration: String,
        integration_display_name: String,
        json_schema: bool,
        model_display_name: String,
        model_id: String,
        provider: String,
        provider_display_name: String,
    ) -> LLMObsIntegrationModel {
        LLMObsIntegrationModel {
            has_access,
            id,
            integration,
            integration_display_name,
            json_schema,
            model_display_name,
            model_id,
            provider,
            provider_display_name,
            region_prefix_overrides: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn region_prefix_overrides(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.region_prefix_overrides = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsIntegrationModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsIntegrationModelVisitor;
        impl<'a> Visitor<'a> for LLMObsIntegrationModelVisitor {
            type Value = LLMObsIntegrationModel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_access: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut integration: Option<String> = None;
                let mut integration_display_name: Option<String> = None;
                let mut json_schema: Option<bool> = None;
                let mut model_display_name: Option<String> = None;
                let mut model_id: Option<String> = None;
                let mut provider: Option<String> = None;
                let mut provider_display_name: Option<String> = None;
                let mut region_prefix_overrides: Option<
                    std::collections::BTreeMap<String, String>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_access" => {
                            has_access = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_display_name" => {
                            integration_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "json_schema" => {
                            json_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_display_name" => {
                            model_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id" => {
                            model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_display_name" => {
                            provider_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region_prefix_overrides" => {
                            if v.is_null() {
                                continue;
                            }
                            region_prefix_overrides =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let has_access = has_access.ok_or_else(|| M::Error::missing_field("has_access"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let integration_display_name = integration_display_name
                    .ok_or_else(|| M::Error::missing_field("integration_display_name"))?;
                let json_schema =
                    json_schema.ok_or_else(|| M::Error::missing_field("json_schema"))?;
                let model_display_name = model_display_name
                    .ok_or_else(|| M::Error::missing_field("model_display_name"))?;
                let model_id = model_id.ok_or_else(|| M::Error::missing_field("model_id"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;
                let provider_display_name = provider_display_name
                    .ok_or_else(|| M::Error::missing_field("provider_display_name"))?;

                let content = LLMObsIntegrationModel {
                    has_access,
                    id,
                    integration,
                    integration_display_name,
                    json_schema,
                    model_display_name,
                    model_id,
                    provider,
                    provider_display_name,
                    region_prefix_overrides,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsIntegrationModelVisitor)
    }
}
