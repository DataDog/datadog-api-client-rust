// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Organization handle attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatOrganizationHandleResponseAttributes {
    /// Organization handle name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Google space display name.
    #[serde(rename = "space_display_name")]
    pub space_display_name: Option<String>,
    /// Google space resource name.
    #[serde(rename = "space_resource_name")]
    pub space_resource_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatOrganizationHandleResponseAttributes {
    pub fn new() -> GoogleChatOrganizationHandleResponseAttributes {
        GoogleChatOrganizationHandleResponseAttributes {
            name: None,
            space_display_name: None,
            space_resource_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn space_display_name(mut self, value: String) -> Self {
        self.space_display_name = Some(value);
        self
    }

    pub fn space_resource_name(mut self, value: String) -> Self {
        self.space_resource_name = Some(value);
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

impl Default for GoogleChatOrganizationHandleResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GoogleChatOrganizationHandleResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatOrganizationHandleResponseAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatOrganizationHandleResponseAttributesVisitor {
            type Value = GoogleChatOrganizationHandleResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut space_display_name: Option<String> = None;
                let mut space_resource_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            space_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            space_resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GoogleChatOrganizationHandleResponseAttributes {
                    name,
                    space_display_name,
                    space_resource_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatOrganizationHandleResponseAttributesVisitor)
    }
}
