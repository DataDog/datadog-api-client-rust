// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RUM application attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMApplicationAttributes {
    /// ID of the RUM application.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Client token of the RUM application.
    #[serde(rename = "client_token")]
    pub client_token: String,
    /// Timestamp in ms of the creation date.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Handle of the creator user.
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: String,
    /// Hash of the RUM application. Optional.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// Indicates if the RUM application is active.
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: String,
    /// Org ID of the RUM application.
    #[serde(rename = "org_id")]
    pub org_id: i32,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Timestamp in ms of the last update date.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    /// Handle of the updater user.
    #[serde(rename = "updated_by_handle")]
    pub updated_by_handle: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMApplicationAttributes {
    pub fn new(
        application_id: String,
        client_token: String,
        created_at: i64,
        created_by_handle: String,
        name: String,
        org_id: i32,
        type_: String,
        updated_at: i64,
        updated_by_handle: String,
    ) -> RUMApplicationAttributes {
        RUMApplicationAttributes {
            application_id,
            client_token,
            created_at,
            created_by_handle,
            hash: None,
            is_active: None,
            name,
            org_id,
            type_,
            updated_at,
            updated_by_handle,
            _unparsed: false,
        }
    }

    pub fn hash(&mut self, value: String) -> &mut Self {
        self.hash = Some(value);
        self
    }

    pub fn is_active(&mut self, value: bool) -> &mut Self {
        self.is_active = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RUMApplicationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMApplicationAttributesVisitor;
        impl<'a> Visitor<'a> for RUMApplicationAttributesVisitor {
            type Value = RUMApplicationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut client_token: Option<String> = None;
                let mut created_at: Option<i64> = None;
                let mut created_by_handle: Option<String> = None;
                let mut hash: Option<String> = None;
                let mut is_active: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i32> = None;
                let mut type_: Option<String> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by_handle: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_token" => {
                            client_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_handle" => {
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hash" => {
                            if v.is_null() {
                                continue;
                            }
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            if v.is_null() {
                                continue;
                            }
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by_handle" => {
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let client_token =
                    client_token.ok_or_else(|| M::Error::missing_field("client_token"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by_handle = created_by_handle
                    .ok_or_else(|| M::Error::missing_field("created_by_handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by_handle = updated_by_handle
                    .ok_or_else(|| M::Error::missing_field("updated_by_handle"))?;

                let content = RUMApplicationAttributes {
                    application_id,
                    client_token,
                    created_at,
                    created_by_handle,
                    hash,
                    is_active,
                    name,
                    org_id,
                    type_,
                    updated_at,
                    updated_by_handle,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMApplicationAttributesVisitor)
    }
}
