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
pub struct EntityResponseDataAttributes {
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "kind")]
    pub kind: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    #[serde(rename = "properties")]
    pub properties: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityResponseDataAttributes {
    pub fn new() -> EntityResponseDataAttributes {
        EntityResponseDataAttributes {
            api_version: None,
            description: None,
            display_name: None,
            kind: None,
            name: None,
            namespace: None,
            owner: None,
            properties: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_version(mut self, value: String) -> Self {
        self.api_version = Some(value);
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

    pub fn kind(mut self, value: String) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn owner(mut self, value: String) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for EntityResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for EntityResponseDataAttributesVisitor {
            type Value = EntityResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_version: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut kind: Option<String> = None;
                let mut name: Option<String> = None;
                let mut namespace: Option<String> = None;
                let mut owner: Option<String> = None;
                let mut properties: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apiVersion" => {
                            if v.is_null() {
                                continue;
                            }
                            api_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "displayName" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            if v.is_null() {
                                continue;
                            }
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner" => {
                            if v.is_null() {
                                continue;
                            }
                            owner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EntityResponseDataAttributes {
                    api_version,
                    description,
                    display_name,
                    kind,
                    name,
                    namespace,
                    owner,
                    properties,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityResponseDataAttributesVisitor)
    }
}
