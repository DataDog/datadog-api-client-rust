// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A template variable that viewers can modify on the secure embed shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecureEmbedSelectableTemplateVariable {
    /// Default selected values for the variable.
    #[serde(rename = "default_values")]
    pub default_values: Option<Vec<String>>,
    /// Name of the template variable. Usually matches the prefix unless you want a different display name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Tag prefix for the variable (e.g., `environment`, `service`).
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// Restrict which tag values are visible to the viewer.
    #[serde(rename = "visible_tags")]
    pub visible_tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecureEmbedSelectableTemplateVariable {
    pub fn new() -> SecureEmbedSelectableTemplateVariable {
        SecureEmbedSelectableTemplateVariable {
            default_values: None,
            name: None,
            prefix: None,
            visible_tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_values(mut self, value: Vec<String>) -> Self {
        self.default_values = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn prefix(mut self, value: String) -> Self {
        self.prefix = Some(value);
        self
    }

    pub fn visible_tags(mut self, value: Vec<String>) -> Self {
        self.visible_tags = Some(value);
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

impl Default for SecureEmbedSelectableTemplateVariable {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecureEmbedSelectableTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecureEmbedSelectableTemplateVariableVisitor;
        impl<'a> Visitor<'a> for SecureEmbedSelectableTemplateVariableVisitor {
            type Value = SecureEmbedSelectableTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_values: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut visible_tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default_values" => {
                            if v.is_null() {
                                continue;
                            }
                            default_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visible_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            visible_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecureEmbedSelectableTemplateVariable {
                    default_values,
                    name,
                    prefix,
                    visible_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecureEmbedSelectableTemplateVariableVisitor)
    }
}
