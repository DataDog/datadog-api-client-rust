// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `InputSchemaDataAttributesParametersItemsDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputSchemaDataAttributesParametersItemsDataAttributes {
    /// The `attributes` `defaultValue`.
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
    /// The `attributes` `description`.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The `attributes` `enum`.
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    /// The `attributes` `label`.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// The `attributes` `name`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `attributes` `type`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl InputSchemaDataAttributesParametersItemsDataAttributes {
    pub fn new() -> InputSchemaDataAttributesParametersItemsDataAttributes {
        InputSchemaDataAttributesParametersItemsDataAttributes {
            default_value: None,
            description: None,
            enum_: None,
            label: None,
            name: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_value(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enum_(mut self, value: Vec<String>) -> Self {
        self.enum_ = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for InputSchemaDataAttributesParametersItemsDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for InputSchemaDataAttributesParametersItemsDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InputSchemaDataAttributesParametersItemsDataAttributesVisitor;
        impl<'a> Visitor<'a> for InputSchemaDataAttributesParametersItemsDataAttributesVisitor {
            type Value = InputSchemaDataAttributesParametersItemsDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_value: Option<serde_json::Value> = None;
                let mut description: Option<String> = None;
                let mut enum_: Option<Vec<String>> = None;
                let mut label: Option<String> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "defaultValue" => {
                            if v.is_null() {
                                continue;
                            }
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enum" => {
                            if v.is_null() {
                                continue;
                            }
                            enum_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = InputSchemaDataAttributesParametersItemsDataAttributes {
                    default_value,
                    description,
                    enum_,
                    label,
                    name,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(InputSchemaDataAttributesParametersItemsDataAttributesVisitor)
    }
}
