// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to update an OpenAI LLM Evaluations integration account. Every field is optional. Only the provided fields are changed. When `dataflows` is provided, only the dataflow IDs in the request are modified. Dataflows omitted from the map keep their current configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
    /// Authentication for updating the OpenAI LLM Evaluations integration account. Exactly one method is set.
    #[serde(rename = "authentication")]
    pub authentication:
        Option<crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate>,
    /// Data Datadog collects from OpenAI LLM Evaluations, keyed by dataflow ID. Each dataflow turns on a distinct type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<crate::datadogV2::model::OpenAILLMEvaluationsIntegrationDataflowsRequest>,
    /// Human-readable name of the OpenAI LLM Evaluations integration account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Settings for updating the OpenAI LLM Evaluations integration account. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings:
        Option<crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountSettingsUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
    pub fn new() -> OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
        OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
            authentication: None,
            dataflows: None,
            name: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn dataflows(
        mut self,
        value: crate::datadogV2::model::OpenAILLMEvaluationsIntegrationDataflowsRequest,
    ) -> Self {
        self.dataflows = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountSettingsUpdate,
    ) -> Self {
        self.settings = Some(value);
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

impl Default for OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAILLMEvaluationsIntegrationAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for OpenAILLMEvaluationsIntegrationAccountUpdateAttributesVisitor {
            type Value = OpenAILLMEvaluationsIntegrationAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate> = None;
                let mut dataflows: Option<
                    crate::datadogV2::model::OpenAILLMEvaluationsIntegrationDataflowsRequest,
                > = None;
                let mut name: Option<String> = None;
                let mut settings: Option<
                    crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountSettingsUpdate,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::OpenAILLMEvaluationsIntegrationAccountAuthenticationUpdate::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OpenAILLMEvaluationsIntegrationAccountUpdateAttributes {
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

        deserializer.deserialize_any(OpenAILLMEvaluationsIntegrationAccountUpdateAttributesVisitor)
    }
}
