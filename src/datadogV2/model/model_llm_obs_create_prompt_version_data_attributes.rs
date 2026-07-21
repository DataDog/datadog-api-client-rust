// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new version of an LLM Observability prompt. `template` is required; all other attributes are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCreatePromptVersionDataAttributes {
    /// Optional description of this version.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Optional feature-flag environment UUIDs the service attempts to enable and configure to use this version as their default after creation.
    #[serde(rename = "env_ids")]
    pub env_ids: Option<Vec<String>>,
    /// Optional labels to attach to this version. Do not use this attribute for new integrations.
    #[deprecated]
    #[serde(rename = "labels")]
    pub labels: Option<Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>>,
    /// A text template or a list of chat messages.
    #[serde(rename = "template")]
    pub template: crate::datadogV2::model::LLMObsPromptTemplate,
    /// Optional user-supplied version identifier for this version.
    #[serde(rename = "user_version")]
    pub user_version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCreatePromptVersionDataAttributes {
    pub fn new(
        template: crate::datadogV2::model::LLMObsPromptTemplate,
    ) -> LLMObsCreatePromptVersionDataAttributes {
        #[allow(deprecated)]
        LLMObsCreatePromptVersionDataAttributes {
            description: None,
            env_ids: None,
            labels: None,
            template,
            user_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn env_ids(mut self, value: Vec<String>) -> Self {
        self.env_ids = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn labels(mut self, value: Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>) -> Self {
        self.labels = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn user_version(mut self, value: String) -> Self {
        self.user_version = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCreatePromptVersionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCreatePromptVersionDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsCreatePromptVersionDataAttributesVisitor {
            type Value = LLMObsCreatePromptVersionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut env_ids: Option<Vec<String>> = None;
                let mut labels: Option<Vec<crate::datadogV2::model::LLMObsPromptVersionLabel>> =
                    None;
                let mut template: Option<crate::datadogV2::model::LLMObsPromptTemplate> = None;
                let mut user_version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            env_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _template) = template {
                                match _template {
                                    crate::datadogV2::model::LLMObsPromptTemplate::UnparsedObject(_template) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "user_version" => {
                            if v.is_null() {
                                continue;
                            }
                            user_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let template = template.ok_or_else(|| M::Error::missing_field("template"))?;

                #[allow(deprecated)]
                let content = LLMObsCreatePromptVersionDataAttributes {
                    description,
                    env_ids,
                    labels,
                    template,
                    user_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCreatePromptVersionDataAttributesVisitor)
    }
}
