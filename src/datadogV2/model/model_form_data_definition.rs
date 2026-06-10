// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A JSON Schema definition that describes the form's data fields.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormDataDefinition {
    /// A description shown to form respondents.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A map of field names to their JSON Schema definitions.
    #[serde(rename = "properties")]
    pub properties: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// List of field names that must be answered.
    #[serde(rename = "required")]
    pub required: Option<Vec<String>>,
    /// The title of the form schema.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The root schema type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::FormDataDefinitionType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormDataDefinition {
    pub fn new() -> FormDataDefinition {
        FormDataDefinition {
            description: None,
            properties: None,
            required: None,
            title: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::FormDataDefinitionType) -> Self {
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

impl Default for FormDataDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FormDataDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormDataDefinitionVisitor;
        impl<'a> Visitor<'a> for FormDataDefinitionVisitor {
            type Value = FormDataDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut properties: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut required: Option<Vec<String>> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::FormDataDefinitionType> = None;
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
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            if v.is_null() {
                                continue;
                            }
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::FormDataDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FormDataDefinition {
                    description,
                    properties,
                    required,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormDataDefinitionVisitor)
    }
}
