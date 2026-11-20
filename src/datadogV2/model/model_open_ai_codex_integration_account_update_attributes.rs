// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Writable attributes used to update an OpenAI Codex integration account. Every field is optional. Only the provided fields are changed. When `dataflows` is provided, only the dataflow IDs in the request are modified. Dataflows omitted from the map keep their current configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAICodexIntegrationAccountUpdateAttributes {
    /// Authentication for updating the OpenAI Codex integration account. Exactly one method is set.
    #[serde(rename = "authentication")]
    pub authentication: Option<crate::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationUpdate>,
    /// Data Datadog collects from OpenAI Codex, keyed by dataflow ID. Datadog always collects daily Codex analytics metrics. Each dataflow in this object adds another type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values. Every dataflow requires an OpenAI enterprise subscription with Codex enabled. If you enable a dataflow without this subscription, Datadog stores the setting but collects no data.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<crate::datadogV2::model::OpenAICodexIntegrationDataflowsRequest>,
    /// Human-readable name of the OpenAI Codex integration account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Settings for updating the OpenAI Codex integration account. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::OpenAICodexIntegrationAccountSettingsUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl OpenAICodexIntegrationAccountUpdateAttributes {
    pub fn new() -> OpenAICodexIntegrationAccountUpdateAttributes {
        OpenAICodexIntegrationAccountUpdateAttributes {
            authentication: None,
            dataflows: None,
            name: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn authentication(mut self, value: crate::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationUpdate) -> Self {
        self.authentication = Some(value);
        self
    }
    
    pub fn dataflows(mut self, value: crate::datadogV2::model::OpenAICodexIntegrationDataflowsRequest) -> Self {
        self.dataflows = Some(value);
        self
    }
    
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    
    pub fn settings(mut self, value: crate::datadogV2::model::OpenAICodexIntegrationAccountSettingsUpdate) -> Self {
        self.settings = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for OpenAICodexIntegrationAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAICodexIntegrationAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for OpenAICodexIntegrationAccountUpdateAttributesVisitor {
            type Value = OpenAICodexIntegrationAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<crate::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationUpdate> = None;
                let mut dataflows: Option<crate::datadogV2::model::OpenAICodexIntegrationDataflowsRequest> = None;
                let mut name: Option<String> = None;
                let mut settings: Option<crate::datadogV2::model::OpenAICodexIntegrationAccountSettingsUpdate> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::OpenAICodexIntegrationAccountAuthenticationUpdate::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = OpenAICodexIntegrationAccountUpdateAttributes {
                    authentication,
                    dataflows,
                    name,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAICodexIntegrationAccountUpdateAttributesVisitor)
    }
}
