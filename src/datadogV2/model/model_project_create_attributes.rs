// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project creation attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectCreateAttributes {
    /// List of enabled custom case type IDs.
    #[serde(rename = "enabled_custom_case_types")]
    pub enabled_custom_case_types: Option<Vec<String>>,
    /// Project's key. Cannot be "CASE".
    #[serde(rename = "key")]
    pub key: String,
    /// Project name.
    #[serde(rename = "name")]
    pub name: String,
    /// Team UUID to associate with the project.
    #[serde(rename = "team_uuid")]
    pub team_uuid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectCreateAttributes {
    pub fn new(key: String, name: String) -> ProjectCreateAttributes {
        ProjectCreateAttributes {
            enabled_custom_case_types: None,
            key,
            name,
            team_uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled_custom_case_types(mut self, value: Vec<String>) -> Self {
        self.enabled_custom_case_types = Some(value);
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

impl<'de> Deserialize<'de> for ProjectCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectCreateAttributesVisitor;
        impl<'a> Visitor<'a> for ProjectCreateAttributesVisitor {
            type Value = ProjectCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled_custom_case_types: Option<Vec<String>> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut team_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled_custom_case_types" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_custom_case_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ProjectCreateAttributes {
                    enabled_custom_case_types,
                    key,
                    name,
                    team_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectCreateAttributesVisitor)
    }
}
