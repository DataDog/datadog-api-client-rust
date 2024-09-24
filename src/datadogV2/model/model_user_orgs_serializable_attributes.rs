// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UserOrgsSerializableAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserOrgsSerializableAttributes {
    /// The `UserOrgsSerializableAttributes` `disabled`.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The `UserOrgsSerializableAttributes` `email`.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The `UserOrgsSerializableAttributes` `name`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `UserOrgsSerializableAttributes` `org_id`.
    #[serde(rename = "org_id")]
    pub org_id: Option<String>,
    /// The `UserOrgsSerializableAttributes` `title`.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The `UserOrgsSerializableAttributes` `verified`.
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserOrgsSerializableAttributes {
    pub fn new() -> UserOrgsSerializableAttributes {
        UserOrgsSerializableAttributes {
            disabled: None,
            email: None,
            name: None,
            org_id: None,
            title: None,
            verified: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn org_id(mut self, value: String) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
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

impl Default for UserOrgsSerializableAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserOrgsSerializableAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserOrgsSerializableAttributesVisitor;
        impl<'a> Visitor<'a> for UserOrgsSerializableAttributesVisitor {
            type Value = UserOrgsSerializableAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disabled: Option<bool> = None;
                let mut email: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<String> = None;
                let mut title: Option<String> = None;
                let mut verified: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "verified" => {
                            if v.is_null() {
                                continue;
                            }
                            verified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UserOrgsSerializableAttributes {
                    disabled,
                    email,
                    name,
                    org_id,
                    title,
                    verified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserOrgsSerializableAttributesVisitor)
    }
}
