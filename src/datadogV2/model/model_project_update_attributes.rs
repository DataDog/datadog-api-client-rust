// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project update attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectUpdateAttributes {
    /// Project columns configuration
    #[serde(rename = "columns_config")]
    pub columns_config: Option<crate::datadogV2::model::ProjectColumnsConfig>,
    /// List of enabled custom case type IDs
    #[serde(rename = "enabled_custom_case_types")]
    pub enabled_custom_case_types: Option<Vec<String>>,
    /// Project name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Project settings
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::ProjectSettings>,
    /// Team UUID to associate with the project
    #[serde(rename = "team_uuid")]
    pub team_uuid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectUpdateAttributes {
    pub fn new() -> ProjectUpdateAttributes {
        ProjectUpdateAttributes {
            columns_config: None,
            enabled_custom_case_types: None,
            name: None,
            settings: None,
            team_uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn columns_config(mut self, value: crate::datadogV2::model::ProjectColumnsConfig) -> Self {
        self.columns_config = Some(value);
        self
    }

    pub fn enabled_custom_case_types(mut self, value: Vec<String>) -> Self {
        self.enabled_custom_case_types = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn settings(mut self, value: crate::datadogV2::model::ProjectSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn team_uuid(mut self, value: String) -> Self {
        self.team_uuid = Some(value);
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

impl Default for ProjectUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ProjectUpdateAttributesVisitor {
            type Value = ProjectUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns_config: Option<crate::datadogV2::model::ProjectColumnsConfig> =
                    None;
                let mut enabled_custom_case_types: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut settings: Option<crate::datadogV2::model::ProjectSettings> = None;
                let mut team_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns_config" => {
                            if v.is_null() {
                                continue;
                            }
                            columns_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled_custom_case_types" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_custom_case_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "team_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            team_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProjectUpdateAttributes {
                    columns_config,
                    enabled_custom_case_types,
                    name,
                    settings,
                    team_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectUpdateAttributesVisitor)
    }
}
