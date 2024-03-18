// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the created user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserCreateAttributes {
    /// The email of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The title of the user.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserCreateAttributes {
    pub fn new(email: String) -> UserCreateAttributes {
        UserCreateAttributes {
            email,
            name: None,
            title: None,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for UserCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserCreateAttributesVisitor;
        impl<'a> Visitor<'a> for UserCreateAttributesVisitor {
            type Value = UserCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut name: Option<String> = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;

                let content = UserCreateAttributes {
                    email,
                    name,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserCreateAttributesVisitor)
    }
}
