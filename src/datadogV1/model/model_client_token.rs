// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Client token object. Client tokens (also known as public API keys) enable you to submit
/// data from your browser or mobile applications to Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClientToken {
    /// Creation timestamp of the client token.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Handle of the user who created the client token.
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: Option<String>,
    /// UUID of the user who created the client token.
    #[serde(rename = "created_by_uuid")]
    pub created_by_uuid: Option<uuid::Uuid>,
    /// Timestamp when the client token was disabled.
    #[serde(
        rename = "disabled_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub disabled_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// ID of the user who disabled the client token.
    #[serde(rename = "disabled_by")]
    pub disabled_by: Option<i64>,
    /// Handle of the user who disabled the client token.
    #[serde(rename = "disabled_by_handle")]
    pub disabled_by_handle: Option<String>,
    /// The hash value of the client token. This is the secret token value that should be
    /// used in your browser or mobile application.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// ID of the user who last modified the client token.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<i64>,
    /// Name of the client token.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Organization ID associated with the client token.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// List of allowed origin URLs for browser-based applications. Requests from URLs
    /// not in this list will be rejected.
    #[serde(rename = "origin_urls")]
    pub origin_urls: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ClientToken {
    pub fn new(origin_urls: Vec<String>) -> ClientToken {
        ClientToken {
            created_at: None,
            created_by_handle: None,
            created_by_uuid: None,
            disabled_at: None,
            disabled_by: None,
            disabled_by_handle: None,
            hash: None,
            modified_by: None,
            name: None,
            org_id: None,
            origin_urls,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_handle(mut self, value: String) -> Self {
        self.created_by_handle = Some(value);
        self
    }

    pub fn created_by_uuid(mut self, value: uuid::Uuid) -> Self {
        self.created_by_uuid = Some(value);
        self
    }

    pub fn disabled_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.disabled_at = Some(value);
        self
    }

    pub fn disabled_by(mut self, value: i64) -> Self {
        self.disabled_by = Some(value);
        self
    }

    pub fn disabled_by_handle(mut self, value: String) -> Self {
        self.disabled_by_handle = Some(value);
        self
    }

    pub fn hash(mut self, value: String) -> Self {
        self.hash = Some(value);
        self
    }

    pub fn modified_by(mut self, value: i64) -> Self {
        self.modified_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
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

impl<'de> Deserialize<'de> for ClientToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClientTokenVisitor;
        impl<'a> Visitor<'a> for ClientTokenVisitor {
            type Value = ClientToken;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by_handle: Option<String> = None;
                let mut created_by_uuid: Option<uuid::Uuid> = None;
                let mut disabled_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut disabled_by: Option<i64> = None;
                let mut disabled_by_handle: Option<String> = None;
                let mut hash: Option<String> = None;
                let mut modified_by: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut origin_urls: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_at" => {
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hash" => {
                            if v.is_null() {
                                continue;
                            }
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "origin_urls" => {
                            origin_urls =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let origin_urls =
                    origin_urls.ok_or_else(|| M::Error::missing_field("origin_urls"))?;

                let content = ClientToken {
                    created_at,
                    created_by_handle,
                    created_by_uuid,
                    disabled_at,
                    disabled_by,
                    disabled_by_handle,
                    hash,
                    modified_by,
                    name,
                    org_id,
                    origin_urls,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ClientTokenVisitor)
    }
}
