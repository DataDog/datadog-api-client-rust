// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a permission.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PermissionAttributes {
    /// Creation time of the permission.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Description of the permission.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Displayed name for the permission.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Display type.
    #[serde(rename = "display_type")]
    pub display_type: Option<String>,
    /// Name of the permission group.
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
    /// Name of the permission.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether or not the permission is restricted.
    #[serde(rename = "restricted")]
    pub restricted: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PermissionAttributes {
    pub fn new() -> PermissionAttributes {
        PermissionAttributes {
            created: None,
            description: None,
            display_name: None,
            display_type: None,
            group_name: None,
            name: None,
            restricted: None,
            _unparsed: false,
        }
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(&mut self, value: String) -> &mut Self {
        self.display_name = Some(value);
        self
    }

    pub fn display_type(&mut self, value: String) -> &mut Self {
        self.display_type = Some(value);
        self
    }

    pub fn group_name(&mut self, value: String) -> &mut Self {
        self.group_name = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn restricted(&mut self, value: bool) -> &mut Self {
        self.restricted = Some(value);
        self
    }
}

impl Default for PermissionAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PermissionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionAttributesVisitor;
        impl<'a> Visitor<'a> for PermissionAttributesVisitor {
            type Value = PermissionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<String> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut display_type: Option<String> = None;
                let mut group_name: Option<String> = None;
                let mut name: Option<String> = None;
                let mut restricted: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_name" => {
                            if v.is_null() {
                                continue;
                            }
                            group_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted" => {
                            if v.is_null() {
                                continue;
                            }
                            restricted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PermissionAttributes {
                    created,
                    description,
                    display_name,
                    display_type,
                    group_name,
                    name,
                    restricted,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PermissionAttributesVisitor)
    }
}
