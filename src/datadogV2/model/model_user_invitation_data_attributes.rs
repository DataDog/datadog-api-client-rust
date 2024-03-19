// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a user invitation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserInvitationDataAttributes {
    /// Creation time of the user invitation.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Time of invitation expiration.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Type of invitation.
    #[serde(rename = "invite_type")]
    pub invite_type: Option<String>,
    /// UUID of the user invitation.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserInvitationDataAttributes {
    pub fn new() -> UserInvitationDataAttributes {
        UserInvitationDataAttributes {
            created_at: None,
            expires_at: None,
            invite_type: None,
            uuid: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn invite_type(mut self, value: String) -> Self {
        self.invite_type = Some(value);
        self
    }

    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
}

impl Default for UserInvitationDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserInvitationDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserInvitationDataAttributesVisitor;
        impl<'a> Visitor<'a> for UserInvitationDataAttributesVisitor {
            type Value = UserInvitationDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut expires_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut invite_type: Option<String> = None;
                let mut uuid: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            if v.is_null() {
                                continue;
                            }
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invite_type" => {
                            if v.is_null() {
                                continue;
                            }
                            invite_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserInvitationDataAttributes {
                    created_at,
                    expires_at,
                    invite_type,
                    uuid,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserInvitationDataAttributesVisitor)
    }
}
