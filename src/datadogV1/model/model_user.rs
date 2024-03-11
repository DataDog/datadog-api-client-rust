// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create, edit, and disable users.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct User {
    /// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
    #[serde(
        rename = "access_role",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub access_role: Option<Option<crate::datadogV1::model::AccessRole>>,
    /// The new disabled status of the user.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The new email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The user handle, must be a valid email.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether or not the user logged in Datadog at least once.
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl User {
    pub fn new() -> User {
        User {
            access_role: None,
            disabled: None,
            email: None,
            handle: None,
            icon: None,
            name: None,
            verified: None,
            _unparsed: false,
        }
    }

    pub fn access_role(&mut self, value: Option<crate::datadogV1::model::AccessRole>) -> &mut Self {
        self.access_role = Some(value);
        self
    }

    pub fn disabled(&mut self, value: bool) -> &mut Self {
        self.disabled = Some(value);
        self
    }

    pub fn email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn icon(&mut self, value: String) -> &mut Self {
        self.icon = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn verified(&mut self, value: bool) -> &mut Self {
        self.verified = Some(value);
        self
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserVisitor;
        impl<'a> Visitor<'a> for UserVisitor {
            type Value = User;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_role: Option<Option<crate::datadogV1::model::AccessRole>> = None;
                let mut disabled: Option<bool> = None;
                let mut email: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut name: Option<String> = None;
                let mut verified: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_role" => {
                            access_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _access_role) = access_role {
                                match _access_role {
                                    Some(crate::datadogV1::model::AccessRole::UnparsedObject(
                                        _access_role,
                                    )) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = User {
                    access_role,
                    disabled,
                    email,
                    handle,
                    icon,
                    name,
                    verified,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserVisitor)
    }
}
