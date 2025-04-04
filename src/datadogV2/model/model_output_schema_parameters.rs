// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `OutputSchemaParameters` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutputSchemaParameters {
    /// The `OutputSchemaParameters` `defaultValue`.
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
    /// The `OutputSchemaParameters` `description`.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The `OutputSchemaParameters` `label`.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// The `OutputSchemaParameters` `name`.
    #[serde(rename = "name")]
    pub name: String,
    /// The definition of `OutputSchemaParametersType` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OutputSchemaParametersType,
    /// The `OutputSchemaParameters` `value`.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutputSchemaParameters {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::OutputSchemaParametersType,
    ) -> OutputSchemaParameters {
        OutputSchemaParameters {
            default_value: None,
            description: None,
            label: None,
            name,
            type_,
            value: None,
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

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
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

impl<'de> Deserialize<'de> for OutputSchemaParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutputSchemaParametersVisitor;
        impl<'a> Visitor<'a> for OutputSchemaParametersVisitor {
            type Value = OutputSchemaParameters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_value: Option<serde_json::Value> = None;
                let mut description: Option<String> = None;
                let mut label: Option<String> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::OutputSchemaParametersType> = None;
                let mut value: Option<serde_json::Value> = None;
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
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::OutputSchemaParametersType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = OutputSchemaParameters {
                    default_value,
                    description,
                    label,
                    name,
                    type_,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutputSchemaParametersVisitor)
    }
}
