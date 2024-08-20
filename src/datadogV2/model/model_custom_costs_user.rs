// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata of the user that has uploaded the Custom Costs file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostsUser {
    /// The name of the Custom Costs file.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The name of the Custom Costs file.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostsUser {
    pub fn new() -> CustomCostsUser {
        CustomCostsUser {
            email: None,
            icon: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn icon(mut self, value: String) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for CustomCostsUser {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostsUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostsUserVisitor;
        impl<'a> Visitor<'a> for CustomCostsUserVisitor {
            type Value = CustomCostsUser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut name: Option<String> = None;
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
                        "icon" => {
                            if v.is_null() {
                                continue;
                            }
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomCostsUser {
                    email,
                    icon,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostsUserVisitor)
    }
}
