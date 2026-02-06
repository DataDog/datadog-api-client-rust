// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an included user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserIncludedAttributes {
    /// The email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// The handle of the user.
    #[serde(rename = "handle")]
    pub handle: String,
    /// The icon URL for the user.
    #[serde(rename = "icon")]
    pub icon: String,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: String,
    /// The UUID of the user.
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserIncludedAttributes {
    pub fn new(
        email: String,
        handle: String,
        icon: String,
        name: String,
        uuid: uuid::Uuid,
    ) -> UserIncludedAttributes {
        UserIncludedAttributes {
            email,
            handle,
            icon,
            name,
            uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for UserIncludedAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserIncludedAttributesVisitor;
        impl<'a> Visitor<'a> for UserIncludedAttributesVisitor {
            type Value = UserIncludedAttributes;

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
                let mut name: Option<String> = None;
                let mut uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon" => {
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let icon = icon.ok_or_else(|| M::Error::missing_field("icon"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = UserIncludedAttributes {
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

        deserializer.deserialize_any(UserIncludedAttributesVisitor)
    }
}
