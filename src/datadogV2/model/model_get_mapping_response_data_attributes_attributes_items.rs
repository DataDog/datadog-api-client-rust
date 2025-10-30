// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetMappingResponseDataAttributesAttributesItems {
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "is_custom")]
    pub is_custom: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetMappingResponseDataAttributesAttributesItems {
    pub fn new() -> GetMappingResponseDataAttributesAttributesItems {
        GetMappingResponseDataAttributesAttributesItems {
            attribute: None,
            description: None,
            display_name: None,
            groups: None,
            is_custom: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attribute(mut self, value: String) -> Self {
        self.attribute = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<String>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn is_custom(mut self, value: bool) -> Self {
        self.is_custom = Some(value);
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

impl Default for GetMappingResponseDataAttributesAttributesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetMappingResponseDataAttributesAttributesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMappingResponseDataAttributesAttributesItemsVisitor;
        impl<'a> Visitor<'a> for GetMappingResponseDataAttributesAttributesItemsVisitor {
            type Value = GetMappingResponseDataAttributesAttributesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut is_custom: Option<bool> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_custom" => {
                            if v.is_null() {
                                continue;
                            }
                            is_custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = GetMappingResponseDataAttributesAttributesItems {
                    attribute,
                    description,
                    display_name,
                    groups,
                    is_custom,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetMappingResponseDataAttributesAttributesItemsVisitor)
    }
}
