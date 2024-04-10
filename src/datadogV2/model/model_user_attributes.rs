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
pub struct UserAttributes {
    /// Creation time of the user.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether the user is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// Email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// URL of the user's icon.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// Time that the user was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// Whether the user is a service account.
    #[serde(rename = "service_account")]
    pub service_account: Option<bool>,
    /// Status of the user.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Title of the user.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option")]
    pub title: Option<Option<String>>,
    /// Whether the user is verified.
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserAttributes {
    pub fn new() -> UserAttributes {
        UserAttributes {
            created_at: None,
            disabled: None,
            email: None,
            handle: None,
            icon: None,
            modified_at: None,
            name: None,
            service_account: None,
            status: None,
            title: None,
            verified: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
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

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn service_account(mut self, value: bool) -> Self {
        self.service_account = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: Option<String>) -> Self {
        self.title = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }
}

impl Default for UserAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserAttributesVisitor;
        impl<'a> Visitor<'a> for UserAttributesVisitor {
            type Value = UserAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled: Option<bool> = None;
                let mut email: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<Option<String>> = None;
                let mut service_account: Option<bool> = None;
                let mut status: Option<String> = None;
                let mut title: Option<Option<String>> = None;
                let mut verified: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account" => {
                            if v.is_null() {
                                continue;
                            }
                            service_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "verified" => {
                            if v.is_null() {
                                continue;
                            }
                            verified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserAttributes {
                    created_at,
                    disabled,
                    email,
                    handle,
                    icon,
                    modified_at,
                    name,
                    service_account,
                    status,
                    title,
                    verified,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserAttributesVisitor)
    }
}
