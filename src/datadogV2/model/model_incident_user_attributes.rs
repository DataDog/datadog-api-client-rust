// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of user object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserAttributes {
    /// Email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// URL of the user's icon.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// UUID of the user.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserAttributes {
    pub fn new() -> IncidentUserAttributes {
        IncidentUserAttributes {
            email: None,
            handle: None,
            icon: None,
            name: None,
            uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn icon(mut self, value: String) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
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

impl Default for IncidentUserAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentUserAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentUserAttributesVisitor {
            type Value = IncidentUserAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut name: Option<Option<String>> = None;
                let mut uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon" => {
                            if v.is_null() {
                                continue;
                            }
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentUserAttributes {
                    email,
                    handle,
                    icon,
                    name,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserAttributesVisitor)
    }
}