// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the IP allowlist entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IPAllowlistEntryAttributes {
    /// The CIDR block describing the IP range of the entry.
    #[serde(rename = "cidr_block")]
    pub cidr_block: Option<String>,
    /// Creation time of the entry.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Time of last entry modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A note describing the IP allowlist entry.
    #[serde(rename = "note")]
    pub note: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IPAllowlistEntryAttributes {
    pub fn new() -> IPAllowlistEntryAttributes {
        IPAllowlistEntryAttributes {
            cidr_block: None,
            created_at: None,
            modified_at: None,
            note: None,
            _unparsed: false,
        }
    }

    pub fn cidr_block(mut self, value: String) -> Self {
        self.cidr_block = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn note(mut self, value: String) -> Self {
        self.note = Some(value);
        self
    }
}

impl Default for IPAllowlistEntryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IPAllowlistEntryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IPAllowlistEntryAttributesVisitor;
        impl<'a> Visitor<'a> for IPAllowlistEntryAttributesVisitor {
            type Value = IPAllowlistEntryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cidr_block: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut note: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cidr_block" => {
                            if v.is_null() {
                                continue;
                            }
                            cidr_block = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "note" => {
                            if v.is_null() {
                                continue;
                            }
                            note = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IPAllowlistEntryAttributes {
                    cidr_block,
                    created_at,
                    modified_at,
                    note,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IPAllowlistEntryAttributesVisitor)
    }
}
