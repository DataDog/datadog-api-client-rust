// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Google Chat app named space attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatAppNamedSpaceResponseAttributes {
    /// Google space display name.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Organization binding ID.
    #[serde(rename = "organization_binding_id")]
    pub organization_binding_id: Option<String>,
    /// Google space resource name.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// Google space URI.
    #[serde(rename = "space_uri")]
    pub space_uri: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatAppNamedSpaceResponseAttributes {
    pub fn new() -> GoogleChatAppNamedSpaceResponseAttributes {
        GoogleChatAppNamedSpaceResponseAttributes {
            display_name: None,
            organization_binding_id: None,
            resource_name: None,
            space_uri: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn organization_binding_id(mut self, value: String) -> Self {
        self.organization_binding_id = Some(value);
        self
    }

    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }

    pub fn space_uri(mut self, value: String) -> Self {
        self.space_uri = Some(value);
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

impl Default for GoogleChatAppNamedSpaceResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GoogleChatAppNamedSpaceResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatAppNamedSpaceResponseAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatAppNamedSpaceResponseAttributesVisitor {
            type Value = GoogleChatAppNamedSpaceResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut organization_binding_id: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut space_uri: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "organization_binding_id" => {
                            if v.is_null() {
                                continue;
                            }
                            organization_binding_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            space_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GoogleChatAppNamedSpaceResponseAttributes {
                    display_name,
                    organization_binding_id,
                    resource_name,
                    space_uri,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatAppNamedSpaceResponseAttributesVisitor)
    }
}
