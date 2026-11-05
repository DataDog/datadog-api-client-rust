// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Provider-specific integration configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayProviderIntegrationConfig {
    /// AWS region used for Bedrock model discovery, routing, and request signing.
    #[serde(rename = "bedrock_region")]
    pub bedrock_region: Option<String>,
    /// Azure AI Foundry project name, used to call the project's agents.
    #[serde(rename = "foundry_project")]
    pub foundry_project: Option<String>,
    /// ARM resource ID of the Azure AI Foundry resource, such as `/subscriptions/<id>/resourceGroups/<group>/providers/Microsoft.CognitiveServices/accounts/<name>`. Required for `azure_foundry` providers.
    #[serde(rename = "foundry_resource_id")]
    pub foundry_resource_id: Option<String>,
    /// GCP location used for Gemini model discovery and credential resolution.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// GCP project ID used for Gemini model discovery and credential resolution.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    /// Custom provider default for discovered and new manual models. Omit to use Chat Completions. An empty list is invalid.
    #[serde(rename = "supported_operations")]
    pub supported_operations:
        Option<Vec<crate::datadogV2::model::AiGatewayCustomProviderSupportedOperation>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayProviderIntegrationConfig {
    pub fn new() -> AiGatewayProviderIntegrationConfig {
        AiGatewayProviderIntegrationConfig {
            bedrock_region: None,
            foundry_project: None,
            foundry_resource_id: None,
            location: None,
            project_id: None,
            supported_operations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bedrock_region(mut self, value: String) -> Self {
        self.bedrock_region = Some(value);
        self
    }

    pub fn foundry_project(mut self, value: String) -> Self {
        self.foundry_project = Some(value);
        self
    }

    pub fn foundry_resource_id(mut self, value: String) -> Self {
        self.foundry_resource_id = Some(value);
        self
    }

    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }

    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }

    pub fn supported_operations(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayCustomProviderSupportedOperation>,
    ) -> Self {
        self.supported_operations = Some(value);
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

impl Default for AiGatewayProviderIntegrationConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayProviderIntegrationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayProviderIntegrationConfigVisitor;
        impl<'a> Visitor<'a> for AiGatewayProviderIntegrationConfigVisitor {
            type Value = AiGatewayProviderIntegrationConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bedrock_region: Option<String> = None;
                let mut foundry_project: Option<String> = None;
                let mut foundry_resource_id: Option<String> = None;
                let mut location: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut supported_operations: Option<
                    Vec<crate::datadogV2::model::AiGatewayCustomProviderSupportedOperation>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bedrock_region" => {
                            if v.is_null() {
                                continue;
                            }
                            bedrock_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "foundry_project" => {
                            if v.is_null() {
                                continue;
                            }
                            foundry_project =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "foundry_resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            foundry_resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supported_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            supported_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayProviderIntegrationConfig {
                    bedrock_region,
                    foundry_project,
                    foundry_resource_id,
                    location,
                    project_id,
                    supported_operations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderIntegrationConfigVisitor)
    }
}
